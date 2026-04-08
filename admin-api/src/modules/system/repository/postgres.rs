use std::sync::Arc;

use async_trait::async_trait;
use serde_json::{Map, Value};
use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row};

use crate::core::errors::AppError;

use super::{CrudRecord, SystemRepository};

const SUPPORTED_RESOURCES: [&str; 8] = [
    "user", "role", "menu", "dept", "post", "dict", "config", "notice",
];

const DEFAULT_PASSWORD_HASH: &str = "$2b$10$jh6uvsoSAuxAfUYOc5ckkecacY3x2zPL0GuvlX38JCpRHM2OtoByi";

#[derive(Debug, Clone)]
pub struct PostgresSystemRepository {
    pool: PgPool,
}

impl PostgresSystemRepository {
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    async fn list(
        &self,
        resource: &str,
        keyword: Option<&str>,
    ) -> Result<Vec<CrudRecord>, AppError> {
        match resource {
            "user" => self.list_users(keyword).await,
            "role" => self.list_roles(keyword).await,
            "menu" => self.list_menus(keyword).await,
            "dept" => self.list_depts(keyword).await,
            "post" => self.list_posts(keyword).await,
            "dict" => self.list_dict_data(keyword).await,
            "config" => self.list_configs(keyword).await,
            "notice" => self.list_notices(keyword).await,
            _ => Err(AppError::bad_request(format!(
                "不支持的资源类型: {resource}"
            ))),
        }
    }

    async fn get_by_id(&self, resource: &str, id: u64) -> Result<Option<CrudRecord>, AppError> {
        match resource {
            "user" => self.get_user_by_id(id).await,
            "role" => self.get_role_by_id(id).await,
            "menu" => self.get_menu_by_id(id).await,
            "dept" => self.get_dept_by_id(id).await,
            "post" => self.get_post_by_id(id).await,
            "dict" => self.get_dict_data_by_id(id).await,
            "config" => self.get_config_by_id(id).await,
            "notice" => self.get_notice_by_id(id).await,
            _ => Err(AppError::bad_request(format!(
                "不支持的资源类型: {resource}"
            ))),
        }
    }

    async fn create(&self, resource: &str, payload: CrudRecord) -> Result<CrudRecord, AppError> {
        let created_id = match resource {
            "user" => self.create_user(&payload).await?,
            "role" => self.create_role(&payload).await?,
            "menu" => self.create_menu(&payload).await?,
            "dept" => self.create_dept(&payload).await?,
            "post" => self.create_post(&payload).await?,
            "dict" => self.create_dict_data(&payload).await?,
            "config" => self.create_config(&payload).await?,
            "notice" => self.create_notice(&payload).await?,
            _ => {
                return Err(AppError::bad_request(format!(
                    "不支持的资源类型: {resource}"
                )))
            }
        };

        self.get_by_id(resource, created_id)
            .await?
            .ok_or_else(|| AppError::internal("创建成功但读取记录失败"))
    }

    async fn update(
        &self,
        resource: &str,
        id: u64,
        payload: CrudRecord,
    ) -> Result<Option<CrudRecord>, AppError> {
        let affected = match resource {
            "user" => self.update_user(id, &payload).await?,
            "role" => self.update_role(id, &payload).await?,
            "menu" => self.update_menu(id, &payload).await?,
            "dept" => self.update_dept(id, &payload).await?,
            "post" => self.update_post(id, &payload).await?,
            "dict" => self.update_dict_data(id, &payload).await?,
            "config" => self.update_config(id, &payload).await?,
            "notice" => self.update_notice(id, &payload).await?,
            _ => {
                return Err(AppError::bad_request(format!(
                    "不支持的资源类型: {resource}"
                )))
            }
        };

        if !affected {
            return Ok(None);
        }

        self.get_by_id(resource, id).await
    }

