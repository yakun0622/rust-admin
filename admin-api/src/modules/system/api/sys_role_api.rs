use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_role_dto::{SysRoleCreateReqDto, SysRoleListQueryDto, SysRoleUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_role_vo::{SysRoleDeleteVo, SysRoleListVo, SysRoleRecordVo},
    },
};

pub struct SysRoleRouter;

impl SysRoleRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/role", get(list).post(create))
            .route("/role/{id}", put(update).delete(remove))
            .route(
                "/role/{id}/menu_ids",
                get(get_menu_ids).put(update_menu_ids),
            )
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysRoleListQueryDto>,
) -> Result<Json<ApiResponse<SysRoleListVo>>, AppError> {
    crate::permission!(state, current_user, "system:role:view");
    let service = state.role_service();
    let items = service.list(query).await?;
    let total = items.len();
    Ok(Json(ApiResponse::success(SysRoleListVo { total, items })))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysRoleCreateReqDto>,
) -> Result<Json<ApiResponse<SysRoleRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:role:create");
    let service = state.role_service();
    let item = crate::admin_log!(state, current_user, "创建角色", 1_i8, async move {
        service.create(payload).await
    })?;
    Ok(Json(ApiResponse::success(SysRoleRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysRoleUpdateReqDto>,
) -> Result<Json<ApiResponse<SysRoleRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:role:update");
    let service = state.role_service();
    let item = crate::admin_log!(state, current_user, "修改角色", 2_i8, async move {
        service.update_by_id(id, payload).await
    })?;
    Ok(Json(ApiResponse::success(SysRoleRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysRoleDeleteVo>>, AppError> {
    crate::permission!(state, current_user, "system:role:delete");
    let service = state.role_service();
    let deleted = crate::admin_log!(state, current_user, "删除角色", 3_i8, async move {
        service.delete_by_id(id).await
    })?;
    Ok(Json(ApiResponse::success(SysRoleDeleteVo { id, deleted })))
}

async fn get_menu_ids(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<Vec<u64>>>, AppError> {
    crate::permission!(state, current_user, "system:role:view");
    let service = state.role_service();
    let data = service.get_role_menu_ids(id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn update_menu_ids(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(menu_ids): Json<Vec<u64>>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    crate::permission!(state, current_user, "system:role:update");
    let service = state.role_service();
    crate::admin_log!(state, current_user, "分配角色菜单", 4_i8, async move {
        service.update_role_menus(id, menu_ids).await
    })?;
    Ok(Json(ApiResponse::success(())))
}
