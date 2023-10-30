use std::io::{stdin, stdout, Write};

pub fn get_user_input() -> String {
    print!("> ");
    stdout().flush().unwrap();
    let mut user_text = String::new();
    stdin().read_line(&mut user_text).expect("Failed to read line");
    user_text
}