use crate::misc::http_client::HttpClient;
use crate::model::SubmissionInfo;
use crate::{library::OnlineJudge, model::Verdict};
mod builder;
mod config;
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
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_id = vec[0];

        let submit_page_url = UrlBuilder::build_submit_page_url(contest_id);
        let resp = match self.client.get(&submit_page_url) {
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
        data.insert("data.TaskScreenName", problem_identifier);
        data.insert("data.LanguageId", lang_id);
        data.insert("sourceCode", code);
        data.insert("csrf_token", &csrf_token);
        let submit_url = UrlBuilder::build_submit_url(contest_id);
        let resp = match self.client.post_form(&submit_url, &data) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return Self::get_recent_submission_id(&resp);
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
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_identifier = vec[0];
        let problem_url = UrlBuilder::build_problem_url(contest_identifier, problem_identifier);
        let resp = match self.client.get(&problem_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        let soup = Soup::new(&resp);
        let mut inputs_ja = vec![];
        let mut outputs_ja = vec![];
        let mut inputs_en = vec![];
        let mut outputs_en = vec![];

        for h3 in soup.tag("h3").find_all() {
            let text = h3.text();
            if text.contains("入力例") {
                let section = match h3.parent() {
                    Some(section) => section,
                    None => continue,
                };

                let pre = match section.tag("pre").find() {
                    Some(pre) => pre,
                    None => continue,
                };
                inputs_ja.push(pre.text());
            } else if text.contains("出力例") {
                let section = match h3.parent() {
                    Some(section) => section,
                    None => continue,
                };

                let pre = match section.tag("pre").find() {
                    Some(pre) => pre,
                    None => continue,
                };
                outputs_ja.push(pre.text());
            } else if text.contains("Sample Input") {
                let section = match h3.parent() {
                    Some(section) => section,
                    None => continue,
                };
                let pre = match section.tag("pre").find() {
                    Some(pre) => pre,
                    None => continue,
                };
                inputs_en.push(pre.text());
            } else if text.contains("Sample Output") {
                let section = match h3.parent() {
                    Some(section) => section,
                    None => continue,
                };
                let pre = match section.tag("pre").find() {
                    Some(pre) => pre,
                    None => continue,
                };
                outputs_en.push(pre.text());
            }
        }
        let mut sample_vec = Vec::new();
        if !inputs_en.is_empty() && inputs_en.len() == outputs_en.len() {
            for i in 0..inputs_en.len() {
                sample_vec.push([inputs_en[i].clone(), outputs_en[i].clone()]);
            }
        } else if !inputs_ja.is_empty() && inputs_ja.len() == outputs_ja.len() {
            for i in 0..inputs_ja.len() {
                sample_vec.push([inputs_ja[i].clone(), outputs_ja[i].clone()]);
            }
        } else {
            return Err(String::from("Failed to get test cases."));
        }
        return Ok(sample_vec);
    }

    fn retrive_result(
        &mut self,
        problem_identifier: &str,
        submission_id: &str,
    ) -> Result<SubmissionInfo, String> {
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_identifier = vec[0];
        let submission_url = UrlBuilder::build_submission_url(contest_identifier, submission_id);
        let resp = match self.client.get(&submission_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        let soup = Soup::new(&resp);
        let table = match soup.tag("tbody").find() {
            Some(table) => table,
            None => return Err(String::from("Failed to find tbody.")),
        };
        let trs = table.tag("tr").find_all().collect::<Vec<_>>();
        if trs.len() != 9 && trs.len() != 7 {
            return Err(String::from("Failed to find trs."));
        }

        let status = match trs[6].tag("td").find() {
            Some(td) => td.text(),
            None => return Err(String::from("Failed to find status.")),
        };
        let mut execute_time = String::new();
        let mut execute_memory = String::new();
        if trs.len() == 9 {
            execute_time = match trs[7].tag("td").find() {
                Some(td) => td.text(),
                None => return Err(String::from("Failed to find execute time.")),
            };
            execute_memory = match trs[8].tag("td").find() {
                Some(td) => td.text(),
                None => return Err(String::from("Failed to find execute memory.")),
            };
        }
        let mut submission_info = SubmissionInfo::new();
        submission_info.submission_id = String::from(submission_id);
        submission_info.identifier = String::from(problem_identifier);
        submission_info.verdict_info = status;
        submission_info.execute_time = execute_time;
        submission_info.execute_memory = execute_memory;
        submission_info.verdict = Self::parse_verdict_info(&submission_info.verdict_info);
        return Ok(submission_info);
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
    pub fn get_recent_submission_id(resp: &str) -> Result<String, String> {
        let soup = Soup::new(&resp);
        for ele in soup.tag("td").attr("class", "submission-score").find_all() {
            match ele.get("data-id") {
                Some(id) => return Ok(String::from(id)),
                None => continue,
            }
        }
        return Err(String::from("Failed to get recent submit id."));
    }
    pub fn parse_verdict_info(verdict_info: &str) -> Verdict {
        if verdict_info == "Judging" {
            return Verdict::Waiting;
        }
        return Verdict::Resulted;
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
    let _ = match atc.login(&username, &password) {
        Ok(resp) => resp,
        Err(info) => {
            println!("Failed to login, {}", info);
            return;
        }
    };
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

#[test]
#[ignore = "reason: need login"]
fn test_get_test_cases() {
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
    match atc.get_test_cases("abc165_c") {
        Ok(test_cases) => {
            println!("{:?}", test_cases);
        }
        Err(info) => {
            println!("Failed to get test cases, {}", info);
        }
    }
}

#[test]
#[ignore = "reason: need login"]
fn test_submit() {
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
    let code = r#"
#include <iostream>
using namespace std;
int main() {
    int a, b;
    cin >> a >> b;
    cout << a + b << endl;
    return 0;
}
    "#;
    let resp = match atc.submit("arc165_b", code, "5001") {
        Ok(resp) => resp,
        Err(info) => {
            println!("Failed to submit, {}", info);
            return;
        }
    };
    HttpClient::debug_save(&resp, ".html");
}

#[test]
fn test_parse_recent_submission_id() {
    let resp = match std::fs::read_to_string("assets/atcoder/submitted.html") {
        Ok(resp) => resp,
        Err(info) => {
            println!("Failed to read file, {}", info);
            return;
        }
    };
    let id = match AtCoder::get_recent_submission_id(&resp) {
        Ok(resp) => resp,
        Err(info) => {
            println!("Failed to get recent submission id, {}", info);
            return;
        }
    };
    println!("{}", id);
}

#[test]
#[ignore = "reason: need login"]
fn test_retrieve_result() {
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
    let _ = match atc.retrive_result("arc165_b", "46003634") {
        Ok(resp) => resp,
        Err(info) => {
            println!("Failed to retrieve result, {}", info);
            return;
        }
    };
}
