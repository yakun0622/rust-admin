use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LoginVo {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
    pub username: String,
    pub nickname: String,
}
