mod atcoder;
mod codeforces;
mod traits;

use colored::Colorize;

use crate::{
    database::CONFIG_DB,
    model::{
        AccountInfo, Contest, ContestStatus, Platform, PlatformLanguage, PostSubmissionInfo,
        TestCase,
    },
    utility::http_client::HttpClient,
};

use self::{atcoder::AtCoder, codeforces::Codeforces, traits::OnlineJudgeBehavior};
pub struct OnlineJudge {
    pub platform: Platform,
    pub username: String,
    pub password: String,
    pub http_client: HttpClient,
}
impl Drop for OnlineJudge {
    fn drop(&mut self) {
        let _ = self.save_cookies();
    }
}

impl OnlineJudge {
    pub fn new(account_info: AccountInfo, platform: Platform) -> Self {
        let endpoint = match platform {
            Platform::Codeforces => Codeforces::build_endpoint_url(),
            Platform::AtCoder => AtCoder::build_endpoint_url(),
        };
        let http_client = HttpClient::new(&account_info.cookies, &endpoint);
        Self {
            platform,
            username: account_info.username,
            password: account_info.password,
            http_client,
        }
    }
    fn save_cookies(&mut self) -> Result<(), String> {
        if !self.username.is_empty() {
            return CONFIG_DB.save_cookies(
                self.platform,
                &self.username,
                &self.http_client.save_cookies(),
            );
        }
        return Ok(());
    }
}

