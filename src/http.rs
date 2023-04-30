use std::sync::Arc;

use reqwest::{
    blocking::Client as ReqwestClient,
    cookie::{CookieStore, Jar},
    header::HeaderValue,
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
        match res.text() {
            Ok(text) => Ok(text),
            Err(err) => Err(format!("Get body error, {}", err)),
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
