use std::{fs::File, io::Write, path::Path, sync::Arc};

use reqwest::{
    blocking::Client as ReqwestClient,
    cookie::{CookieStore, Jar},
    header::{HeaderValue, SET_COOKIE},
    Url,
};

pub struct Client {
    client: ReqwestClient,
    cookies_store: Arc<Jar>,
    endpoint: String,
}

fn load_cookie_store(cookies: &str, endpoint: &str) -> Result<Jar, String> {
    let url = endpoint.parse().unwrap();
    let jar = reqwest::cookie::Jar::default();
    let v = cookies
        .split("; ")
        .map(|s| HeaderValue::from_str(s).unwrap())
        .collect::<Vec<_>>();
    jar.set_cookies(&mut v.iter(), &url);
    Ok(jar)
}
impl Client {
    pub fn new(cookies: &str, endpoint: &str) -> Result<Self, String> {
        let cookies_store = Arc::new(load_cookie_store(cookies, endpoint)?);
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_provider(cookies_store.clone())
            .cookie_store(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
            .build()
            .unwrap();
        Ok(Self {
            client: client,
            cookies_store: cookies_store,
            endpoint: endpoint.to_string(),
        })
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
    pub fn get(&mut self, url: &str) -> Result<String, String> {
        let res = match self.client.get(url).send() {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!("Request error, request url: {}", url));
        }
        if res.headers().contains_key(SET_COOKIE){
            self.cookies_store.add_cookie_str(res.headers().get(SET_COOKIE).unwrap().to_str().unwrap(), &self.endpoint.parse::<Url>().unwrap());
        }
        log::debug!("cookies: {}", self.save_cookies());
        match res.text() {
            Ok(text) => {
                Ok(text)
            }
            Err(err) => Err(format!("Get body error, {}", err)),
        }
    }
    pub fn post_form(&mut self, url: &str, form: &[(&str, &str)]) -> Result<String, String> {
        let res = match self.client.post(url).form(form).send() {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!("Request error, request url: {}", url));
        }
        if res.headers().contains_key(SET_COOKIE){
            self.cookies_store.add_cookie_str(res.headers().get(SET_COOKIE).unwrap().to_str().unwrap(), &self.endpoint.parse::<Url>().unwrap());
        }
        log::debug!("cookies: {}", self.save_cookies());

        match res.text() {
            Ok(text) => {
                Ok(text)
            }
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
}

#[test]
fn test_client() {
    let url = "https://baidu.com";
    let mut client = Client::new("", url).unwrap();
    match client.get("https://baidu.com") {
        Ok(resp) => {
            log::debug!("{}", resp);
            assert!(resp.is_empty() == false)
        }
        Err(err) => {
            log::error!("{}", err);
        }
    }
    let cookies = client.save_cookies();
    log::debug!("{}", cookies);
    assert_ne!(cookies.len(), 0);
}
