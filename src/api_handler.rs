use std::error::Error;

use log::info;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio;

use crate::models::{ChatRequest, ChatResponse, Message, OpenAIRequest};


pub async fn get_chat_ChatGPT_response(api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let messages = vec![Message {
        role: "user".to_string(),
        content: prompt.to_string(),
    }];

    let chat_request = ChatRequest {
        model: "gpt-4o-mini".to_string(), // Model yang Anda pilih
        messages,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&chat_request)
        .send()
        .await?;

    let chat_response: ChatResponse = response.json().await?;
    
    if let Some(choice) = chat_response.choices.first() {
        let json_chat_response = convert_to_json(choice.message.content.clone().as_str());

        Ok(choice.message.content.clone())
    } else {
        Err("No response from the API".into())
    }
}

pub async fn get_chat_gemini_response(api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=".to_owned();
    url.push_str(api_key);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let request_body = json!({
        "contents": [{
            "parts": [{
                "text": prompt,
            }]
        }]
    });

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;

    let text:String;
    let response_json: Value = response.json().await?;

    let generated_content = if let Some(candidate) = response_json["candidates"].get(0) {
        if let Some(content) = candidate["content"]["parts"].get(0) {
            if let Some(text_str) = content["text"].as_str() {
                text_str.to_string()
            } else {
                return Err("Failed to extract text".into());
            }
        } else {
            return Err("Failed to extract content".into());
        }
    } else {
        return Err("Failed to extract candidate".into());
    };

    // Akses bagian "usageMetadata" -> "totalTokenCount"
    let total_token_count = if let Some(total_token) = response_json["usageMetadata"]["totalTokenCount"].as_i64() {
        total_token
    } else {
        return Err("Failed to extract total token count".into());
    };
    let Result = format!("{}",generated_content);
     Ok((Result))
}

fn convert_to_json(input: &str) -> serde_json::Value {
    // Split input by newline and collect into a vector
    let lines: Vec<&str> = input.lines().filter(|line| !line.trim().is_empty())
                                        .collect();

    // Create JSON array from the vector
    let json_array = json!(lines);

    json_array
}