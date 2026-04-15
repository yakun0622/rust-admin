use crate::core::{model::auth::UserCredentialPo, vo::auth_vo::LoginVo};

pub fn to_login_vo(user: &UserCredentialPo, token: String, expires_in: u64) -> LoginVo {
    LoginVo {
        access_token: token,
        token_type: "Bearer",
        expires_in,
        username: user.username.clone(),
        nickname: user.nickname.clone(),
    }
}
