#[macro_use]
extern crate log;
extern crate env_logger;

mod oai;
mod interact;

use std::env;

use oai::{create_oai_request, send_request, OAIMessage};
use interact::{get_user_input, special_commands};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let uri = "https://api.openai.com/v1/chat/completions";
    let oai_token: String = env::var("OPENAI_API_KEY").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);

    let mut logging = false; 
    let mut chat_history: Vec<OAIMessage> = Vec::new();

    loop {

        let user_input = get_user_input();

        let action = special_commands(&user_input, &mut chat_history);

        if action == 1 {
            break;
        } else if action == 2 {
            continue;
        } else if action == 3 {
            logging = true;
            continue;
        } else if action == 4 {
            logging = false;
            continue;
        }

        let user_message = OAIMessage {
            role: String::from("user"),
            content: user_input.clone(),
        };
        chat_history.push(user_message);

        let oai_request = create_oai_request(&chat_history);


        match send_request(uri, &auth_header_val, &oai_request).await {
            Ok(json) => {
                let ai_response = json.choices[0].message.content.clone();
                println!("{}", ai_response);

                let ai_message = OAIMessage {
                    role: String::from("assistant"),
                    content: ai_response.clone(),
                };
                chat_history.push(ai_message);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
        println!{"log status: {:?}", logging}
        if logging { 
            info!("Chat History: {:?}", chat_history);
        }

    }

    Ok(())

}