    async fn delete(&self, resource: &str, id: u64) -> Result<bool, AppError> {
        let sql = match resource {
            "user" => "UPDATE sys_user SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            "role" => "UPDATE sys_role SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            "menu" => "UPDATE sys_menu SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            "dept" => "UPDATE sys_dept SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            "post" => "UPDATE sys_post SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            "dict" => "UPDATE sys_dict_data SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            "config" => "UPDATE sys_config SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            "notice" => "UPDATE sys_notice SET is_deleted = 1 WHERE id = $1 AND is_deleted = 0",
            _ => {
                return Err(AppError::bad_request(format!(
                    "不支持的资源类型: {resource}"
                )))
            }
        };

        let result = sqlx::query(sql)
            .bind(to_i64(id, "id")?)
            .execute(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("删除 {resource} 失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_users(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT id, username, nickname, phone, status
            FROM sys_user
            WHERE is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (username ILIKE ")
                .push_bind(like.clone())
                .push(" OR nickname ILIKE ")
                .push_bind(like.clone())
                .push(" OR phone ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY id DESC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询用户失败: {err}")))?;

        Ok(rows.into_iter().map(map_user_row).collect())
    }

    async fn get_user_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, nickname, phone, status
            FROM sys_user
            WHERE id = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户失败: {err}")))?;
        Ok(row.map(map_user_row))
    }

    async fn create_user(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let username = required_string(payload, "username", "用户名")?;
        let nickname = required_string(payload, "nickname", "昵称")?;
        let phone = optional_string(payload, "phone");
        let status = enabled_status(payload.get("status"));

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_user (
                username, nickname, password_hash, phone, status,
                created_by, updated_by, is_deleted
            ) VALUES ($1, $2, $3, $4, $5, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(username)
        .bind(nickname)
        .bind(DEFAULT_PASSWORD_HASH)
        .bind(phone)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增用户失败: {err}")))?;

        Ok(created_id as u64)
    }

    async fn update_user(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let username = required_string(payload, "username", "用户名")?;
        let nickname = required_string(payload, "nickname", "昵称")?;
        let phone = optional_string(payload, "phone");
        let status = enabled_status(payload.get("status"));

        let result = sqlx::query(
            r#"
            UPDATE sys_user
            SET username = $1, nickname = $2, phone = $3, status = $4, updated_by = 1
            WHERE id = $5 AND is_deleted = 0
            "#,
        )
        .bind(username)
        .bind(nickname)
        .bind(phone)
        .bind(status)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新用户失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_roles(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT id, role_name, role_key, role_sort, status
            FROM sys_role
            WHERE is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (role_name ILIKE ")
                .push_bind(like.clone())
                .push(" OR role_key ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY id DESC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询角色失败: {err}")))?;

        Ok(rows.into_iter().map(map_role_row).collect())
    }

    async fn get_role_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, role_name, role_key, role_sort, status
            FROM sys_role
            WHERE id = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询角色失败: {err}")))?;
        Ok(row.map(map_role_row))
    }

    async fn create_role(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let name = required_string(payload, "name", "角色名称")?;
        let key = required_string(payload, "key", "权限标识")?;
        let sort = optional_i32(payload, "sort").unwrap_or(1);
        let status = enabled_status(payload.get("status"));

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_role (
                role_name, role_key, role_sort, status, data_scope, created_by, updated_by, is_deleted
            ) VALUES ($1, $2, $3, $4, 5, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(name)
        .bind(key)
        .bind(sort)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增角色失败: {err}")))?;
        Ok(created_id as u64)
    }

    async fn update_role(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let name = required_string(payload, "name", "角色名称")?;
        let key = required_string(payload, "key", "权限标识")?;
        let sort = optional_i32(payload, "sort").unwrap_or(1);
        let status = enabled_status(payload.get("status"));

        let result = sqlx::query(
            r#"
            UPDATE sys_role
            SET role_name = $1, role_key = $2, role_sort = $3, status = $4, updated_by = 1
            WHERE id = $5 AND is_deleted = 0
            "#,
        )
        .bind(name)
        .bind(key)
        .bind(sort)
        .bind(status)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新角色失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_menus(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT id, parent_id, menu_name, route_path, component_path, is_visible
            FROM sys_menu
            WHERE is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (menu_name ILIKE ")
                .push_bind(like.clone())
                .push(" OR route_path ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY parent_id ASC, order_num ASC, id ASC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询菜单失败: {err}")))?;
        Ok(rows.into_iter().map(map_menu_row).collect())
    }

    async fn get_menu_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, parent_id, menu_name, route_path, component_path, is_visible
            FROM sys_menu
            WHERE id = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询菜单失败: {err}")))?;
        Ok(row.map(map_menu_row))
    }

