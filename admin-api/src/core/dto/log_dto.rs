use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LogListQueryDto {
    pub keyword: Option<String>,
}
