use crate::core::dbal::query::fragments;
use crate::core::dto::sys_menu_dto::SysMenuListQueryDto;
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::MySqlPool;

use crate::core::{errors::AppError, model::sys_menu::SysMenuModel};

#[async_trait]
pub trait ISysMenuRepository: Interface {
    async fn list(&self, query: SysMenuListQueryDto) -> Result<Vec<SysMenuModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysMenuModel>, AppError>;
    async fn insert(&self, model: &SysMenuModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysMenuModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysMenuRepository)]
pub(crate) struct SysMenuRepository {
    pool: MySqlPool,
}

impl SysMenuRepository {
    pub(crate) async fn list(
        &self,
        query: SysMenuListQueryDto,
    ) -> Result<Vec<SysMenuModel>, AppError> {
        let (name_kw, name_like) = fragments::keyword_args(query.name.as_deref());
        let status = query.status.filter(|s| !s.trim().is_empty());

        sqlx::query_as::<_, SysMenuModel>(
            r#"
            SELECT
                id, parent_id, menu_type, menu_name, route_name, route_path, component_path,
                perms, permission, icon, order_num, is_visible, status
            FROM sys_menu
            WHERE is_deleted = 0
              AND (? = '' OR menu_name LIKE ?)
              AND (? IS NULL OR status = ?)
            ORDER BY parent_id ASC, order_num ASC, id ASC
            "#,
        )
        .bind(&name_kw)
        .bind(&name_like)
        .bind(&status)
        .bind(&status)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询菜单失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysMenuModel>, AppError> {
        sqlx::query_as::<_, SysMenuModel>(
            r#"
            SELECT
                id, parent_id, menu_type, menu_name, route_name, route_path, component_path,
                perms, permission, icon, order_num, is_visible, status
            FROM sys_menu
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询菜单失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysMenuModel) -> Result<u64, AppError> {
        self.ensure_parent_exists(model.parent_id).await?;
        let permission = model.permission.as_deref().or(model.perms.as_deref());
        let result = sqlx::query(
            r#"
            INSERT INTO sys_menu (
                parent_id, menu_type, menu_name, route_name, route_path, component_path, perms, permission,
                icon, order_num, is_visible, status, created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 1, 0)
            "#,
        )
        .bind(model.parent_id)
        .bind(model.menu_type)
        .bind(&model.menu_name)
        .bind(model.route_name.as_deref())
        .bind(model.route_path.as_deref())
        .bind(model.component_path.as_deref())
        .bind(permission)
        .bind(permission)
        .bind(model.icon.as_deref())
        .bind(model.order_num)
        .bind(model.is_visible)
        .bind(model.status)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增菜单失败: {err}")))?;
        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        model: &SysMenuModel,
    ) -> Result<bool, AppError> {
        self.ensure_parent_exists(model.parent_id).await?;
        let permission = model.permission.as_deref().or(model.perms.as_deref());
        let result = sqlx::query(
            r#"
            UPDATE sys_menu
            SET parent_id = ?, menu_type = ?, menu_name = ?, route_name = ?, route_path = ?, component_path = ?,
                perms = ?, permission = ?, icon = ?, order_num = ?, is_visible = ?, status = ?, updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(model.parent_id)
        .bind(model.menu_type)
        .bind(&model.menu_name)
        .bind(model.route_name.as_deref())
        .bind(model.route_path.as_deref())
        .bind(model.component_path.as_deref())
        .bind(permission)
        .bind(permission)
        .bind(model.icon.as_deref())
        .bind(model.order_num)
        .bind(model.is_visible)
        .bind(model.status)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新菜单失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_menu
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除菜单失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn ensure_parent_exists(&self, parent_id: u64) -> Result<(), AppError> {
        if parent_id == 0 {
            return Ok(());
        }

        let exists = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(1)
            FROM sys_menu
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(parent_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询上级菜单失败: {err}")))?;

        if exists == 0 {
            return Err(AppError::bad_request("上级菜单不存在"));
        }
        Ok(())
    }
}

#[async_trait]
impl ISysMenuRepository for SysMenuRepository {
    async fn list(&self, query: SysMenuListQueryDto) -> Result<Vec<SysMenuModel>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysMenuModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysMenuModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, model: &SysMenuModel) -> Result<bool, AppError> {
        self.update_by_id(id, model).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
