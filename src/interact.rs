use std::io::{stdin, stdout, Write};

use crate::oai::OAIMessage;

pub fn get_user_input() -> String {
    print!("> ");
    stdout().flush().unwrap();
    let mut user_text = String::new();
    stdin().read_line(&mut user_text).expect("Failed to read line");
    user_text
}

pub fn special_commands(user_input: &str, chat_history: &mut Vec<OAIMessage>) -> u8 {
    match user_input.trim().to_lowercase().as_str() {
        "exit" => {
            println!("Exiting the program.");
            return 1;
        }
        "clear" => {
            println!("Chat history cleared.");
            chat_history.clear();
            return 2;
        }
        "undo" if chat_history.len() >= 2 => {
            chat_history.pop();
            chat_history.pop();
            println!("Undone last user input and assistant response.");
            return 2;
        }
        _ => {
            return 0;
        }
    }
}