    async fn create_menu(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let parent_id = required_u64(payload, "parent_id", "上级菜单")?;
        self.ensure_menu_parent_exists(parent_id).await?;
        let name = required_string(payload, "name", "菜单名称")?;
        let path = required_string(payload, "path", "路由地址")?;
        let component = required_string(payload, "component", "组件名")?;
        let visible = visible_status(payload.get("visible"));

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_menu (
                parent_id, menu_type, menu_name, route_name, route_path, component_path,
                perms, order_num, is_visible, status, created_by, updated_by, is_deleted
            ) VALUES ($1, 2, $2, $3, $4, $5, NULL, 0, $6, 1, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(to_i64(parent_id, "parent_id")?)
        .bind(&name)
        .bind(&name)
        .bind(path)
        .bind(component)
        .bind(visible)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增菜单失败: {err}")))?;
        Ok(created_id as u64)
    }

    async fn update_menu(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let name = required_string(payload, "name", "菜单名称")?;
        let path = required_string(payload, "path", "路由地址")?;
        let component = required_string(payload, "component", "组件名")?;
        let visible = visible_status(payload.get("visible"));

        let result = sqlx::query(
            r#"
            UPDATE sys_menu
            SET menu_name = $1, route_name = $2, route_path = $3, component_path = $4, is_visible = $5, updated_by = 1
            WHERE id = $6 AND is_deleted = 0
            "#,
        )
        .bind(&name)
        .bind(&name)
        .bind(path)
        .bind(component)
        .bind(visible)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新菜单失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_depts(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT id, parent_id, dept_name, leader, phone, status
            FROM sys_dept
            WHERE is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (dept_name ILIKE ")
                .push_bind(like.clone())
                .push(" OR leader ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY parent_id ASC, order_num ASC, id ASC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询部门失败: {err}")))?;
        Ok(rows.into_iter().map(map_dept_row).collect())
    }

    async fn get_dept_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, parent_id, dept_name, leader, phone, status
            FROM sys_dept
            WHERE id = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询部门失败: {err}")))?;
        Ok(row.map(map_dept_row))
    }

    async fn create_dept(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let parent_id = required_u64(payload, "parent_id", "上级部门")?;
        let ancestors = self.resolve_dept_ancestors(parent_id).await?;
        let name = required_string(payload, "name", "部门名称")?;
        let leader = optional_string(payload, "leader");
        let phone = optional_string(payload, "phone");
        let status = enabled_status(payload.get("status"));

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_dept (
                parent_id, ancestors, dept_name, order_num, leader, phone, status,
                created_by, updated_by, is_deleted
            ) VALUES ($1, $2, $3, 0, $4, $5, $6, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(to_i64(parent_id, "parent_id")?)
        .bind(ancestors)
        .bind(name)
        .bind(leader)
        .bind(phone)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增部门失败: {err}")))?;
        Ok(created_id as u64)
    }

    async fn update_dept(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let name = required_string(payload, "name", "部门名称")?;
        let leader = optional_string(payload, "leader");
        let phone = optional_string(payload, "phone");
        let status = enabled_status(payload.get("status"));

        let result = sqlx::query(
            r#"
            UPDATE sys_dept
            SET dept_name = $1, leader = $2, phone = $3, status = $4, updated_by = 1
            WHERE id = $5 AND is_deleted = 0
            "#,
        )
        .bind(name)
        .bind(leader)
        .bind(phone)
        .bind(status)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新部门失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_posts(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT id, post_name, post_code, post_sort, status
            FROM sys_post
            WHERE is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (post_name ILIKE ")
                .push_bind(like.clone())
                .push(" OR post_code ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY id DESC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询岗位失败: {err}")))?;
        Ok(rows.into_iter().map(map_post_row).collect())
    }

    async fn get_post_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, post_name, post_code, post_sort, status
            FROM sys_post
            WHERE id = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询岗位失败: {err}")))?;
        Ok(row.map(map_post_row))
    }

