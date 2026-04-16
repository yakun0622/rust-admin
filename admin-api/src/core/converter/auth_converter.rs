use crate::core::{
    model::auth::{UserCredentialPo, UserProfilePo},
    vo::auth_vo::{AuthProfileUserVo, LoginVo},
};

pub fn to_login_vo(user: &UserCredentialPo, token: String, expires_in: u64) -> LoginVo {
    LoginVo {
        access_token: token,
        token_type: "Bearer",
        expires_in,
        username: user.username.clone(),
        nickname: user.nickname.clone(),
    }
}

pub fn to_auth_profile_user_vo(user: UserProfilePo) -> AuthProfileUserVo {
    AuthProfileUserVo {
        user_id: user.id,
        username: user.username,
        nickname: user.nickname,
    }
}
