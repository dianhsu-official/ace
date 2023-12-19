use crate::model::{Contest, PlatformLanguage, PostSubmissionInfo, TestCase};

#[async_trait::async_trait]
pub trait OnlineJudge {
    async fn submit(
        &mut self,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String>;
    async fn is_login(&mut self) -> bool;
    async fn login(&mut self) -> Result<String, String>;
    /// Get all problems in a contest, return a vector of problem identifier and problem url  
    async fn get_problems(&mut self, contest_identifier: &str) -> Result<Vec<[String; 2]>, String>;
    async fn get_test_cases(&mut self, problem_url: &str) -> Result<Vec<TestCase>, String>;
    async fn retrive_result(
        &mut self,
        problem_identifier: &str,
        submission_id: &str,
    ) -> Result<PostSubmissionInfo, String>;
    async fn get_contest(&mut self, contest_identifier: &str) -> Result<Contest, String>;
    fn save_cookies(&mut self) -> Result<(), String>;
    fn get_platform_languages() -> Vec<PlatformLanguage>;
}