    async fn create_post(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let name = required_string(payload, "name", "岗位名称")?;
        let code = required_string(payload, "code", "岗位编码")?;
        let sort = optional_i32(payload, "sort").unwrap_or(1);
        let status = enabled_status(payload.get("status"));

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_post (
                post_code, post_name, post_sort, status,
                created_by, updated_by, is_deleted
            ) VALUES ($1, $2, $3, $4, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(code)
        .bind(name)
        .bind(sort)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增岗位失败: {err}")))?;
        Ok(created_id as u64)
    }

    async fn update_post(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let name = required_string(payload, "name", "岗位名称")?;
        let code = required_string(payload, "code", "岗位编码")?;
        let sort = optional_i32(payload, "sort").unwrap_or(1);
        let status = enabled_status(payload.get("status"));

        let result = sqlx::query(
            r#"
            UPDATE sys_post
            SET post_name = $1, post_code = $2, post_sort = $3, status = $4, updated_by = 1
            WHERE id = $5 AND is_deleted = 0
            "#,
        )
        .bind(name)
        .bind(code)
        .bind(sort)
        .bind(status)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新岗位失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_dict_data(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT d.id, t.dict_type, d.label, d.value, d.status
            FROM sys_dict_data d
            INNER JOIN sys_dict_type t ON d.dict_type_id = t.id
            WHERE d.is_deleted = 0 AND t.is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (t.dict_type ILIKE ")
                .push_bind(like.clone())
                .push(" OR d.label ILIKE ")
                .push_bind(like.clone())
                .push(" OR d.value ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY d.id DESC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询字典失败: {err}")))?;
        Ok(rows.into_iter().map(map_dict_row).collect())
    }

    async fn get_dict_data_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT d.id, t.dict_type, d.label, d.value, d.status
            FROM sys_dict_data d
            INNER JOIN sys_dict_type t ON d.dict_type_id = t.id
            WHERE d.id = $1 AND d.is_deleted = 0 AND t.is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询字典失败: {err}")))?;
        Ok(row.map(map_dict_row))
    }

    async fn create_dict_data(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let dict_type = required_string(payload, "type", "字典类型")?;
        let label = required_string(payload, "label", "字典标签")?;
        let value = required_string(payload, "value", "字典值")?;
        let status = enabled_status(payload.get("status"));
        let dict_type_id = self.ensure_dict_type_id(&dict_type).await?;

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_dict_data (
                dict_type_id, label, value, status, sort,
                created_by, updated_by, is_deleted
            ) VALUES ($1, $2, $3, $4, 0, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(to_i64(dict_type_id, "dict_type_id")?)
        .bind(label)
        .bind(value)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增字典失败: {err}")))?;
        Ok(created_id as u64)
    }

    async fn update_dict_data(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let dict_type = required_string(payload, "type", "字典类型")?;
        let label = required_string(payload, "label", "字典标签")?;
        let value = required_string(payload, "value", "字典值")?;
        let status = enabled_status(payload.get("status"));
        let dict_type_id = self.ensure_dict_type_id(&dict_type).await?;

        let result = sqlx::query(
            r#"
            UPDATE sys_dict_data
            SET dict_type_id = $1, label = $2, value = $3, status = $4, updated_by = 1
            WHERE id = $5 AND is_deleted = 0
            "#,
        )
        .bind(to_i64(dict_type_id, "dict_type_id")?)
        .bind(label)
        .bind(value)
        .bind(status)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新字典失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_configs(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT id, config_key, config_value, remark, status
            FROM sys_config
            WHERE is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (config_key ILIKE ")
                .push_bind(like.clone())
                .push(" OR config_value ILIKE ")
                .push_bind(like.clone())
                .push(" OR remark ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY id DESC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询参数失败: {err}")))?;
        Ok(rows.into_iter().map(map_config_row).collect())
    }

    async fn get_config_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, config_key, config_value, remark, status
            FROM sys_config
            WHERE id = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询参数失败: {err}")))?;
        Ok(row.map(map_config_row))
    }

