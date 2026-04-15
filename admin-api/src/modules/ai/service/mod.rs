pub mod integration;

mod message_service;
mod session_service;

use std::sync::Arc;

use crate::modules::ai::repository::InMemoryAiRepository;

#[derive(Clone)]
pub struct AiService {
    repo: Arc<InMemoryAiRepository>,
}

impl AiService {
    pub fn new(repo: Arc<InMemoryAiRepository>) -> Self {
        Self { repo }
    }
}
