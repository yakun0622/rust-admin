pub mod integration;

use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use bcrypt::verify;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        converter::auth::to_login_vo, dto::auth::LoginReqDto, errors::AppError,
        model::auth::UserCredentialPo, vo::auth::LoginVo,
    },
    modules::auth::repository::{AuthRepository, LoginAuditRecord},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: u64,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct AuthService {
    repo: Arc<dyn AuthRepository>,
    jwt_secret: Arc<String>,
    jwt_expires_secs: u64,
}

impl AuthService {
    pub fn new(repo: Arc<dyn AuthRepository>, jwt_secret: String, jwt_expires_secs: u64) -> Self {
        Self {
            repo,
            jwt_secret: Arc::new(jwt_secret),
            jwt_expires_secs,
        }
    }

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
        };

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
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|err| AppError::internal(format!("生成令牌失败: {err}")))?;
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
}

impl AuthService {
    async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
    ) {
        let _ = self
            .repo
            .append_login_log(LoginAuditRecord {
                username: username.map(ToString::to_string),
                login_type,
                status,
                message: message.to_string(),
                ip: ip.to_string(),
            })
            .await;
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