    async fn create_config(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let name = required_string(payload, "name", "参数名称")?;
        let value = required_string(payload, "value", "参数值")?;
        let remark = optional_string(payload, "remark");
        let status = enabled_status(payload.get("status"));

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_config (
                config_name, config_key, config_value, config_type, status, remark,
                created_by, updated_by, is_deleted
            ) VALUES ($1, $2, $3, 0, $4, $5, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(&name)
        .bind(&name)
        .bind(value)
        .bind(status)
        .bind(remark)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增参数失败: {err}")))?;
        Ok(created_id as u64)
    }

    async fn update_config(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let name = required_string(payload, "name", "参数名称")?;
        let value = required_string(payload, "value", "参数值")?;
        let remark = optional_string(payload, "remark");
        let status = enabled_status(payload.get("status"));

        let result = sqlx::query(
            r#"
            UPDATE sys_config
            SET config_name = $1, config_key = $2, config_value = $3, status = $4, remark = $5, updated_by = 1
            WHERE id = $6 AND is_deleted = 0
            "#,
        )
        .bind(&name)
        .bind(&name)
        .bind(value)
        .bind(status)
        .bind(remark)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新参数失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_notices(&self, keyword: Option<&str>) -> Result<Vec<CrudRecord>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT n.id, n.title, n.notice_type, n.status, COALESCE(u.username, '') AS publisher
            FROM sys_notice n
            LEFT JOIN sys_user u ON n.published_by = u.id
            WHERE n.is_deleted = 0
            "#,
        );
        if let Some(like) = keyword_like(keyword) {
            qb.push(" AND (n.title ILIKE ")
                .push_bind(like.clone())
                .push(" OR COALESCE(u.username, '') ILIKE ")
                .push_bind(like.clone())
                .push(")");
        }
        qb.push(" ORDER BY n.id DESC");

        let rows = qb
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("查询公告失败: {err}")))?;
        Ok(rows.into_iter().map(map_notice_row).collect())
    }

    async fn get_notice_by_id(&self, id: u64) -> Result<Option<CrudRecord>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT n.id, n.title, n.notice_type, n.status, COALESCE(u.username, '') AS publisher
            FROM sys_notice n
            LEFT JOIN sys_user u ON n.published_by = u.id
            WHERE n.id = $1 AND n.is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(id, "id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询公告失败: {err}")))?;
        Ok(row.map(map_notice_row))
    }

    async fn create_notice(&self, payload: &CrudRecord) -> Result<u64, AppError> {
        let title = required_string(payload, "title", "公告标题")?;
        let notice_type = notice_type(payload.get("type"));
        let status = notice_status(payload.get("status"));
        let publisher_username = optional_string(payload, "publisher");
        let publisher_id = self
            .resolve_user_id_by_username(publisher_username.as_deref())
            .await?;

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_notice (
                title, notice_type, summary, content, status, published_by, published_at,
                created_by, updated_by, is_deleted
            ) VALUES ($1, $2, '', $3, $4, $5, CASE WHEN $6 = 1 THEN CURRENT_TIMESTAMP(3) ELSE NULL END, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(&title)
        .bind(notice_type)
        .bind(format!("{title}\n（由系统管理页创建）"))
        .bind(status)
        .bind(publisher_id.map(|id| id as i64))
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增公告失败: {err}")))?;
        Ok(created_id as u64)
    }

    async fn update_notice(&self, id: u64, payload: &CrudRecord) -> Result<bool, AppError> {
        let title = required_string(payload, "title", "公告标题")?;
        let notice_type = notice_type(payload.get("type"));
        let status = notice_status(payload.get("status"));
        let publisher_username = optional_string(payload, "publisher");
        let publisher_id = self
            .resolve_user_id_by_username(publisher_username.as_deref())
            .await?;

        let result = sqlx::query(
            r#"
            UPDATE sys_notice
            SET title = $1, notice_type = $2, content = $3, status = $4, published_by = $5,
                published_at = CASE WHEN $6 = 1 THEN CURRENT_TIMESTAMP(3) ELSE NULL END, updated_by = 1
            WHERE id = $7 AND is_deleted = 0
            "#,
        )
        .bind(&title)
        .bind(notice_type)
        .bind(format!("{title}\n（由系统管理页更新）"))
        .bind(status)
        .bind(publisher_id.map(|id| id as i64))
        .bind(status)
        .bind(to_i64(id, "id")?)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新公告失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn ensure_menu_parent_exists(&self, parent_id: u64) -> Result<(), AppError> {
        if parent_id == 0 {
            return Ok(());
        }

        let exists = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(1)
            FROM sys_menu
            WHERE id = $1 AND is_deleted = 0
            "#,
        )
        .bind(to_i64(parent_id, "parent_id")?)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询上级菜单失败: {err}")))?;

        if exists == 0 {
            return Err(AppError::bad_request("上级菜单不存在"));
        }
        Ok(())
    }

    async fn resolve_dept_ancestors(&self, parent_id: u64) -> Result<String, AppError> {
        if parent_id == 0 {
            return Ok("0".to_string());
        }

        let parent = sqlx::query(
            r#"
            SELECT ancestors
            FROM sys_dept
            WHERE id = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(to_i64(parent_id, "parent_id")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询上级部门失败: {err}")))?;

        let Some(parent) = parent else {
            return Err(AppError::bad_request("上级部门不存在"));
        };

        let parent_ancestors = parent
            .get::<Option<String>, _>("ancestors")
            .unwrap_or_default();
        let normalized = if parent_ancestors.trim().is_empty() {
            "0".to_string()
        } else {
            parent_ancestors
        };
        Ok(format!("{normalized},{parent_id}"))
    }

    async fn ensure_dict_type_id(&self, dict_type: &str) -> Result<u64, AppError> {
        let found = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT id FROM sys_dict_type
            WHERE dict_type = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(dict_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询字典类型失败: {err}")))?;

        if let Some(id) = found {
            return Ok(id as u64);
        }

        let created_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO sys_dict_type (dict_name, dict_type, status, created_by, updated_by, is_deleted)
            VALUES ($1, $2, 1, 1, 1, 0)
            RETURNING id
            "#,
        )
        .bind(dict_type)
        .bind(dict_type)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("创建字典类型失败: {err}")))?;

        Ok(created_id as u64)
    }

    async fn resolve_user_id_by_username(
        &self,
        username: Option<&str>,
    ) -> Result<Option<u64>, AppError> {
        let Some(username) = username else {
            return Ok(None);
        };
        if username.trim().is_empty() {
            return Ok(None);
        }

        let user_id = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT id
            FROM sys_user
            WHERE username = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询发布人失败: {err}")))?;
        Ok(user_id.map(|value| value as u64))
    }
}

