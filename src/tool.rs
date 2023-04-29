use std::{
    io::{self, stdout, Write},
};

use rpassword::read_password;

pub struct Tool {}
impl Tool {
    pub fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        let _ = stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unexpect message");
        return input.trim().to_string();
    }
    pub fn get_password_input(prompt: &str) -> String {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        read_password().unwrap()
    }
    pub fn choose_index(max_size: i32) -> i32 {
        let prompt = format!("Choose index from [0, {}]: ", max_size - 1);
        let mut idx = -1;
        while idx >= max_size || idx < 0 {
            let res = Tool::get_input(&prompt);
            idx = match res.parse::<i32>() {
                Ok(val) => val,
                Err(_) => -1,
            }
        }
        return idx;
    }
}
