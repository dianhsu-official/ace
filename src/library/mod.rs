use crate::model::{Contest, SubmissionInfo};
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
        problem_identifier: &str,
        submission_id: &str,
    ) -> Result<SubmissionInfo, String>;
    fn get_contest(&mut self, contest_identifier: &str) -> Result<Contest, String>;
}