#[async_trait]
impl SystemRepository for PostgresSystemRepository {
    async fn list(
        &self,
        resource: &str,
        keyword: Option<&str>,
    ) -> Result<Vec<CrudRecord>, AppError> {
        PostgresSystemRepository::list(self, resource, keyword).await
    }

    async fn get_by_id(&self, resource: &str, id: u64) -> Result<Option<CrudRecord>, AppError> {
        PostgresSystemRepository::get_by_id(self, resource, id).await
    }

    async fn create(&self, resource: &str, payload: CrudRecord) -> Result<CrudRecord, AppError> {
        PostgresSystemRepository::create(self, resource, payload).await
    }

    async fn update(
        &self,
        resource: &str,
        id: u64,
        payload: CrudRecord,
    ) -> Result<Option<CrudRecord>, AppError> {
        PostgresSystemRepository::update(self, resource, id, payload).await
    }

    async fn delete(&self, resource: &str, id: u64) -> Result<bool, AppError> {
        PostgresSystemRepository::delete(self, resource, id).await
    }
}

fn to_i64(value: u64, field: &str) -> Result<i64, AppError> {
    i64::try_from(value).map_err(|_| AppError::bad_request(format!("{field} 超出范围")))
}

fn keyword_like(keyword: Option<&str>) -> Option<String> {
    let kw = keyword.unwrap_or_default().trim();
    if kw.is_empty() {
        None
    } else {
        Some(format!("%{kw}%"))
    }
}

