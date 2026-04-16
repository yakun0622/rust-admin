use crate::core::{
    dto::sys_role_dto::SysRoleCreateReqDto, errors::AppError, model::sys_role::SysRoleModel,
    vo::sys_role_vo::SysRoleVo,
};

pub fn to_sys_role_vo(model: SysRoleModel) -> SysRoleVo {
    SysRoleVo {
        id: model.id,
        name: model.role_name,
        key: model.role_key,
        sort: model.role_sort,
        status: if model.status == 1 {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
    }
}

pub fn from_create_dto(dto: SysRoleCreateReqDto) -> Result<SysRoleModel, AppError> {
    Ok(SysRoleModel {
        id: 0,
        role_name: normalize_required_text("角色名称", dto.name)?,
        role_key: normalize_required_text("权限标识", dto.key)?,
        role_sort: dto.sort.unwrap_or(1),
        status: normalize_status(dto.status.as_deref())?,
    })
}

fn normalize_required_text(field_name: &str, value: String) -> Result<String, AppError> {
    let normalized = value.trim().to_string();
    if normalized.is_empty() {
        return Err(AppError::bad_request(format!("{field_name}不能为空")));
    }
    Ok(normalized)
}

fn normalize_status(raw: Option<&str>) -> Result<i16, AppError> {
    let normalized = raw
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("enabled");

    match normalized {
        "enabled" | "1" => Ok(1),
        "disabled" | "0" => Ok(0),
        _ => Err(AppError::bad_request(
            "状态值非法，仅支持 enabled/disabled/1/0",
        )),
    }
}
