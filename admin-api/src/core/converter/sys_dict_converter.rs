use crate::core::{
    dto::sys_dict_dto::{SysDictCreateReqDto, SysDictUpdateReqDto},
    errors::AppError,
    model::sys_dict::SysDictModel,
    vo::sys_dict_vo::SysDictVo,
};

pub fn to_sys_dict_vo(model: SysDictModel) -> SysDictVo {
    SysDictVo {
        id: model.id,
        dict_type: model.dict_type,
        label: model.label,
        value: model.value,
        status: if model.status == 1 {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
    }
}

pub fn from_create_dto(dto: SysDictCreateReqDto) -> Result<SysDictModel, AppError> {
    Ok(SysDictModel {
        id: 0,
        dict_type: normalize_required_text("字典类型", dto.dict_type)?,
        label: normalize_required_text("字典标签", dto.label)?,
        value: normalize_required_text("字典值", dto.value)?,
        status: normalize_status(dto.status.as_deref())?,
    })
}

pub fn from_update_dto(id: u64, dto: SysDictUpdateReqDto) -> Result<SysDictModel, AppError> {
    Ok(SysDictModel {
        id,
        dict_type: normalize_required_text("字典类型", dto.dict_type)?,
        label: normalize_required_text("字典标签", dto.label)?,
        value: normalize_required_text("字典值", dto.value)?,
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
