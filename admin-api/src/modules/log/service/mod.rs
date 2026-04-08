pub mod integration;

use std::sync::Arc;

use crate::{
    core::{
        errors::AppError,
        vo::log::{LoginLogItemVo, LoginLogListVo, OperLogItemVo, OperLogListVo},
    },
    modules::log::repository::LogRepository,
};

#[derive(Clone)]
pub struct LogService {
    repo: Arc<dyn LogRepository>,
}

impl LogService {
    pub fn new(repo: Arc<dyn LogRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_oper(&self, keyword: Option<&str>) -> Result<OperLogListVo, AppError> {
        let items = self
            .repo
            .list_oper(keyword)
            .await?
            .into_iter()
            .map(|item| OperLogItemVo {
                id: item.id,
                module: item.module,
                business_type: item.business_type,
                request_method: item.request_method,
                oper_name: item.oper_name,
                ip: item.ip,
                status: item.status,
                duration_ms: item.duration_ms,
                oper_at: item.oper_at,
            })
            .collect::<Vec<_>>();

        Ok(OperLogListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn list_login(&self, keyword: Option<&str>) -> Result<LoginLogListVo, AppError> {
        let items = self
            .repo
            .list_login(keyword)
            .await?
            .into_iter()
            .map(|item| LoginLogItemVo {
                id: item.id,
                username: item.username,
                login_type: item.login_type,
                ip: item.ip,
                status: item.status,
                message: item.message,
                login_at: item.login_at,
            })
            .collect::<Vec<_>>();

        Ok(LoginLogListVo {
            total: items.len(),
            items,
        })
    }
}
