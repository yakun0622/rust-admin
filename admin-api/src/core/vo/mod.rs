use serde::Serialize;

pub mod ai;
pub mod auth;
pub mod dashboard;
pub mod log;
pub mod monitor;
pub mod system;

#[derive(Debug, Clone, Serialize)]
pub struct SimpleMessageVo {
    pub module: &'static str,
    pub message: &'static str,
}
