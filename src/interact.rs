use std::io::{stdin, stdout, Write};

use crate::oai::OAIMessage;


pub fn get_user_input() -> String {
    print!("\x1b[96m>\x1b[0m ");
    stdout().flush().unwrap();
    let mut user_text = String::new();
    stdin().read_line(&mut user_text).expect("Failed to read line");
    return user_text
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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_commands_exit() {
        let mut chat_history = Vec::new();
        assert_eq!(special_commands("exit", &mut chat_history), 1);
    }

    #[test]
    fn test_special_commands_clear() {
        let mut chat_history = vec![OAIMessage {
            role: String::from("User"),
            content: String::from("Hello!"),
        }];
        assert_eq!(special_commands("clear", &mut chat_history), 2);
        assert_eq!(chat_history.len(), 0);
    }

    #[test]
    fn test_special_commands_undo() {
        let mut chat_history = vec![
            OAIMessage {
                role: String::from("User"),
                content: String::from("Hello!"),
            },
            OAIMessage {
                role: String::from("Assistant"),
                content: String::from("Hi there!"),
            },
        ];
        assert_eq!(special_commands("undo", &mut chat_history), 2);
        assert_eq!(chat_history.len(), 0);
    }

    #[test]
    fn test_special_commands_default() {
        let mut chat_history = Vec::new();
        assert_eq!(special_commands("other", &mut chat_history), 0);
    }
}