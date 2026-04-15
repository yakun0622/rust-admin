use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysNoticePo {
    pub id: u64,
    pub title: String,
    pub notice_type: i16,
    pub status: i16,
    pub publisher: String,
}
