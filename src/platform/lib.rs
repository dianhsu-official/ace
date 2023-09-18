pub trait OnlineJudge {
    fn submit(&mut self, identifier: &str, code: &str, lang_id: &str) -> Result<String, String>;
    fn is_login(&mut self) -> Result<String, bool>;
    fn login(&mut self, username: &str, password: &str) -> String;
    fn get_test_cases(&mut self, identifier: &str) -> String;
}
