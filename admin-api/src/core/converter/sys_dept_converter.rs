use crate::core::{
    dto::sys_dept_dto::{SysDeptCreateReqDto, SysDeptUpdateReqDto},
    errors::AppError,
    model::sys_dept::SysDeptModel,
    vo::sys_dept_vo::SysDeptVo,
};

pub fn to_sys_dept_vo(model: SysDeptModel) -> SysDeptVo {
    SysDeptVo {
        id: model.id,
        parent_id: model.parent_id,
        name: model.dept_name,
        leader: model.leader.unwrap_or_default(),
        phone: model.phone.unwrap_or_default(),
        status: if model.status == 1 {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
    }
}

pub fn from_create_dto(dto: SysDeptCreateReqDto) -> Result<SysDeptModel, AppError> {
    Ok(SysDeptModel {
        id: 0,
        parent_id: dto.parent_id,
        dept_name: normalize_required_text("部门名称", dto.name)?,
        leader: normalize_optional_text(dto.leader),
        phone: normalize_optional_text(dto.phone),
        status: normalize_status(dto.status.as_deref())?,
    })
}

pub fn from_update_dto(
    id: u64,
    parent_id: u64,
    dto: SysDeptUpdateReqDto,
) -> Result<SysDeptModel, AppError> {
    Ok(SysDeptModel {
        id,
        parent_id,
        dept_name: normalize_required_text("部门名称", dto.name)?,
        leader: normalize_optional_text(dto.leader),
        phone: normalize_optional_text(dto.phone),
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

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
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
