
pub trait OnlineJudge {
    fn submit(&mut self)  -> String;
    fn is_login(&mut self) -> Result<String, bool>;
    fn login(&mut self, username: &str, password: &str) -> String;
    fn get_test_cases(&mut self) -> String;
}