fn required_string(payload: &CrudRecord, key: &str, label: &str) -> Result<String, AppError> {
    let value = optional_string(payload, key).unwrap_or_default();
    if value.is_empty() {
        return Err(AppError::bad_request(format!("{label}不能为空")));
    }
    Ok(value)
}

fn required_u64(payload: &CrudRecord, key: &str, label: &str) -> Result<u64, AppError> {
    optional_u64(payload, key).ok_or_else(|| AppError::bad_request(format!("{label}不能为空")))
}

fn optional_string(payload: &CrudRecord, key: &str) -> Option<String> {
    payload
        .get(key)
        .and_then(|value| match value {
            Value::String(text) => Some(text.trim().to_string()),
            Value::Number(num) => Some(num.to_string()),
            Value::Bool(boolean) => Some(boolean.to_string()),
            _ => None,
        })
        .filter(|value| !value.is_empty())
}

fn optional_i32(payload: &CrudRecord, key: &str) -> Option<i32> {
    payload.get(key).and_then(|value| match value {
        Value::Number(num) => num.as_i64().map(|v| v as i32),
        Value::String(text) => text.trim().parse::<i32>().ok(),
        _ => None,
    })
}

fn optional_u64(payload: &CrudRecord, key: &str) -> Option<u64> {
    payload.get(key).and_then(|value| match value {
        Value::Number(num) => num.as_u64().or_else(|| {
            num.as_i64()
                .and_then(|v| if v >= 0 { Some(v as u64) } else { None })
        }),
        Value::String(text) => text.trim().parse::<u64>().ok(),
        _ => None,
    })
}

fn enabled_status(value: Option<&Value>) -> i16 {
    match value.and_then(Value::as_str) {
        Some("disabled") => 0,
        _ => 1,
    }
}

fn enabled_status_label(value: i16) -> &'static str {
    if value == 1 {
        "enabled"
    } else {
        "disabled"
    }
}

fn visible_status(value: Option<&Value>) -> i16 {
    match value.and_then(Value::as_str) {
        Some("no") => 0,
        _ => 1,
    }
}

fn visible_label(value: i16) -> &'static str {
    if value == 1 {
        "yes"
    } else {
        "no"
    }
}

fn notice_type(value: Option<&Value>) -> i16 {
    match value.and_then(Value::as_str) {
        Some("公告") => 2,
        _ => 1,
    }
}

fn notice_type_label(value: i16) -> &'static str {
    if value == 2 {
        "公告"
    } else {
        "通知"
    }
}

fn notice_status(value: Option<&Value>) -> i16 {
    match value.and_then(Value::as_str) {
        Some("published") => 1,
        Some("offline") => 2,
        _ => 0,
    }
}

fn notice_status_label(value: i16) -> &'static str {
    match value {
        1 => "published",
        2 => "offline",
        _ => "draft",
    }
}

fn map_user_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "username".to_string(),
        Value::from(row.get::<String, _>("username")),
    );
    record.insert(
        "nickname".to_string(),
        Value::from(row.get::<String, _>("nickname")),
    );
    record.insert(
        "phone".to_string(),
        Value::from(row.get::<Option<String>, _>("phone").unwrap_or_default()),
    );
    record.insert(
        "status".to_string(),
        Value::from(enabled_status_label(row.get::<i16, _>("status"))),
    );
    record
}

