pub struct UrlBuilder {}

impl UrlBuilder {
    /**
    Build the url for the submission.

    contest_id: The contest id of the problem.

    submit_id: The submit id of the submission.
    */
    pub fn build_submission_url(contest_id: &str, submit_id: &str) -> String {
        return format!(
            "https://codeforces.com/contest/{}/submission/{}",
            contest_id, submit_id
        );
    }

    pub fn build_problem_url(contest_id: &str, problem_id: &str) -> String {
        return format!(
            "https://codeforces.com/contest/{}/problem/{}",
            contest_id, problem_id
        );
    }
    pub fn build_login_url() -> String {
        return String::from("https://codeforces.com/enter");
    }
    pub fn build_submit_page_url(contest_id: &str) -> String {
        return format!("https://codeforces.com/contest/{}/submit", contest_id);
    }
    pub fn build_submit_url(contest_id: &str, csrf_token: &str) -> String {
        return format!(
            "https://codeforces.com/contest/{}/submit?csrf_token={}",
            contest_id, csrf_token
        );
    }

    pub fn build_index_url() -> String {
        return String::from("https://codeforces.com/");
    }
}
