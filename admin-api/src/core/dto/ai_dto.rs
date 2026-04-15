use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateAiSessionReqDto {
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendAiMessageReqDto {
    pub content: String,
}
