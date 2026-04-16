use crate::core::{
    dto::sys_user_dto::SysUserCreateReqDto, model::sys_user::SysUserModel,
    vo::sys_user_vo::SysUserVo,
};

pub fn to_sys_user_vo(model: SysUserModel) -> SysUserVo {
    SysUserVo {
        id: model.id,
        username: model.username,
        nickname: model.nickname,
        phone: model.phone.unwrap_or_default(),
        status: if model.status == 1 {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
    }
}

pub fn from_create_dto(dto: SysUserCreateReqDto, password_hash: &str) -> SysUserModel {
    SysUserModel {
        id: 0,
        username: dto.username,
        nickname: dto.nickname,
        phone: dto.phone,
        status: status_text_to_value(dto.status.as_deref()),
        password_hash: password_hash.to_string(),
        created_by: 1,
        updated_by: 1,
        is_deleted: 0,
    }
}

fn status_text_to_value(value: Option<&str>) -> i16 {
    match value.map(str::trim) {
        Some("disabled") => 0,
        _ => 1,
    }
}
