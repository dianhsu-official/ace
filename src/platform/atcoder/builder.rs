pub struct UrlBuilder {}

impl UrlBuilder {
    pub fn build_index_url() -> String {
        return String::from("https://atcoder.jp");
    }
    pub fn build_login_page_url() -> String {
        return String::from("https://atcoder.jp/login");
    }
    pub fn build_login_url() -> String {
        return String::from("https://atcoder.jp/login");
    }
    pub fn build_problem_list_url(contest_identifier: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}/tasks",
            contest_identifier
        ));
    }
    pub fn build_problem_url(problem_identifier: &str) -> String {
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
        let contest_id = vec[0];
        return String::from(format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            contest_id, problem_identifier
        ));
    }
    pub fn build_submit_page_url(contest_identifier: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}/submit",
            contest_identifier
        ));
    }
    pub fn build_submit_url(contest_identifier: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}/submit",
            contest_identifier
        ));
    }
}
