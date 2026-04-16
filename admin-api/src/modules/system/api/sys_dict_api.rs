use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_dict_dto::{SysDictCreateReqDto, SysDictListQueryDto, SysDictUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_dict_vo::{SysDictDeleteVo, SysDictListVo, SysDictRecordVo},
    },
    middleware::auth::ensure_permission,
};

pub struct SysDictRouter;

impl SysDictRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/dict", get(list).post(create))
            .route("/dict/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysDictListQueryDto>,
) -> Result<Json<ApiResponse<SysDictListVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:dict:view").await?;
    let service = state.dict_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysDictCreateReqDto>,
) -> Result<Json<ApiResponse<SysDictRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:dict:create").await?;
    let service = state.dict_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysDictRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysDictUpdateReqDto>,
) -> Result<Json<ApiResponse<SysDictRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:dict:update").await?;
    let service = state.dict_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysDictRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysDictDeleteVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:dict:delete").await?;
    let service = state.dict_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysDictDeleteVo { id, deleted })))
}
