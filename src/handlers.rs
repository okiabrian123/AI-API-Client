use actix_web::{web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use API_AI::{api_handler::{get_chat_gemini_response, get_chat_ChatGPT_response}, models::{DataResult, AppState, DataInputGenerateAI}};

use log::info;
use serde::{Deserialize, Serialize};


pub async fn handle_generate_request(input: web::Json<DataInputGenerateAI>, data: web::Data<AppState>) -> impl Responder {
    info!("masuk");
    let prompt = input.prompt.clone();
    let option = input.option.clone();
    let response = if option == "chatgpt" {
        ChatGPT(prompt, data).await
    } else if option == "gemini" {
        Gemini(prompt, data).await
    } else {
        HttpResponse::BadRequest().body("Invalid option")
    };

    response
}


async fn ChatGPT(prompt:String,data: web::Data<AppState>) -> HttpResponse{
            info!("masuk chatgpt");
            match get_chat_ChatGPT_response(&data.chatgpt_api_key, &prompt).await {
                Ok(response) => {
                    let dataResult = DataResult {
                        prompt: prompt.clone(),
                        response: response.clone(),
                    };
 
                    HttpResponse::Ok().body(serde_json::to_string(&dataResult).unwrap())
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }    
}
async fn Gemini(prompt:String,data: web::Data<AppState>) -> HttpResponse{
    info!("masuk Gemini");
    match get_chat_gemini_response(&data.gemini_api_key, &prompt).await {
        Ok(response) => {
            let dataResult = DataResult {
                prompt: prompt.clone(),
                response: response.clone(),
            };

            HttpResponse::Ok().body(serde_json::to_string(&dataResult).unwrap())
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }    
}