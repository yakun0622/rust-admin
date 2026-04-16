use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{query_as, MySqlPool};

use crate::core::{
    errors::AppError,
    model::{
        auth::{UserCredentialPo, UserProfilePo},
        sys_menu::SysMenuModel,
    },
};

#[async_trait]
pub trait ISysAuthRepository: Interface {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError>;
    async fn get_profile_by_user_id(&self, user_id: u64)
        -> Result<Option<UserProfilePo>, AppError>;
    async fn list_permissions_by_user_id(&self, user_id: u64) -> Result<Vec<String>, AppError>;
    async fn list_menus_by_user_id(&self, user_id: u64) -> Result<Vec<SysMenuModel>, AppError>;

    async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
        location: Option<&str>,
    ) -> Result<(), AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysAuthRepository)]
pub struct SysAuthRepository {
    pool: MySqlPool,
}

impl SysAuthRepository {
    pub(crate) async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserCredentialPo>, AppError> {
        query_as::<_, UserCredentialPo>(
            r#"
            SELECT id, username, nickname, password_hash, status
            FROM sys_user
            WHERE username = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户失败: {err}")))
    }

    pub(crate) async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
        location: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO sys_login_log (username, login_type, ip, location, status, message)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(username.unwrap_or_default())
        .bind(login_type)
        .bind(ip)
        .bind(location)
        .bind(status)
        .bind(message)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("写入登录日志失败: {err}")))?;
        Ok(())
    }

    pub(crate) async fn get_profile_by_user_id(
        &self,
        user_id: u64,
    ) -> Result<Option<UserProfilePo>, AppError> {
        query_as::<_, UserProfilePo>(
            r#"
            SELECT id, username, nickname
            FROM sys_user
            WHERE id = ? AND status = 1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询当前用户信息失败: {err}")))
    }

    pub(crate) async fn list_permissions_by_user_id(
        &self,
        user_id: u64,
    ) -> Result<Vec<String>, AppError> {
        let mut permissions = sqlx::query_scalar::<_, Option<String>>(
            r#"
            SELECT DISTINCT COALESCE(NULLIF(m.permission, ''), NULLIF(m.perms, '')) AS perm
            FROM sys_user_role ur
            JOIN sys_role r ON r.id = ur.role_id AND r.status = 1 AND r.is_deleted = 0
            JOIN sys_role_menu rm ON rm.role_id = ur.role_id
            JOIN sys_menu m ON m.id = rm.menu_id AND m.status = 1 AND m.is_deleted = 0
            WHERE ur.user_id = ?
              AND m.menu_type IN (2, 3)
            ORDER BY perm ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户权限失败: {err}")))?
        .into_iter()
        .flatten()
        .filter(|item| !item.trim().is_empty())
        .collect::<Vec<_>>();

        let is_super_admin = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(1)
            FROM sys_user_role ur
            JOIN sys_role r ON r.id = ur.role_id
            WHERE ur.user_id = ?
              AND r.is_deleted = 0
              AND (r.role_key = 'super_admin' OR r.id = 1)
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户角色失败: {err}")))?;

        if is_super_admin > 0 && !permissions.iter().any(|item| item == "*:*:*") {
            permissions.insert(0, "*:*:*".to_string());
        }

        Ok(permissions)
    }

    pub(crate) async fn list_menus_by_user_id(
        &self,
        user_id: u64,
    ) -> Result<Vec<SysMenuModel>, AppError> {
        query_as::<_, SysMenuModel>(
            r#"
            SELECT DISTINCT
                m.id, m.parent_id, m.menu_type, m.menu_name, m.route_name, m.route_path, m.component_path,
                m.perms, m.permission, m.icon, m.order_num, m.is_visible, m.status
            FROM sys_user_role ur
            JOIN sys_role r ON r.id = ur.role_id AND r.status = 1 AND r.is_deleted = 0
            JOIN sys_role_menu rm ON rm.role_id = ur.role_id
            JOIN sys_menu m ON m.id = rm.menu_id
            WHERE ur.user_id = ?
              AND m.status = 1
              AND m.is_visible = 1
              AND m.is_deleted = 0
              AND m.menu_type IN (1, 2)
            ORDER BY m.parent_id ASC, m.order_num ASC, m.id ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户菜单失败: {err}")))
    }
}

#[async_trait]
impl ISysAuthRepository for SysAuthRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError> {
        self.find_by_username(username).await
    }

    async fn get_profile_by_user_id(
        &self,
        user_id: u64,
    ) -> Result<Option<UserProfilePo>, AppError> {
        self.get_profile_by_user_id(user_id).await
    }

    async fn list_permissions_by_user_id(&self, user_id: u64) -> Result<Vec<String>, AppError> {
        self.list_permissions_by_user_id(user_id).await
    }

    async fn list_menus_by_user_id(&self, user_id: u64) -> Result<Vec<SysMenuModel>, AppError> {
        self.list_menus_by_user_id(user_id).await
    }

    async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
        location: Option<&str>,
    ) -> Result<(), AppError> {
        self.append_login_log(username, login_type, status, message, ip, location)
            .await
    }
}
