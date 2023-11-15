use log::{info, error};

use std::env;
use spinners::{Spinner, Spinners};

use crate::oai::{create_oai_request, send_request, OAIMessage};
use crate::interact::{get_user_input, special_commands};


pub async fn run_chat(max_tokens: Option<u32>, model: Option<String>, temperature: Option<f32>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let uri = "https://api.openai.com/v1/chat/completions";
    let oai_token: String = env::var("OPENAI_API_KEY").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);

    let mut chat_history: Vec<OAIMessage> = Vec::new();

    loop {
        let user_input = get_user_input();
        let action = special_commands(&user_input, &mut chat_history);

        if action == 1 {
            break;
        } else if action == 2 {
            continue;
        }

        let user_message = OAIMessage {
            role: String::from("user"),
            content: user_input.clone(),
        };
        chat_history.push(user_message);

        let mut spin = Spinner::new(Spinners::Pipe, String::new());
        let oai_request = create_oai_request(&chat_history, max_tokens, model.clone(), temperature);

        match send_request(uri, &auth_header_val, &oai_request).await {
            Ok(json) => {
                spin.stop_with_symbol("\x1b[96m>\x1b[0m\x1b[32m<\x1b[0m");
                let ai_response = json.choices[0].message.content.clone();
                println!("{}", ai_response);

                let ai_message = OAIMessage {
                    role: String::from("assistant"),
                    content: ai_response.clone(),
                };
                chat_history.push(ai_message);
            }
            Err(err) => {
                spin.stop_with_symbol("\x1b[96m>\x1b[0m\x1b[31m<\x1b[0m");
                eprintln!("\x1b[31mError: Please ensure to set the env var OPENAI_API_KEY with a valid API key.\x1b[0m");
                error!("Error: {:?}\n", err);
            }
        }
        info!("Chat History: {:?}\n", chat_history);
        info!("Max Tokens: {:?}\n", max_tokens);
        info!("Temperatur: {:?}\n", temperature);
        info!("GPT Model: {:?}\n", model);
    }
    Ok(())
}
