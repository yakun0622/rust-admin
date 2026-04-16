use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_notice_dto::{
            SysNoticeCreateReqDto, SysNoticeListQueryDto, SysNoticeUpdateReqDto,
        },
        errors::AppError,
        response::ApiResponse,
        vo::system_vo::{SystemCrudDeleteVo, SystemCrudListVo, SystemCrudRecordVo},
    },
};

pub struct SysNoticeRouter;

impl SysNoticeRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/notice", get(list).post(create))
            .route("/notice/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysNoticeListQueryDto>,
) -> Result<Json<ApiResponse<SystemCrudListVo>>, AppError> {
    crate::permission!(state, current_user, "system:notice:view");
    let service = state.notice_service();
    let data = service.list(query).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysNoticeCreateReqDto>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:notice:create");
    let service = state.notice_service();
    let item = crate::admin_log!(state, current_user, "创建公告", 1_i8, async move {
        service.create(payload).await
    })?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysNoticeUpdateReqDto>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:notice:update");
    let service = state.notice_service();
    let item = crate::admin_log!(state, current_user, "修改公告", 2_i8, async move {
        service.update_by_id(id, payload).await
    })?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SystemCrudDeleteVo>>, AppError> {
    crate::permission!(state, current_user, "system:notice:delete");
    let service = state.notice_service();
    let deleted = crate::admin_log!(state, current_user, "删除公告", 3_i8, async move {
        service.delete_by_id(id).await
    })?;
    Ok(Json(ApiResponse::success(SystemCrudDeleteVo {
        id,
        deleted,
    })))
}
