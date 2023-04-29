use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub handle: String,
    pub password: String,
    pub ftaa: String,
    pub bfaa: String,
}
#[derive(Serialize, Deserialize)]
pub struct Codeforces {
    pub host: String,
    pub accounts: Vec<Account>,
}
impl Codeforces {
    pub fn new() -> Codeforces {
        Codeforces {
            host: "https://codeforces.com".to_string(),
            accounts: Vec::new(),
        }
    }
    #[allow(unused)]
    pub fn login(&mut self) {
        unimplemented!()
    }
    #[allow(unused)]
    pub fn parse(&mut self) {
        unimplemented!()
    }
    #[allow(unused)]
    pub fn submit(&mut self) {}
}
