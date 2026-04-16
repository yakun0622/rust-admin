use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    errors::AppError,
    model::log::OperLogCreatePo,
    utils::ip_util,
    vo::log_vo::{LoginLogItemVo, LoginLogListVo, OperLogItemVo, OperLogListVo},
};
use crate::modules::system::repository::ISysLogRepository;

#[async_trait]
pub trait ISysLogService: Interface {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<OperLogListVo, AppError>;
    async fn list_login(&self, keyword: Option<&str>) -> Result<LoginLogListVo, AppError>;
    async fn append_oper_log(&self, input: OperLogCreatePo) -> Result<(), AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysLogService)]
pub struct SysLogService {
    #[shaku(inject)]
    repo: Arc<dyn ISysLogRepository>,
}

impl SysLogService {
    pub async fn append_oper_log(&self, mut input: OperLogCreatePo) -> Result<(), AppError> {
        if (input.location.is_none()
            || input
                .location
                .as_ref()
                .map(|s| s.is_empty())
                .unwrap_or(true))
            && input.ip.is_some()
        {
            if let Some(ref ip) = input.ip {
                if let Some(info) = ip_util::get_ip_location(ip).await {
                    input.location = Some(ip_util::format_location(&info));
                }
            }
        }
        self.repo.append_oper(input).await
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
                location: item.location,
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
                location: item.location,
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

#[async_trait]
impl ISysLogService for SysLogService {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<OperLogListVo, AppError> {
        self.list_oper(keyword).await
    }

    async fn list_login(&self, keyword: Option<&str>) -> Result<LoginLogListVo, AppError> {
        self.list_login(keyword).await
    }

    async fn append_oper_log(&self, input: OperLogCreatePo) -> Result<(), AppError> {
        self.append_oper_log(input).await
    }
}
