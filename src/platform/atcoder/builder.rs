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
}
