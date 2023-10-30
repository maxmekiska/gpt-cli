mod oai;
mod interact;

use std::env;

use oai::{create_oai_request, send_request};
use interact::get_user_input;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let uri = "https://api.openai.com/v1/chat/completions";
    let oai_token: String = env::var("OPENAI_API_KEY").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);

    loop {
        let user_input = get_user_input();

        if user_input.trim().to_lowercase() == "exit" {
            println!("Exiting the program.");
            break;
        }

        let oai_request = create_oai_request(&user_input);
        match send_request(uri, &auth_header_val, &oai_request).await {
            Ok(json) => {
                println!("{}", json.choices[0].message.content);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
    Ok(())
}
