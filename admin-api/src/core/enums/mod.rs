use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(i8)]
pub enum CommonStatus {
    Disabled = 0,
    Enabled = 1,
}
