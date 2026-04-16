use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use bcrypt::verify;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use shaku::{Component, Interface};

use crate::{
    core::{
        common::JwtClaims,
        converter::{
            auth_converter::{to_auth_profile_user_vo, to_login_vo},
            sys_menu_converter::to_sys_menu_vo,
        },
        dto::auth_dto::LoginReqDto,
        errors::AppError,
        model::auth::UserCredentialPo,
        utils::ip_util,
        vo::{
            auth_vo::{AuthProfileVo, LoginVo},
            sys_menu_vo::SysMenuVo,
        },
    },
    modules::system::repository::ISysAuthRepository,
};

#[async_trait]
pub trait ISysAuthService: Interface {
    async fn login(
        &self,
        payload: LoginReqDto,
        client_ip: Option<String>,
    ) -> Result<LoginVo, AppError>;

    fn verify_token(&self, token: &str) -> Result<JwtClaims, AppError>;
    async fn profile(&self, user_id: u64) -> Result<AuthProfileVo, AppError>;
    async fn has_permission(
        &self,
        user_id: u64,
        required_permission: &str,
    ) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysAuthService)]
pub struct SysAuthService {
    #[shaku(inject)]
    repo: Arc<dyn ISysAuthRepository>,
    jwt_secret: Arc<String>,
    jwt_expires_secs: u64,
}

impl SysAuthService {
    pub async fn login(
        &self,
        payload: LoginReqDto,
        client_ip: Option<String>,
    ) -> Result<LoginVo, AppError> {
        let username = payload.username.trim();
        let password = payload.password.trim();
        let ip = client_ip.unwrap_or_default();

        if username.is_empty() || password.is_empty() {
            self.append_login_log(None, 3, 0, "用户名和密码不能为空", &ip)
                .await;
            return Err(AppError::bad_request("用户名和密码不能为空"));
        }

        let user = self
            .repo
            .find_by_username(username)
            .await?
            .ok_or_else(|| AppError::unauthorized("用户名或密码错误"))?;

        if !verify_password(password, &user)? {
            self.append_login_log(Some(username), 3, 0, "用户名或密码错误", &ip)
                .await;
            return Err(AppError::unauthorized("用户名或密码错误"));
        }

        if user.status != 1 {
            self.append_login_log(Some(username), 3, 0, "账号已停用", &ip)
                .await;
            return Err(AppError::unauthorized("账号已停用"));
        }

        let now_secs = now_unix_secs();
        let claims = JwtClaims {
            sub: user.id,
            username: user.username.clone(),
            iat: now_secs as usize,
            exp: (now_secs + self.jwt_expires_secs) as usize,
        };
        let token = self.generate_token(&claims)?;
        self.append_login_log(Some(username), 1, 1, "登录成功", &ip)
            .await;

        Ok(to_login_vo(&user, token, self.jwt_expires_secs))
    }

    pub fn verify_token(&self, token: &str) -> Result<JwtClaims, AppError> {
        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::unauthorized("无效或已过期的令牌"))
    }

    pub async fn profile(&self, user_id: u64) -> Result<AuthProfileVo, AppError> {
        let user = self
            .repo
            .get_profile_by_user_id(user_id)
            .await?
            .ok_or_else(|| AppError::unauthorized("用户不存在或已停用"))?;

        let permissions = self.repo.list_permissions_by_user_id(user_id).await?;
        let menus = self
            .repo
            .list_menus_by_user_id(user_id)
            .await?
            .into_iter()
            .map(to_sys_menu_vo)
            .collect::<Vec<_>>();

        Ok(AuthProfileVo {
            user: to_auth_profile_user_vo(user),
            permissions,
            menus: build_menu_tree(menus),
        })
    }

    pub async fn has_permission(
        &self,
        user_id: u64,
        required_permission: &str,
    ) -> Result<bool, AppError> {
        let normalized_permission = required_permission.trim();
        if normalized_permission.is_empty() {
            return Ok(true);
        }

        let permissions = self.repo.list_permissions_by_user_id(user_id).await?;
        Ok(permissions.iter().any(|item| {
            let candidate = item.trim();
            candidate == "*:*:*" || candidate == normalized_permission
        }))
    }

    async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
    ) {
        let mut location = None;
        if !ip.is_empty() && ip != "127.0.0.1" && ip != "::1" {
            if let Some(info) = ip_util::get_ip_location(ip).await {
                location = Some(ip_util::format_location(&info));
            }
        }

        let _ = self
            .repo
            .append_login_log(
                username,
                login_type,
                status,
                message,
                ip,
                location.as_deref(),
            )
            .await;
    }

    fn generate_token(&self, claims: &JwtClaims) -> Result<String, AppError> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|err| AppError::internal(format!("生成令牌失败: {err}")))
    }
}

#[async_trait]
impl ISysAuthService for SysAuthService {
    async fn login(
        &self,
        payload: LoginReqDto,
        client_ip: Option<String>,
    ) -> Result<LoginVo, AppError> {
        self.login(payload, client_ip).await
    }

    fn verify_token(&self, token: &str) -> Result<JwtClaims, AppError> {
        self.verify_token(token)
    }

    async fn profile(&self, user_id: u64) -> Result<AuthProfileVo, AppError> {
        self.profile(user_id).await
    }

    async fn has_permission(
        &self,
        user_id: u64,
        required_permission: &str,
    ) -> Result<bool, AppError> {
        self.has_permission(user_id, required_permission).await
    }
}

fn verify_password(raw_password: &str, user: &UserCredentialPo) -> Result<bool, AppError> {
    if user.password_hash.trim().is_empty() {
        return Ok(false);
    }

    if looks_like_bcrypt_hash(&user.password_hash) {
        return verify(raw_password, &user.password_hash)
            .map_err(|err| AppError::internal(format!("密码校验失败: {err}")));
    }

    Ok(raw_password == user.password_hash)
}

fn looks_like_bcrypt_hash(value: &str) -> bool {
    value.starts_with("$2a$") || value.starts_with("$2b$") || value.starts_with("$2y$")
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or(0)
}

fn build_menu_tree(items: Vec<SysMenuVo>) -> Vec<SysMenuVo> {
    if items.is_empty() {
        return Vec::new();
    }

    let mut index_by_id: HashMap<u64, usize> = HashMap::new();
    for (index, item) in items.iter().enumerate() {
        index_by_id.insert(item.id, index);
    }

    let mut children_map: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut root_indices = Vec::new();
    for (index, item) in items.iter().enumerate() {
        if item.parent_id == 0
            || !index_by_id.contains_key(&item.parent_id)
            || item.parent_id == item.id
        {
            root_indices.push(index);
        } else {
            children_map.entry(item.parent_id).or_default().push(index);
        }
    }

    let mut visited = HashSet::new();
    root_indices
        .into_iter()
        .map(|index| build_menu_tree_node(index, &items, &children_map, &mut visited))
        .collect()
}

fn build_menu_tree_node(
    index: usize,
    items: &[SysMenuVo],
    children_map: &HashMap<u64, Vec<usize>>,
    visited: &mut HashSet<u64>,
) -> SysMenuVo {
    let mut node = items[index].clone();
    if !visited.insert(node.id) {
        return node;
    }

    if let Some(children_indices) = children_map.get(&node.id) {
        node.children = children_indices
            .iter()
            .map(|child_index| build_menu_tree_node(*child_index, items, children_map, visited))
            .collect();
    }

    visited.remove(&node.id);
    node
}
