use std::{collections::HashMap, fs::File, io::Write, path::Path, sync::Arc};

use reqwest::{
    blocking::Client as ReqwestClient,
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Url,
};

pub struct HttpClient {
    client: ReqwestClient,
    cookies_store: Arc<Jar>,
    endpoint: String,
}

impl HttpClient {
    #[allow(unused)]
    pub fn new(cookies: &str, endpoint: &str) -> Self {
        let cookies_store = Arc::new(Self::load_cookie_store(cookies, endpoint).unwrap());
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_provider(cookies_store.clone())
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
            .build()
            .unwrap();
        Self {
            client: client,
            cookies_store: cookies_store,
            endpoint: endpoint.to_string(),
        }
    }
    pub fn save_cookies(&mut self) -> String {
        let mut cookies = String::new();
        if let Some(cookie) = self
            .cookies_store
            .cookies(&self.endpoint.parse::<Url>().unwrap())
        {
            cookies.push_str(cookie.to_str().unwrap());
        }
        return cookies;
    }

    #[allow(unused)]
    pub fn get(&mut self, url: &str) -> Result<String, String> {
        let res = match self.client.get(url).send() {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!("Request error, request url: {}", url));
        }
        match res.text() {
            Ok(text) => Ok(text),
            Err(err) => Err(format!("Get body error, {}", err)),
        }
    }

    #[allow(unused)]
    pub fn post_form(&mut self, url: &str, form: &HashMap<&str, &str>) -> Result<String, String> {
        log::info!("post data {:?} to {}.", form, url);
        let res = match self.client.post(url).form(&form).send() {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!(
                "Request error, request url: {}, status: {}, error: {}",
                url,
                res.status(),
                res.text_with_charset("utf-8").unwrap()
            ));
        }
        match res.text() {
            Ok(text) => {
                HttpClient::debug_save(&text, ".html");
                Ok(text)
            }
            Err(err) => Err(format!("Post form error, {}", err)),
        }
    }
    #[allow(unused)]
    pub fn post_data(&mut self, url: &str, json: &HashMap<&str, &str>) -> Result<String, String> {
        log::info!("post data to {}.", url);
        let res = match self.client.post(url).json(json).send() {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!(
                "Request error, request url: {}, status: {}, error: {}",
                url,
                res.status(),
                res.text_with_charset("utf-8").unwrap()
            ));
        }
        match res.text() {
            Ok(text) => Ok(text),
            Err(err) => Err(format!("Post form error, {}", err)),
        }
    }
    #[allow(unused)]
    pub fn debug_save(text: &str, suffix: &str) {
        let mut save_path = random_str::get_string(10, true, false, false, false);
        save_path.push_str(suffix);
        let path = Path::new(save_path.as_str());
        match File::create(path) {
            Ok(mut file) => match file.write_all(text.as_bytes()) {
                Ok(_) => {
                    log::info!("debug content write to {}", path.display());
                }
                Err(_) => {
                    log::error!("write content failed.");
                }
            },
            Err(_) => {
                log::error!("Write content failed.");
            }
        }
    }
    pub fn load_cookie_store(cookies: &str, endpoint: &str) -> Result<Jar, String> {
        let url = endpoint.parse().unwrap();
        let jar = reqwest::cookie::Jar::default();
        let v = cookies
            .split("; ")
            .map(|s| HeaderValue::from_str(s).unwrap())
            .collect::<Vec<_>>();
        jar.set_cookies(&mut v.iter(), &url);
        Ok(jar)
    }
}

#[test]
fn test_client() {
    let url = "https://atcoder.jp";
    let mut client = HttpClient::new("", url);
    match client.get("https://atcoder.jp") {
        Ok(resp) => {
            use std::fs::File;
            let mut file = File::create("test.html").unwrap();
            file.write_all(resp.as_bytes()).unwrap();
            assert!(resp.is_empty() == false);
            std::fs::remove_file("test.html").unwrap();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
    let cookies = client.save_cookies();
    print!("{}", cookies);
    assert_ne!(cookies.len(), 0);
}
