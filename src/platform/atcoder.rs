use crate::misc::http_client::HttpClient;
use crate::platform::lib::OnlineJudge;
pub struct AtCoder {
    pub client: HttpClient,
    pub host: String,
}
impl OnlineJudge for AtCoder {
    fn submit(&mut self) -> String {
        todo!()
    }

    fn is_login(&mut self) -> Result<String, bool> {
        todo!()
    }

    fn login(&mut self, username: &str, password: &str) -> String {
        let _ = password;
        let _ = username;
        todo!()
    }

    fn get_test_cases(&mut self) -> String {
        todo!()
    }
}
impl AtCoder {
    #[allow(unused)]
    pub fn new(cookies: &str) -> Self {
        return Self {
            client: HttpClient::new(cookies, "https://atcoder.jp"),
            host: String::from("https://atcoder.jp"),
        };
    }
}
