pub enum SubmitResult {
    Waiting = 0,  // Waiting for judge or judge in progress
    Resulted = 1, // Judge finished
    Error = 2,    // Judge error or network error
}
impl SubmitResult {
    #[allow(unused)]
    fn new() -> SubmitResult {
        SubmitResult::Waiting
    }
}
pub struct SubmitInfo {
    pub submit_id: String,
    pub result: SubmitResult,
    pub result_info: String,
    pub time: String,
    pub memory: String,
}

pub trait OnlineJudge {
    fn submit(&mut self, identifier: &str, code: &str, lang_id: &str) -> Result<String, String>;
    fn is_login(&mut self) -> Result<String, String>;
    fn login(&mut self, username: &str, password: &str) -> Result<String, String>;
    fn get_test_cases(&mut self, identifier: &str) -> Result<Vec<[String; 2]>, String>;
    fn retrive_result(&mut self, contest_id: &str, submit_id: &str) -> Result<(SubmitResult, SubmitInfo), String>;
}
