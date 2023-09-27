#[derive(Debug)]
pub enum Verdict {
    Waiting = 0,  // Waiting for judge or judge in progress
    Resulted = 1, // Judge finished
}
impl Verdict {
    #[allow(unused)]
    fn new() -> Verdict {
        Verdict::Waiting
    }
}
#[derive(Debug)]
pub struct SubmissionInfo {
    pub submission_id: String,
    pub identifier: String,
    pub verdict: Verdict,
    pub verdict_info: String,
    pub execute_time: String,
    pub execute_memory: String,
}
impl SubmissionInfo {
    pub fn new() -> SubmissionInfo {
        SubmissionInfo {
            submission_id: String::new(),
            identifier: String::new(),
            verdict: Verdict::new(),
            verdict_info: String::new(),
            execute_time: String::new(),
            execute_memory: String::new(),
        }
    }
}
pub trait OnlineJudge {
    fn submit(
        &mut self,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String>;
    fn is_login(&mut self) -> Result<String, String>;
    fn login(&mut self, username: &str, password: &str) -> Result<String, String>;
    fn get_problems(&mut self, contest_identifier: &str) -> Result<Vec<String>, String>;
    fn get_test_cases(&mut self, problem_identifier: &str) -> Result<Vec<[String; 2]>, String>;
    fn retrive_result(
        &mut self,
        identifier: &str,
        submit_id: &str,
    ) -> Result<SubmissionInfo, String>;
}
