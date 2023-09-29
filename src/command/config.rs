use inquire::Select;

use crate::config::Platform;

pub struct ConfigCommand {}

impl ConfigCommand {
    pub fn handle() -> Result<(), String> {
        let options = vec![Platform::Codeforces, Platform::Atcoder];
        let ans = Select::new("Select an option", options).prompt();
        match ans {
            Ok(ans) => println!("{}", ans),
            Err(_) => {
                return Err("Error when choosing option".to_string());
            }
        }
        return Ok(());
    }
}
