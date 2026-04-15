use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginReqDto {
    pub username: String,
    pub password: String,
}