fn map_role_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "name".to_string(),
        Value::from(row.get::<String, _>("role_name")),
    );
    record.insert(
        "key".to_string(),
        Value::from(row.get::<String, _>("role_key")),
    );
    record.insert(
        "sort".to_string(),
        Value::from(row.get::<i32, _>("role_sort")),
    );
    record.insert(
        "status".to_string(),
        Value::from(enabled_status_label(row.get::<i16, _>("status"))),
    );
    record
}

fn map_menu_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "parent_id".to_string(),
        Value::from(row.get::<i64, _>("parent_id") as u64),
    );
    record.insert(
        "name".to_string(),
        Value::from(row.get::<String, _>("menu_name")),
    );
    record.insert(
        "path".to_string(),
        Value::from(
            row.get::<Option<String>, _>("route_path")
                .unwrap_or_default(),
        ),
    );
    record.insert(
        "component".to_string(),
        Value::from(
            row.get::<Option<String>, _>("component_path")
                .unwrap_or_default(),
        ),
    );
    record.insert(
        "visible".to_string(),
        Value::from(visible_label(row.get::<i16, _>("is_visible"))),
    );
    record
}

fn map_dept_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "parent_id".to_string(),
        Value::from(row.get::<i64, _>("parent_id") as u64),
    );
    record.insert(
        "name".to_string(),
        Value::from(row.get::<String, _>("dept_name")),
    );
    record.insert(
        "leader".to_string(),
        Value::from(row.get::<Option<String>, _>("leader").unwrap_or_default()),
    );
    record.insert(
        "phone".to_string(),
        Value::from(row.get::<Option<String>, _>("phone").unwrap_or_default()),
    );
    record.insert(
        "status".to_string(),
        Value::from(enabled_status_label(row.get::<i16, _>("status"))),
    );
    record
}

fn map_post_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "name".to_string(),
        Value::from(row.get::<String, _>("post_name")),
    );
    record.insert(
        "code".to_string(),
        Value::from(row.get::<String, _>("post_code")),
    );
    record.insert(
        "sort".to_string(),
        Value::from(row.get::<i32, _>("post_sort")),
    );
    record.insert(
        "status".to_string(),
        Value::from(enabled_status_label(row.get::<i16, _>("status"))),
    );
    record
}

fn map_dict_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "type".to_string(),
        Value::from(row.get::<String, _>("dict_type")),
    );
    record.insert(
        "label".to_string(),
        Value::from(row.get::<String, _>("label")),
    );
    record.insert(
        "value".to_string(),
        Value::from(row.get::<String, _>("value")),
    );
    record.insert(
        "status".to_string(),
        Value::from(enabled_status_label(row.get::<i16, _>("status"))),
    );
    record
}

fn map_config_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "name".to_string(),
        Value::from(row.get::<String, _>("config_key")),
    );
    record.insert(
        "value".to_string(),
        Value::from(row.get::<String, _>("config_value")),
    );
    record.insert(
        "remark".to_string(),
        Value::from(row.get::<Option<String>, _>("remark").unwrap_or_default()),
    );
    record.insert(
        "status".to_string(),
        Value::from(enabled_status_label(row.get::<i16, _>("status"))),
    );
    record
}

fn map_notice_row(row: PgRow) -> CrudRecord {
    let mut record = Map::new();
    record.insert(
        "id".to_string(),
        Value::from(row.get::<i64, _>("id") as u64),
    );
    record.insert(
        "title".to_string(),
        Value::from(row.get::<String, _>("title")),
    );
    record.insert(
        "type".to_string(),
        Value::from(notice_type_label(row.get::<i16, _>("notice_type"))),
    );
    record.insert(
        "status".to_string(),
        Value::from(notice_status_label(row.get::<i16, _>("status"))),
    );
    record.insert(
        "publisher".to_string(),
        Value::from(row.get::<String, _>("publisher")),
    );
    record
}

pub fn supports_resource(resource: &str) -> bool {
    SUPPORTED_RESOURCES.contains(&resource)
}
