use crate::core::dbal::query::fragments;
use sqlx::MySqlPool;

use crate::core::{errors::AppError, model::sys_menu::SysMenuModel};

#[derive(Clone)]
pub(crate) struct SysMenuRepository {
    pool: MySqlPool,
}

impl SysMenuRepository {
    pub(crate) fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysMenuModel>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        sqlx::query_as::<_, SysMenuModel>(
            r#"
            SELECT id, parent_id, menu_name, route_path, component_path, is_visible
            FROM sys_menu
            WHERE is_deleted = 0
              AND (? = '' OR menu_name LIKE ? OR route_path LIKE ?)
            ORDER BY parent_id ASC, order_num ASC, id ASC
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询菜单失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysMenuModel>, AppError> {
        sqlx::query_as::<_, SysMenuModel>(
            r#"
            SELECT id, parent_id, menu_name, route_path, component_path, is_visible
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
        let result = sqlx::query(
            r#"
            INSERT INTO sys_menu (
                parent_id, menu_type, menu_name, route_name, route_path, component_path,
                perms, order_num, is_visible, status, created_by, updated_by, is_deleted
            ) VALUES (?, 2, ?, ?, ?, ?, NULL, 0, ?, 1, 1, 1, 0)
            "#,
        )
        .bind(model.parent_id)
        .bind(&model.menu_name)
        .bind(&model.menu_name)
        .bind(model.route_path.as_deref().unwrap_or_default())
        .bind(model.component_path.as_deref().unwrap_or_default())
        .bind(model.is_visible)
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
        let result = sqlx::query(
            r#"
            UPDATE sys_menu
            SET parent_id = ?, menu_name = ?, route_name = ?, route_path = ?, component_path = ?, is_visible = ?, updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(model.parent_id)
        .bind(&model.menu_name)
        .bind(&model.menu_name)
        .bind(model.route_path.as_deref().unwrap_or_default())
        .bind(model.component_path.as_deref().unwrap_or_default())
        .bind(model.is_visible)
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
