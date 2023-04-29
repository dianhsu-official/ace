use reqwest::{cookie::{Jar, CookieStore}, Url};
use rpassword::read_password;
use std::io::{self, stdout, Write};

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
        let re_prompt = format!(
            "Input index invalid. Choose index from [0, {}]: ",
            max_size - 1
        );
        let mut idx = -1;
        let mut res = Tool::get_input(&prompt);
        while idx >= max_size || idx < 0 {
            idx = match res.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    res = Tool::get_input(&re_prompt);
                    -1
                }
            }
        }
        return idx;
    }
    pub fn save_cookies(jar: Jar, url: &str) -> String {
        let cookies = jar.cookies(&url.parse::<Url>().unwrap());
        let mut res = String::new();
        for header in cookies{
            match header.to_str() {
                Ok(cookie) => {
                    res.push_str(cookie);
                },
                Err(_) => {},
            }
        };
        return res;
    }
    pub fn load_cookies(cookies: &str, url: &str) -> Jar {
        let jar = Jar::default();
        jar.add_cookie_str(cookies, &url.parse::<Url>().unwrap());
        return jar;
    }
}