impl OnlineJudge {
    pub async fn submit(
        &mut self,
        full_problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String> {
         if let Err(info) = self.login().await{
            return Err(info);
        }
        let vec = full_problem_identifier.split("_").collect::<Vec<_>>();
        log::info!("vec: {:?}", vec);
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_identifier = vec[0];
        let problem_identifier = vec[1];

        let submit_page_url = match self.platform {
            Platform::Codeforces => Codeforces::build_submit_page_url(contest_identifier),
            Platform::AtCoder => AtCoder::build_submit_page_url(contest_identifier),
        };
        let resp = match self.http_client.get(&submit_page_url).await {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };

        let csrf_token = match match self.platform {
                    Platform::Codeforces => Codeforces::get_csrf_token(&resp),
                    Platform::AtCoder => AtCoder::get_csrf_token(&resp),
                } {
            Ok(token) => {token},
            Err(info) => {
                return Err(info);
            },
        };
        
        let submit_url = match self.platform {
            Platform::Codeforces => Codeforces::build_submit_url(contest_identifier, &csrf_token),
            Platform::AtCoder => AtCoder::build_submit_url(contest_identifier, &csrf_token),
        };
        let data = match self.platform {
            Platform::Codeforces => Codeforces::build_submit_request_data(
                contest_identifier,
                problem_identifier,
                code,
                lang_id,
                &csrf_token,
            ),
            Platform::AtCoder => AtCoder::build_submit_request_data(
                contest_identifier,
                problem_identifier,
                code,
                lang_id,
                &csrf_token,
            ),
        };
        let resp = match self.http_client.post_form(&submit_url, &data).await {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };

        return match self.platform {
            Platform::Codeforces => Codeforces::parse_recent_submission_id(&resp),
            Platform::AtCoder => AtCoder::parse_recent_submission_id(&resp),
        };
    }

    pub async fn is_login(&mut self) -> bool {
        let check_login_url = match self.platform {
            Platform::Codeforces => Codeforces::build_check_login_url(),
            Platform::AtCoder => AtCoder::build_check_login_url(),
        };
        let main_page = match self.http_client.get(&check_login_url).await {
            Ok(resp) => resp,
            Err(info) => {
                log::error!("Failed to get main page: {}", info);
                return false;
            }
        };
        match self.platform {
            Platform::Codeforces => Codeforces::check_login(&main_page),
            Platform::AtCoder => AtCoder::check_login(&main_page),
        }
    }

    pub async fn login(&mut self) -> Result<String, String> {
        if self.is_login().await {
            return Ok(String::from("Already login."));
        }
        let login_page_url = match self.platform {
            Platform::Codeforces => Codeforces::build_login_page_url(),
            Platform::AtCoder => AtCoder::build_login_page_url(),
        };
        let resp = match self.http_client.get(&login_page_url).await {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        let csrf_token = match match self.platform {
            Platform::Codeforces => Codeforces::get_csrf_token(&resp),
            Platform::AtCoder => AtCoder::get_csrf_token(&resp),
        } {
            Ok(csrf_token) => csrf_token,
            Err(info) => {
                return Err(info);
            }
        };
        let data = match self.platform {
            Platform::Codeforces => {
                Codeforces::build_login_request_data(&self.username, &self.password, &csrf_token)
            }
            Platform::AtCoder => {
                AtCoder::build_login_request_data(&self.username, &self.password, &csrf_token)
            }
        };

        let login_url = match self.platform {
            Platform::Codeforces => Codeforces::build_login_url(),
            Platform::AtCoder => AtCoder::build_login_url(),
        };
        let _ = match self.http_client.post_form(&login_url, &data).await {
            Ok(_) => {}
            Err(info) => return Err(info),
        };
        if self.is_login().await {
            return Ok(String::from("Login successfully."));
        } else {
            return Err(String::from("Login failed."));
        }
    }

    /// Get all problems in a contest, return a vector of problem identifier and problem url  
    pub async fn get_problems(
        &mut self,
        contest_identifier: &str,
    ) -> Result<Vec<[String; 2]>, String> {
        if let Err(info) = self.login().await {
            return Err(info);
        }
        let problem_list_url = match self.platform {
            Platform::Codeforces => Codeforces::build_problem_list_url(contest_identifier),
            Platform::AtCoder => AtCoder::build_problem_list_url(contest_identifier),
        };
        let resp = match self.http_client.get(&problem_list_url).await {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return match self.platform {
            Platform::Codeforces => Codeforces::parse_problem_list(contest_identifier, &resp),
            Platform::AtCoder => AtCoder::parse_problem_list(contest_identifier, &resp),
        };
    }

    pub async fn get_test_cases(&mut self, problem_url: &str) -> Result<Vec<TestCase>, String> {
        let resp = match self.http_client.get(&problem_url).await {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        match self.platform {
            Platform::Codeforces => Codeforces::parse_test_cases(&resp),
            Platform::AtCoder => AtCoder::parse_test_cases(&resp),
        }
    }

    pub async fn retrive_result(
        &mut self,
        full_problem_identifier: &str,
        submission_id: &str,
    ) -> Result<PostSubmissionInfo, String> {
        let vec = full_problem_identifier.split("_").collect::<Vec<_>>();
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_identifier = vec[0];
        let problem_identifier = vec[1];
        let submission_url = match self.platform {
            Platform::Codeforces => {
                Codeforces::build_submission_url(contest_identifier, submission_id)
            }
            Platform::AtCoder => AtCoder::build_submission_url(contest_identifier, submission_id),
        };
        let resp = match self.http_client.get(&submission_url).await {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        match self.platform {
            Platform::Codeforces => Codeforces::parse_submission_page(
                contest_identifier,
                problem_identifier,
                submission_id,
                &resp,
            ),
            Platform::AtCoder => AtCoder::parse_submission_page(
                contest_identifier,
                problem_identifier,
                submission_id,
                &resp,
            ),
        }
    }

    pub async fn get_contest(&mut self, contest_identifier: &str) -> Result<Contest, String> {
        if let Err(info) = self.login().await {
            return Err(info);
        }
        let contest_url = match self.platform {
            Platform::Codeforces => Codeforces::build_contest_url(contest_identifier),
            Platform::AtCoder => AtCoder::build_contest_url(contest_identifier),
        };
        let resp = match self.http_client.get(&contest_url).await {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return match self.platform {
            Platform::Codeforces => Codeforces::parse_contest(contest_identifier, &resp),
            Platform::AtCoder => AtCoder::parse_contest(contest_identifier, &resp),
        };
    }
    pub async fn get_contest_test_cases(
        &mut self,
        contest_identifier: &str,
    ) -> Result<Vec<(String, Vec<TestCase>)>, String> {
        let contest = match self.get_contest(contest_identifier).await {
            Ok(contest) => contest,
            Err(info) => {
                return Err(info);
            }
        };
        println!("Get contest {} success.", contest_identifier.bright_blue());
        let mut contest_test_cases = Vec::new();
        if contest.status != ContestStatus::NotStarted {
            let problem_infos = match self.get_problems(contest_identifier).await {
                Ok(problem_infos) => problem_infos,
                Err(info) => {
                    return Err(info);
                }
            };
            for problem_info in problem_infos {
                let test_cases = match self.get_test_cases(&problem_info[1]).await {
                    Ok(test_cases) => test_cases,
                    Err(info) => {
                        return Err(info);
                    }
                };
                println!(
                    "Get test case for {} success.",
                    problem_info[0].bright_blue()
                );
                let problem_identifier = problem_info[0].clone();
                contest_test_cases.push((problem_identifier, test_cases));
            }
        } else {
            return Err(format!("Contest {} not started", contest_identifier));
        }
        return Ok(contest_test_cases);
    }

    pub fn get_platform_languages(platform: Platform) -> Vec<PlatformLanguage> {
        match platform {
            Platform::Codeforces => Codeforces::get_platform_languages(),
            Platform::AtCoder => AtCoder::get_platform_languages(),
        }
    }
}
