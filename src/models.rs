
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
pub struct DataResult {
    pub prompt: String,
    pub response: String,
}

#[derive(Clone)]
pub struct AppState {
    pub chatgpt_api_key: String,
    pub gemini_api_key:String,
}

impl AppState {
    // Initializes AppState with a database connection and an API key
    pub fn new(chatgpt_api_key: String,gemini_api_key:String) -> Self {
        AppState {
            chatgpt_api_key,
            gemini_api_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIRequest<'a> {
    pub model: &'a str,
    pub prompt: Vec<String>,
    pub max_tokens: u16,
}


#[derive(Deserialize)]
pub struct BatchRequest {
    pub prompts: Vec<String>,
}

#[derive(Deserialize)]
pub struct DataInputGenerateAI {
    pub option: String,
    pub prompt: String,
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}
#[derive(Deserialize)]
pub struct Choice {
    pub message: MessageContent,
}

#[derive(Deserialize)]
pub struct MessageContent {
    pub content: String,
}
