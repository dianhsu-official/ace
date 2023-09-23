use crate::misc::http_client::HttpClient;
mod config;
use super::lib::OnlineJudge;
pub struct AtCoder {
    pub client: HttpClient,
    pub host: String,
}
impl OnlineJudge for AtCoder {
    fn submit(&mut self, identifier: &str, code: &str, lang_id: &str) -> Result<String, String> {
        let _ = identifier;
        let _ = code;
        let _ = lang_id;
        todo!()
    }

    fn is_login(&mut self) -> Result<String, String> {
        todo!()
    }

    fn login(&mut self, username: &str, password: &str) -> Result<String, String> {
        let _ = username;
        let _ = password;
        todo!()
    }

    fn get_test_cases(&mut self, identifier: &str) -> Result<Vec<[String; 2]>, String> {
        let _ = identifier;
        todo!()
    }

    fn retrive_result(
        &mut self,
        identifier: &str,
        submit_id: &str,
    ) -> Result<super::lib::SubmissionInfo, String> {
        let _ = identifier;
        let _ = submit_id;
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
