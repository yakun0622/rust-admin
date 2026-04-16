use crate::core::dbal::query::fragments;
use crate::core::dto::sys_role_dto::{SysRoleListQueryDto, SysRoleUpdateReqDto};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::core::{errors::AppError, model::sys_role::SysRoleModel};

#[async_trait]
pub trait ISysRoleRepository: Interface {
    async fn list(&self, query: SysRoleListQueryDto) -> Result<Vec<SysRoleModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysRoleModel>, AppError>;
    async fn insert(&self, model: &SysRoleModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysRoleUpdateReqDto) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
    async fn get_menu_ids_by_role_id(&self, role_id: u64) -> Result<Vec<u64>, AppError>;
    async fn update_role_menus(&self, role_id: u64, menu_ids: Vec<u64>) -> Result<(), AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysRoleRepository)]
pub(crate) struct SysRoleRepository {
    pool: MySqlPool,
}

impl SysRoleRepository {
    pub(crate) async fn list(
        &self,
        query: SysRoleListQueryDto,
    ) -> Result<Vec<SysRoleModel>, AppError> {
        let (name_kw, name_like) = fragments::keyword_args(query.name.as_deref());
        let (key_kw, key_like) = fragments::keyword_args(query.key.as_deref());
        let status = query.status.filter(|s| !s.trim().is_empty());

        sqlx::query_as::<_, SysRoleModel>(
            r#"
            SELECT id, role_name, role_key, role_sort, status
            FROM sys_role
            WHERE is_deleted = 0
              AND (? = '' OR role_name LIKE ?)
              AND (? = '' OR role_key LIKE ?)
              AND (? IS NULL OR status = ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&name_kw)
        .bind(&name_like)
        .bind(&key_kw)
        .bind(&key_like)
        .bind(&status)
        .bind(&status)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询角色失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysRoleModel>, AppError> {
        sqlx::query_as::<_, SysRoleModel>(
            r#"
            SELECT id, role_name, role_key, role_sort, status
            FROM sys_role
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询角色失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysRoleModel) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO sys_role (
                role_name, role_key, role_sort, status, data_scope, created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, ?, 5, 1, 1, 0)
            "#,
        )
        .bind(&model.role_name)
        .bind(&model.role_key)
        .bind(model.role_sort)
        .bind(model.status)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增角色失败: {err}")))?;

        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        dto: SysRoleUpdateReqDto,
    ) -> Result<bool, AppError> {
        let mut builder = QueryBuilder::<MySql>::new("UPDATE sys_role SET ");
        let mut separated = builder.separated(", ");
        let mut has_update = false;

        if let Some(name) = dto.name {
            separated.push("role_name = ").push_bind(name);
            has_update = true;
        }
        if let Some(key) = dto.key {
            separated.push("role_key = ").push_bind(key);
            has_update = true;
        }
        if let Some(sort) = dto.sort {
            separated.push("role_sort = ").push_bind(sort);
            has_update = true;
        }
        if let Some(status) = dto.status {
            let status_value = if matches!(status.as_str(), "disabled" | "0") {
                0_i16
            } else {
                1_i16
            };
            separated.push("status = ").push_bind(status_value);
            has_update = true;
        }

        if !has_update {
            return Err(AppError::bad_request("没有可更新的字段"));
        }

        separated.push("updated_by = 1");
        builder
            .push(" WHERE id = ")
            .push_bind(id)
            .push(" AND is_deleted = 0");

        let result = builder
            .build()
            .execute(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("更新角色失败: {err}")))?;

        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_role
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除角色失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn get_menu_ids_by_role_id(&self, role_id: u64) -> Result<Vec<u64>, AppError> {
        sqlx::query_scalar::<_, u64>(
            r#"
            SELECT menu_id
            FROM sys_role_menu
            WHERE role_id = ?
            "#,
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询角色菜单失败: {err}")))
    }

    pub(crate) async fn update_role_menus(
        &self,
        role_id: u64,
        menu_ids: Vec<u64>,
    ) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| AppError::internal(format!("开始事务失败: {err}")))?;

        sqlx::query("DELETE FROM sys_role_menu WHERE role_id = ?")
            .bind(role_id)
            .execute(&mut *tx)
            .await
            .map_err(|err| AppError::internal(format!("删除旧权限失败: {err}")))?;

        if !menu_ids.is_empty() {
            // Simple bulk insert without QueryBuilder to avoid potential import issues
            for menu_id in menu_ids {
                sqlx::query("INSERT INTO sys_role_menu (role_id, menu_id) VALUES (?, ?)")
                    .bind(role_id)
                    .bind(menu_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|err| AppError::internal(format!("分配权限失败: {err}")))?;
            }
        }

        tx.commit()
            .await
            .map_err(|err| AppError::internal(format!("提交事务失败: {err}")))?;

        Ok(())
    }
}

#[async_trait]
impl ISysRoleRepository for SysRoleRepository {
    async fn list(&self, query: SysRoleListQueryDto) -> Result<Vec<SysRoleModel>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysRoleModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysRoleModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, dto: SysRoleUpdateReqDto) -> Result<bool, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }

    async fn get_menu_ids_by_role_id(&self, role_id: u64) -> Result<Vec<u64>, AppError> {
        self.get_menu_ids_by_role_id(role_id).await
    }

    async fn update_role_menus(&self, role_id: u64, menu_ids: Vec<u64>) -> Result<(), AppError> {
        self.update_role_menus(role_id, menu_ids).await
    }
}
