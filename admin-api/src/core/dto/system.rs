use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SystemListQueryDto {
    pub keyword: Option<String>,
}
