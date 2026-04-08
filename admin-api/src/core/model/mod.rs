use serde::{Deserialize, Serialize};

pub mod ai;
pub mod auth;
pub mod dashboard;
pub mod log;
pub mod monitor;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BasePo {
    pub id: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_deleted: bool,
}
