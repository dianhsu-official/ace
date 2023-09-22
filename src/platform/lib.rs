pub trait OnlineJudge {
    fn submit(&mut self, identifier: &str, code: &str, lang_id: &str) -> Result<String, String>;
    fn is_login(&mut self) -> Result<String, String>;
    fn login(&mut self, username: &str, password: &str) -> Result<String, String>;
    fn get_test_cases(&mut self, identifier: &str) -> Result<Vec<[String; 2]>, String>;
}
