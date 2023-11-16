use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Clone, Debug, PartialEq)]
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

    if !res.status().is_success() {
        return Err(format!("OpenAI Request failed with status: {}", res.status()).into());
    }

    let body = hyper::body::aggregate(res).await?;
    let json: OAIResponse = serde_json::from_reader(body.reader())?;
    Ok(json)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oai_message_creation() {
        let oai_message = OAIMessage {
            role: "test_role".to_string(),
            content: "test_content".to_string(),
        };

        assert_eq!(oai_message.role, "test_role");
        assert_eq!(oai_message.content, "test_content");
    }

    #[test]
    fn test_oai_message_debug_format() {
        let oai_message = OAIMessage {
            role: "test_role".to_string(),
            content: "test_content".to_string(),
        };

        assert_eq!(
            format!("{:?}", oai_message),
            "OAIMessage { role: \"test_role\", content: \"test_content\" }"
        );
    }

    #[test]
    fn test_oai_res_message_deserialization() {
        let json_data = r#"{ "role": "test_role", "content": "test_content" }"#;

        let oai_res_message: OAIResMessage = serde_json::from_str(json_data).unwrap();

        assert_eq!(oai_res_message.role, "test_role");
        assert_eq!(oai_res_message.content, "test_content");
    }

    #[test]
    fn test_oai_res_message_debug_format() {
        let oai_res_message = OAIResMessage {
            role: "test_role".to_string(),
            content: "test_content".to_string(),
        };

        assert_eq!(
            format!("{:?}", oai_res_message),
            "OAIResMessage { role: \"test_role\", content: \"test_content\" }"
        );
    }

    #[test]
    fn test_oai_choices_deserialization() {
        let json_data = r#"
            {
                "message": { "role": "test_role", "content": "test_content" },
                "index": 1,
                "logprobs": 42,
                "finnish_reason": "test_reason"
            }"#;

        let oai_choices: OAIChoices = serde_json::from_str(json_data).unwrap();

        assert_eq!(oai_choices.message.role, "test_role");
        assert_eq!(oai_choices.index, 1);
        assert_eq!(oai_choices.logprobs, Some(42));
        assert_eq!(oai_choices.finnish_reason, Some("test_reason".to_string()));
    }

    #[test]
    fn test_oai_response_deserialization() {
        let json_data = r#"
            {
                "id": "test_id",
                "object": "test_object",
                "create": 123,
                "model": "test_model",
                "choices": [
                    {
                        "message": { "role": "test_role", "content": "test_content" },
                        "index": 1,
                        "logprobs": 42,
                        "finnish_reason": "test_reason"
                    }
                ]
            }"#;

        let oai_response: OAIResponse = serde_json::from_str(json_data).unwrap();

        assert_eq!(oai_response.id, Some("test_id".to_string()));
        assert_eq!(oai_response.object, Some("test_object".to_string()));
        assert_eq!(oai_response.create, Some(123));
        assert_eq!(oai_response.model, Some("test_model".to_string()));
        assert_eq!(oai_response.choices.len(), 1);
        assert_eq!(oai_response.choices[0].message.role, "test_role");
    }

    #[test]
    fn test_oai_request_serialization() {
        let oai_request = OAIRequest {
            messages: vec![
                OAIMessage {
                    role: "test_role".to_string(),
                    content: "test_content".to_string(),
                },
            ],
            max_tokens: 42,
            model: "test_model".to_string(),
            temperature: 0.8,
        };

        let json_data = serde_json::to_string(&oai_request).unwrap();

        assert_eq!(
            json_data,
            r#"{"messages":[{"role":"test_role","content":"test_content"}],"max_tokens":42,"model":"test_model","temperature":0.8}"#
        );
    }

    #[test]
    fn test_create_oai_request() {
        let chat_history = vec![
            OAIMessage {
                role: "test_role".to_string(),
                content: "test_content".to_string(),
            },
        ];
        let max_tokens = Some(42);
        let model = Some("test_model".to_string());
        let temperature = Some(0.8);

        let result_request = create_oai_request(&chat_history, max_tokens, model, temperature);

        assert_eq!(result_request.messages, chat_history);
        assert_eq!(result_request.max_tokens, 42);
        assert_eq!(result_request.model, "test_model");
        assert_eq!(result_request.temperature, 0.8);

        let default_request = create_oai_request(&chat_history, None, None, None);

        assert_eq!(default_request.messages, chat_history);
        assert_eq!(default_request.max_tokens, 0);
        assert_eq!(default_request.model, "");
        assert_eq!(default_request.temperature, 0.0);
    }
}