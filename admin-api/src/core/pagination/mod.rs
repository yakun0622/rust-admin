use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PageQuery {
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageResult<T>
where
    T: Serialize,
{
    pub list: Vec<T>,
    pub page_num: u32,
    pub page_size: u32,
    pub total: u64,
}
