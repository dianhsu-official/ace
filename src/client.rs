use crate::{
    codeforces::{self, Codeforces},
    tool::Tool,
};

pub struct Client<T> {
    pub platform: T,
}

impl<T> Client<T> {
    pub fn new(platform: T) -> Client<T> {
        Client { platform: platform }
    }
}
impl Client<Codeforces> {
    pub fn account_management(&mut self) {
        println!("Choose your operation:");
        println!("0) List accounts");
        println!("1) Add account");
        println!("2) Remove account");
        let idx = Tool::choose_index(3);
        match idx {
            0 => {
                for idx in 0..self.platform.accounts.len() {
                    println!("{}: {}", idx, self.platform.accounts[idx].handle)
                }
            }
            1 => {
                let mut account = codeforces::Account::new();
                account.handle =
                    Tool::get_input("Please input your handle name or email address: ");
                account.password = Tool::get_password_input("Please input your password: ");
                self.platform.accounts.push(account);
            }
            2 => {}
            _ => {}
        }
    }
}
