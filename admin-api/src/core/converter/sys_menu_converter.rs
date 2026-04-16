use crate::core::{
    dto::sys_menu_dto::{SysMenuCreateReqDto, SysMenuUpdateReqDto},
    errors::AppError,
    model::sys_menu::SysMenuModel,
    vo::sys_menu_vo::SysMenuVo,
};

pub fn to_sys_menu_vo(model: SysMenuModel) -> SysMenuVo {
    let permission = model
        .permission
        .clone()
        .or(model.perms.clone())
        .unwrap_or_default();

    SysMenuVo {
        id: model.id,
        parent_id: model.parent_id,
        menu_type: model.menu_type,
        name: model.menu_name,
        route_name: model.route_name.unwrap_or_default(),
        path: model.route_path.unwrap_or_default(),
        component: model.component_path.unwrap_or_default(),
        permission,
        icon: model.icon.unwrap_or_default(),
        order_num: model.order_num,
        status: if model.status == 1 {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
        visible: if model.is_visible == 1 {
            "yes".to_string()
        } else {
            "no".to_string()
        },
        children: Vec::new(),
    }
}

pub fn from_create_dto(dto: SysMenuCreateReqDto) -> Result<SysMenuModel, AppError> {
    let menu_type = normalize_menu_type(dto.menu_type)?;
    let menu_name = normalize_required_text("菜单名称", dto.name)?;
    let route_name = normalize_optional_text(dto.route_name);
    let route_path = normalize_optional_text(dto.path);
    let component_path = normalize_optional_text(dto.component);
    let permission = normalize_optional_text(dto.permission);
    let icon = normalize_optional_text(dto.icon);
    let order_num = normalize_order_num(dto.order_num)?;
    let is_visible = normalize_visible_status(dto.visible.as_deref())?;
    let status = normalize_status(dto.status.as_deref(), 1)?;

    validate_by_menu_type(
        menu_type,
        route_path.as_deref(),
        component_path.as_deref(),
        permission.as_deref(),
    )?;

    Ok(SysMenuModel {
        id: 0,
        parent_id: dto.parent_id,
        menu_type,
        menu_name,
        route_name,
        route_path,
        component_path,
        perms: permission.clone(),
        permission,
        icon,
        order_num,
        is_visible,
        status,
    })
}

pub fn from_update_dto(
    current: SysMenuModel,
    dto: SysMenuUpdateReqDto,
) -> Result<SysMenuModel, AppError> {
    let menu_type = normalize_menu_type(dto.menu_type.or(Some(current.menu_type)))?;
    let menu_name = match dto.name {
        Some(value) => normalize_required_text("菜单名称", value)?,
        None => current.menu_name,
    };
    let parent_id = dto.parent_id.unwrap_or(current.parent_id);
    let route_name = match dto.route_name {
        Some(value) => normalize_optional_text(Some(value)),
        None => current.route_name,
    };
    let route_path = match dto.path {
        Some(value) => normalize_optional_text(Some(value)),
        None => current.route_path,
    };
    let component_path = match dto.component {
        Some(value) => normalize_optional_text(Some(value)),
        None => current.component_path,
    };
    let permission = match dto.permission {
        Some(value) => normalize_optional_text(Some(value)),
        None => current.permission.or(current.perms),
    };
    let icon = match dto.icon {
        Some(value) => normalize_optional_text(Some(value)),
        None => current.icon,
    };
    let order_num = normalize_order_num(dto.order_num.or(Some(current.order_num)))?;
    let is_visible =
        normalize_visible_status_with_default(dto.visible.as_deref(), current.is_visible)?;
    let status = normalize_status(dto.status.as_deref(), current.status)?;

    validate_by_menu_type(
        menu_type,
        route_path.as_deref(),
        component_path.as_deref(),
        permission.as_deref(),
    )?;

    Ok(SysMenuModel {
        id: current.id,
        parent_id,
        menu_type,
        menu_name,
        route_name,
        route_path,
        component_path,
        perms: permission.clone(),
        permission,
        icon,
        order_num,
        is_visible,
        status,
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

fn normalize_menu_type(raw: Option<i16>) -> Result<i16, AppError> {
    let menu_type = raw.unwrap_or(2);
    if matches!(menu_type, 1 | 2 | 3) {
        Ok(menu_type)
    } else {
        Err(AppError::bad_request(
            "菜单类型非法，仅支持 1目录/2菜单/3按钮",
        ))
    }
}

fn normalize_order_num(raw: Option<i32>) -> Result<i32, AppError> {
    let order_num = raw.unwrap_or(0);
    if order_num < 0 {
        return Err(AppError::bad_request("排序值不能小于0"));
    }
    Ok(order_num)
}

fn normalize_visible_status(raw: Option<&str>) -> Result<i16, AppError> {
    normalize_visible_status_with_default(raw, 1)
}

fn normalize_visible_status_with_default(raw: Option<&str>, default: i16) -> Result<i16, AppError> {
    let normalized = raw
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(match default {
            0 => "no",
            _ => "yes",
        });
    match normalized {
        "yes" | "1" => Ok(1),
        "no" | "0" => Ok(0),
        _ => Err(AppError::bad_request("可见值非法，仅支持 yes/no/1/0")),
    }
}

fn normalize_status(raw: Option<&str>, default: i16) -> Result<i16, AppError> {
    let normalized = raw
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(match default {
            0 => "disabled",
            _ => "enabled",
        });

    match normalized {
        "enabled" | "1" => Ok(1),
        "disabled" | "0" => Ok(0),
        _ => Err(AppError::bad_request(
            "状态值非法，仅支持 enabled/disabled/1/0",
        )),
    }
}

fn validate_by_menu_type(
    menu_type: i16,
    route_path: Option<&str>,
    component_path: Option<&str>,
    permission: Option<&str>,
) -> Result<(), AppError> {
    if menu_type == 2 {
        if route_path.is_none() {
            return Err(AppError::bad_request(
                "菜单类型为“菜单”时，路由地址不能为空",
            ));
        }
        if component_path.is_none() {
            return Err(AppError::bad_request(
                "菜单类型为“菜单”时，组件路径不能为空",
            ));
        }
    }

    if menu_type == 3 && permission.is_none() {
        return Err(AppError::bad_request(
            "菜单类型为“按钮”时，permission不能为空",
        ));
    }

    Ok(())
}
