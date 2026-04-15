use crate::core::{
    errors::AppError,
    vo::log_vo::{LoginLogItemVo, LoginLogListVo, OperLogItemVo, OperLogListVo},
};
use crate::modules::system::repository::SysLogRepository;

#[derive(Clone)]
pub struct SysLogService {
    repo: SysLogRepository,
}

impl SysLogService {
    pub(crate) fn new(repo: SysLogRepository) -> Self {
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
