use log::{info, error};
use env_logger;

use std::env;
use spinners::{Spinner, Spinners};


use crate::oai::{create_oai_request, send_request, OAIMessage};


pub async fn one_shot(user_input: String, max_tokens: Option<u32>, model: Option<String>, temperature: Option<f32>, logging: Option<bool>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let uri = "https://api.openai.com/v1/chat/completions";
    let oai_token: String = env::var("OPENAI_API_KEY").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);


    let mut chat_history: Vec<OAIMessage> = Vec::new();


    let user_message = OAIMessage {
        role: String::from("user"),
        content: user_input.clone(),
    };

    chat_history.push(user_message);

    let mut spin = Spinner::new(Spinners::Pipe, String::new());
    let oai_request = create_oai_request(&chat_history, max_tokens, model.clone(), temperature);

    match send_request(uri, &auth_header_val, &oai_request).await {
        Ok(json) => {
            spin.stop_with_symbol("<\x1b[32m>\x1b[0m");
            let ai_response = json.choices[0].message.content.clone();
            println!("{}", ai_response);

        }
        Err(err) => {
            spin.stop_with_symbol("<\x1b[31m>\x1b[0m");
            eprintln!("Error: {:?}", err);
        }
    }
    if logging == Some(true) {
        info!("Chat History: {:?}\n", chat_history);
        info!("Max Tokens: {:?}\n", max_tokens);
        info!("Temperatur: {:?}\n", temperature);
        info!("GPT Model: {:?}\n", model);
    }

Ok(())
}