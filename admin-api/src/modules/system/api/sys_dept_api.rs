use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_dept_dto::{SysDeptCreateReqDto, SysDeptListQueryDto, SysDeptUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_dept_vo::{SysDeptDeleteVo, SysDeptListVo, SysDeptRecordVo},
    },
};

pub struct SysDeptRouter;

impl SysDeptRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/dept", get(list).post(create))
            .route("/dept/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysDeptListQueryDto>,
) -> Result<Json<ApiResponse<SysDeptListVo>>, AppError> {
    crate::permission!(state, current_user, "system:dept:view");
    let service = state.dept_service();
    let data = service.list(query).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysDeptCreateReqDto>,
) -> Result<Json<ApiResponse<SysDeptRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:dept:create");
    let service = state.dept_service();
    let item = crate::admin_log!(state, current_user, "创建部门", 1_i8, async move {
        service.create(payload).await
    })?;
    Ok(Json(ApiResponse::success(SysDeptRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysDeptUpdateReqDto>,
) -> Result<Json<ApiResponse<SysDeptRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:dept:update");
    let service = state.dept_service();
    let item = crate::admin_log!(state, current_user, "修改部门", 2_i8, async move {
        service.update_by_id(id, payload).await
    })?;
    Ok(Json(ApiResponse::success(SysDeptRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysDeptDeleteVo>>, AppError> {
    crate::permission!(state, current_user, "system:dept:delete");
    let service = state.dept_service();
    let deleted = crate::admin_log!(state, current_user, "删除部门", 3_i8, async move {
        service.delete_by_id(id).await
    })?;
    Ok(Json(ApiResponse::success(SysDeptDeleteVo { id, deleted })))
}
