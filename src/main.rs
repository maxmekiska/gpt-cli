#[macro_use]
extern crate log;
extern crate env_logger;


mod oai;
mod interact;

use std::env;
use spinners::{Spinner, Spinners};
use clap::{Parser, Subcommand};


use oai::{create_oai_request, send_request, OAIMessage};
use interact::{get_user_input, special_commands};


async fn run_chat(max_tokens: Option<u32>, model: Option<String>, temperature: Option<f32>, logging: Option<bool>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

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
                spin.stop_with_symbol("<\x1b[32m>\x1b[0m");
                let ai_response = json.choices[0].message.content.clone();
                println!("{}", ai_response);

                let ai_message = OAIMessage {
                    role: String::from("assistant"),
                    content: ai_response.clone(),
                };
                chat_history.push(ai_message);
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
    }

    Ok(())
}


async fn one_shot(user_input: String, max_tokens: Option<u32>, model: Option<String>, temperature: Option<f32>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    let oai_request = create_oai_request(&chat_history, max_tokens, model, temperature);

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

Ok(())
}




#[derive(Parser)]
#[command(name = "gpt-cli")]
#[command(author = "Max Mekiska. <maxmekiska@gmail.com>")]
#[command(version = "0.2.0")]
#[command(about = "CLI to interact with OpenAi LLMs.", long_about = None)]
struct Args {

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {

    OneShot {
        
        #[arg(short, long)]
        prompt: String,

        #[arg(short, long, default_value_t = 300)]
        max_tokens: u32,

        #[arg(short, long, default_value_t = String::from("gpt-3.5-turbo"))]
        gpt_model: String,

        #[arg(short, long, default_value_t = 0.2)]
        temperature: f32,

    },

    Chat {
        
        #[arg(short, long, default_value_t = 300)]
        max_tokens: u32,

        #[arg(short, long, default_value_t = String::from("gpt-3.5-turbo"))]
        gpt_model: String,

        #[arg(short, long, default_value_t = 0.2)]
        temperature: f32,

        #[arg(short, long, action)]
        logging: bool,

    },
    
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let args = Args::parse();

    match &args.command {

        Some(Commands::OneShot { prompt, max_tokens, gpt_model, temperature }) => {
            one_shot(prompt.to_string(), Some(*max_tokens), Some(gpt_model.to_string()), Some(*temperature)).await
        },

        Some(Commands::Chat { max_tokens, gpt_model, temperature, logging }) => {
            run_chat(Some(*max_tokens), Some(gpt_model.to_string()), Some(*temperature), Some(*logging)).await
        }
        None => Ok({})
    }
}