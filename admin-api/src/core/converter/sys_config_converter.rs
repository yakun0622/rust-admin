use crate::core::{
    dto::sys_config_dto::{SysConfigCreateReqDto, SysConfigUpdateReqDto},
    errors::AppError,
    model::sys_config::SysConfigModel,
    vo::sys_config_vo::SysConfigVo,
};

pub fn to_sys_config_vo(model: SysConfigModel) -> SysConfigVo {
    SysConfigVo {
        id: model.id,
        name: model.config_key,
        value: model.config_value,
        remark: model.remark.unwrap_or_default(),
        status: if model.status == 1 {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
    }
}

pub fn from_create_dto(dto: SysConfigCreateReqDto) -> Result<SysConfigModel, AppError> {
    let name = normalize_required_text("参数名称", dto.name)?;
    Ok(SysConfigModel {
        id: 0,
        config_name: name.clone(),
        config_key: name,
        config_value: normalize_required_text("参数值", dto.value)?,
        remark: normalize_optional_text(dto.remark),
        status: normalize_status(dto.status.as_deref())?,
    })
}

pub fn from_update_dto(id: u64, dto: SysConfigUpdateReqDto) -> Result<SysConfigModel, AppError> {
    let name = normalize_required_text("参数名称", dto.name)?;
    Ok(SysConfigModel {
        id,
        config_name: name.clone(),
        config_key: name,
        config_value: normalize_required_text("参数值", dto.value)?,
        remark: normalize_optional_text(dto.remark),
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
