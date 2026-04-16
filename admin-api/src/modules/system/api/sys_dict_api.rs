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
    crate::permission!(state, current_user, "system:dict:view");
    let service = state.dict_service();
    let data = service.list(query).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysDictCreateReqDto>,
) -> Result<Json<ApiResponse<SysDictRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:dict:create");
    let service = state.dict_service();
    let item = crate::admin_log!(state, current_user, "创建字典", 1_i8, async move {
        service.create(payload).await
    })?;
    Ok(Json(ApiResponse::success(SysDictRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysDictUpdateReqDto>,
) -> Result<Json<ApiResponse<SysDictRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:dict:update");
    let service = state.dict_service();
    let item = crate::admin_log!(state, current_user, "修改字典", 2_i8, async move {
        service.update_by_id(id, payload).await
    })?;
    Ok(Json(ApiResponse::success(SysDictRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysDictDeleteVo>>, AppError> {
    crate::permission!(state, current_user, "system:dict:delete");
    let service = state.dict_service();
    let deleted = crate::admin_log!(state, current_user, "删除字典", 3_i8, async move {
        service.delete_by_id(id).await
    })?;
    Ok(Json(ApiResponse::success(SysDictDeleteVo { id, deleted })))
}
