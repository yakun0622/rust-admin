use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_config_dto::{
            SysConfigCreateReqDto, SysConfigListQueryDto, SysConfigUpdateReqDto,
        },
        errors::AppError,
        response::ApiResponse,
        vo::sys_config_vo::{SysConfigDeleteVo, SysConfigListVo, SysConfigRecordVo},
    },
};

pub struct SysConfigRouter;

impl SysConfigRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/config", get(list).post(create))
            .route("/config/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysConfigListQueryDto>,
) -> Result<Json<ApiResponse<SysConfigListVo>>, AppError> {
    crate::permission!(state, current_user, "system:config:view");
    let service = state.config_service();
    let data = service.list(query).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysConfigCreateReqDto>,
) -> Result<Json<ApiResponse<SysConfigRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:config:create");
    let service = state.config_service();
    let item = crate::admin_log!(state, current_user, "创建参数配置", 1_i8, async move {
        service.create(payload).await
    })?;
    Ok(Json(ApiResponse::success(SysConfigRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysConfigUpdateReqDto>,
) -> Result<Json<ApiResponse<SysConfigRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:config:update");
    let service = state.config_service();
    let item = crate::admin_log!(state, current_user, "修改参数配置", 2_i8, async move {
        service.update_by_id(id, payload).await
    })?;
    Ok(Json(ApiResponse::success(SysConfigRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysConfigDeleteVo>>, AppError> {
    crate::permission!(state, current_user, "system:config:delete");
    let service = state.config_service();
    let deleted = crate::admin_log!(state, current_user, "删除参数配置", 3_i8, async move {
        service.delete_by_id(id).await
    })?;
    Ok(Json(ApiResponse::success(SysConfigDeleteVo {
        id,
        deleted,
    })))
}
