use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Clone, Debug)]
pub struct OAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct OAIResMessage {
    role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct OAIChoices {
    pub message: OAIResMessage,
    index: u8,
    logprobs: Option<u8>,
    finnish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    create: Option<u64>,
    model: Option<String>,
    pub choices: Vec<OAIChoices>,
}

#[derive(Serialize, Debug)]
pub struct OAIRequest {
    pub messages: Vec<OAIMessage>,
    pub max_tokens: u32,
    pub model: String,
    pub temperature: f32,
}


impl Default for OAIRequest {
    fn default() -> Self {
        OAIRequest {
            messages: Vec::new(),
            max_tokens: 300,
            model: String::from("gpt-3.5-turbo"),
            temperature: 0.2,
        }
    }
}

pub fn create_oai_request(chat_history: &Vec<OAIMessage>, max_tokens: Option<u32>, model: Option<String>, temperature: Option<f32>) -> OAIRequest {
    OAIRequest {
        messages: chat_history.clone(),
        max_tokens: max_tokens.unwrap_or_default(),
        model: model.unwrap_or_default(),
        temperature: temperature.unwrap_or_default(),
    }
}

pub async fn send_request(uri: &str, auth_header_val: &str, oai_request: &OAIRequest) -> Result<OAIResponse, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let body = Body::from(serde_json::to_vec(oai_request)?);
    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", auth_header_val)
        .body(body)
        .unwrap();
    let res = client.request(req).await?;
    let body = hyper::body::aggregate(res).await?;
    let json: OAIResponse = serde_json::from_reader(body.reader())?;
    Ok(json)
}