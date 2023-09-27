use crate::misc::http_client::HttpClient;
mod builder;
mod config;
use super::lib::OnlineJudge;
use builder::UrlBuilder;
use soup::{NodeExt, QueryBuilderExt, Soup};
pub struct AtCoder {
    pub client: HttpClient,
    pub host: String,
}
impl OnlineJudge for AtCoder {
    fn submit(
        &mut self,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String> {
        let _ = problem_identifier;
        let _ = code;
        let _ = lang_id;
        todo!()
    }

    fn is_login(&mut self) -> Result<String, String> {
        let resp = match self.client.get(&UrlBuilder::build_index_url()) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        if resp.contains("</span> Sign Out</a></li>") {
            return Ok(String::from("You have logged in."));
        } else {
            return Err(String::from("You have not logged in."));
        }
    }

    fn login(&mut self, username: &str, password: &str) -> Result<String, String> {
        let login_page_url = UrlBuilder::build_login_page_url();
        let resp = match self.client.get(&login_page_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        let csrf_token = match Self::get_csrf(&resp) {
            Some(token) => token,
            None => {
                return Err(String::from("Failed to get csrf token."));
            }
        };
        let mut data = std::collections::HashMap::new();
        data.insert("username", username);
        data.insert("password", password);
        data.insert("csrf_token", &csrf_token);
        let login_url = UrlBuilder::build_login_url();
        let _ = match self.client.post_form(&login_url, &data) {
            Ok(_) => {}
            Err(info) => return Err(info),
        };
        return self.is_login();
    }

    /// Get test cases from AtCoder
    /// Success: Vec<[String; 2]> where [0] is input and [1] is output
    fn get_test_cases(&mut self, problem_identifier: &str) -> Result<Vec<[String; 2]>, String> {
        let _ = problem_identifier;
        todo!()
    }

    fn retrive_result(
        &mut self,
        problem_identifier: &str,
        submit_id: &str,
    ) -> Result<super::lib::SubmissionInfo, String> {
        let _ = problem_identifier;
        let _ = submit_id;
        todo!()
    }

    fn get_problems(&mut self, contest_identifier: &str) -> Result<Vec<String>, String> {
        let problem_list_url = UrlBuilder::build_problem_list_url(contest_identifier);
        let resp = match self.client.get(&problem_list_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        let soup = Soup::new(&resp);
        let tbody = match soup.tag("tbody").find() {
            Some(tbody) => tbody,
            None => {
                return Err(String::from("Failed to find tbody."));
            }
        };
        let trs = tbody.tag("tr").find_all();
        let mut problems = Vec::new();
        for tr in trs {
            let tds = tr.tag("td").find_all().collect::<Vec<_>>();
            if tds.len() != 5 {
                continue;
            }
            let problem_key = tds[0].text().to_lowercase();
            problems.push(format!("{}_{}", contest_identifier, problem_key));
        }
        return Ok(problems);
    }
}
impl AtCoder {
    #[allow(unused)]
    pub fn new(cookies: &str) -> Self {
        return Self {
            client: HttpClient::new(cookies, "https://atcoder.jp"),
            host: String::from("https://atcoder.jp"),
        };
    }

    pub fn get_csrf(resp: &str) -> Option<String> {
        let re = match regex::Regex::new(r#"var csrfToken = "([\S]+)""#) {
            Ok(re) => re,
            Err(_) => return None,
        };
        let caps = match re.captures(resp) {
            Some(caps) => caps,
            None => return None,
        };
        return Some(String::from(&caps[1]));
    }
}

#[test]
#[ignore = "reason: need login"]
fn test_login() {
    dotenv::dotenv().ok();
    let username = match dotenv::var("ATCODER_USERNAME") {
        Ok(username) => username,
        Err(info) => {
            println!("Failed to get username, {}", info);
            return;
        }
    };
    let password = match dotenv::var("ATCODER_PASSWORD") {
        Ok(password) => password,
        Err(info) => {
            println!("Failed to get password, {}", info);
            return;
        }
    };
    let mut atc = AtCoder::new("");
    let resp = match atc.login(&username, &password) {
        Ok(resp) => resp,
        Err(info) => {
            println!("Failed to login, {}", info);
            return;
        }
    };
    println!("{}", resp);
}

#[test]
#[ignore = "reason: need login"]
fn test_get_problems() {
    dotenv::dotenv().ok();
    let username = match dotenv::var("ATCODER_USERNAME") {
        Ok(username) => username,
        Err(info) => {
            println!("Failed to get username, {}", info);
            return;
        }
    };
    let password = match dotenv::var("ATCODER_PASSWORD") {
        Ok(password) => password,
        Err(info) => {
            println!("Failed to get password, {}", info);
            return;
        }
    };
    let mut atc = AtCoder::new("");
    let _ = match atc.login(&username, &password) {
        Ok(resp) => resp,
        Err(info) => {
            println!("Failed to login, {}", info);
            return;
        }
    };
    match atc.get_problems("abc319") {
        Ok(problems) => {
            println!("{:?}", problems);
        }
        Err(info) => {
            println!("Failed to get problems, {}", info)
        }
    }
}
