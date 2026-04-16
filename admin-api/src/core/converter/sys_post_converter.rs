use crate::core::{
    dto::sys_post_dto::SysPostCreateReqDto, errors::AppError, model::sys_post::SysPostModel,
    vo::sys_post_vo::SysPostVo,
};

pub fn to_sys_post_vo(model: SysPostModel) -> SysPostVo {
    SysPostVo {
        id: model.id,
        name: model.post_name,
        code: model.post_code,
        sort: model.post_sort,
        status: if model.status == 1 {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
    }
}

pub fn from_create_dto(dto: SysPostCreateReqDto) -> Result<SysPostModel, AppError> {
    Ok(SysPostModel {
        id: 0,
        post_name: normalize_required_text("岗位名称", dto.name)?,
        post_code: normalize_required_text("岗位编码", dto.code)?,
        post_sort: dto.sort.unwrap_or(1),
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
