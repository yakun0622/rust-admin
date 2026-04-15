use crate::core::{
    dto::sys_menu_dto::{SysMenuCreateReqDto, SysMenuUpdateReqDto},
    errors::AppError,
    model::sys_menu::SysMenuModel,
    vo::sys_menu_vo::SysMenuVo,
};

pub fn to_sys_menu_vo(model: SysMenuModel) -> SysMenuVo {
    SysMenuVo {
        id: model.id,
        parent_id: model.parent_id,
        name: model.menu_name,
        path: model.route_path.unwrap_or_default(),
        component: model.component_path.unwrap_or_default(),
        visible: if model.is_visible == 1 {
            "yes".to_string()
        } else {
            "no".to_string()
        },
    }
}

pub fn from_create_dto(dto: SysMenuCreateReqDto) -> Result<SysMenuModel, AppError> {
    Ok(SysMenuModel {
        id: 0,
        parent_id: dto.parent_id,
        menu_name: normalize_required_text("菜单名称", dto.name)?,
        route_path: Some(normalize_required_text("路由地址", dto.path)?),
        component_path: Some(normalize_required_text("组件名", dto.component)?),
        is_visible: normalize_visible_status(dto.visible.as_deref())?,
    })
}

pub fn from_update_dto(
    id: u64,
    parent_id: u64,
    dto: SysMenuUpdateReqDto,
) -> Result<SysMenuModel, AppError> {
    Ok(SysMenuModel {
        id,
        parent_id,
        menu_name: normalize_required_text("菜单名称", dto.name)?,
        route_path: Some(normalize_required_text("路由地址", dto.path)?),
        component_path: Some(normalize_required_text("组件名", dto.component)?),
        is_visible: normalize_visible_status(dto.visible.as_deref())?,
    })
}

fn normalize_required_text(field_name: &str, value: String) -> Result<String, AppError> {
    let normalized = value.trim().to_string();
    if normalized.is_empty() {
        return Err(AppError::bad_request(format!("{field_name}不能为空")));
    }
    Ok(normalized)
}

fn normalize_visible_status(raw: Option<&str>) -> Result<i16, AppError> {
    let normalized = raw
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("yes");
    match normalized {
        "yes" | "1" => Ok(1),
        "no" | "0" => Ok(0),
        _ => Err(AppError::bad_request("可见值非法，仅支持 yes/no/1/0")),
    }
}
