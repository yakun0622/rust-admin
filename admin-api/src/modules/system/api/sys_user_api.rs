use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_user_dto::{SysUserCreateReqDto, SysUserListQueryDto, SysUserUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_user_vo::{SysUserDeleteVo, SysUserListVo, SysUserRecordVo},
    },
};

pub struct SysUserRouter;

impl SysUserRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/user", get(list).post(create))
            .route("/user/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysUserListQueryDto>,
) -> Result<Json<ApiResponse<SysUserListVo>>, AppError> {
    crate::permission!(state, current_user, "system:user:view");
    let service = state.user_service();
    let items = service.list(query).await?;
    let total = items.len();
    Ok(Json(ApiResponse::success(SysUserListVo { total, items })))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysUserCreateReqDto>,
) -> Result<Json<ApiResponse<SysUserRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:user:create");
    let service = state.user_service();
    let item = crate::admin_log!(state, current_user, "创建用户", 1_i8, async move {
        service.create(payload).await
    })?;
    Ok(Json(ApiResponse::success(SysUserRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysUserUpdateReqDto>,
) -> Result<Json<ApiResponse<SysUserRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:user:update");
    let service = state.user_service();
    let item = crate::admin_log!(state, current_user, "修改用户", 2_i8, async move {
        service.update_by_id(id, payload).await
    })?;
    Ok(Json(ApiResponse::success(SysUserRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysUserDeleteVo>>, AppError> {
    crate::permission!(state, current_user, "system:user:delete");
    let service = state.user_service();
    let deleted = crate::admin_log!(state, current_user, "删除用户", 3_i8, async move {
        service.delete_by_id(id).await
    })?;
    Ok(Json(ApiResponse::success(SysUserDeleteVo { id, deleted })))
}
