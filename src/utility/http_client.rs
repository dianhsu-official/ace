use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Client as ReqwestClient, Url,
};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::{fs, io::AsyncWriteExt};

pub struct HttpClient {
    client: ReqwestClient,
    cookies_store: Arc<Jar>,
    endpoint: String,
}

impl HttpClient {
    pub fn new(cookies: &str, endpoint: &str) -> Self {
        let cookies_store = Arc::new(Self::load_cookie_store(cookies, endpoint).unwrap());
        let client = reqwest::ClientBuilder::new()
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

    pub async fn get(&mut self, url: &str) -> Result<String, String> {
        let res = match self.client.get(url).send().await {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!("Request error, request url: {}", url));
        }
        match res.text().await {
            Ok(text) => Ok(text),
            Err(err) => Err(format!("Get body error, {}", err)),
        }
    }

    pub async fn post_form(
        &mut self,
        url: &str,
        form: &HashMap<&str, &str>,
    ) -> Result<String, String> {
        log::info!("post data {:?} to {}.", form, url);
        let res = match self.client.post(url).form(&form).send().await {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!(
                "Request error, request url: {}, status: {}, error: {}",
                url,
                res.status(),
                res.text_with_charset("utf-8").await.unwrap()
            ));
        }
        match res.text().await {
            Ok(text) => Ok(text),
            Err(err) => Err(format!("Post form error, {}", err)),
        }
    }
    #[allow(unused)]
    pub async fn post_data(
        &mut self,
        url: &str,
        json: &HashMap<&str, &str>,
    ) -> Result<String, String> {
        log::info!("post data to {}.", url);
        let res = match self.client.post(url).json(json).send().await {
            Ok(res) => res,
            Err(err) => return Err(format!("Request error, {}", err)),
        };
        if !res.status().is_success() {
            return Err(format!(
                "Request error, request url: {}, status: {}, error: {}",
                url,
                res.status(),
                res.text_with_charset("utf-8").await.unwrap()
            ));
        }
        match res.text().await {
            Ok(text) => Ok(text),
            Err(err) => Err(format!("Post form error, {}", err)),
        }
    }
    #[allow(unused)]
    pub async fn debug_save(text: &str, suffix: &str) {
        let mut save_path = random_str::get_string(10, true, false, false, false);
        save_path.push_str(suffix);
        let path = Path::new(save_path.as_str());
        match fs::File::create(path).await {
            Ok(mut file) => match file.write_all(text.as_bytes()).await {
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

#[tokio::test]
async fn test_client() {
    let url = "https://atcoder.jp";
    let mut client = HttpClient::new("", url);
    match client.get("https://atcoder.jp").await {
        Ok(resp) => {
            use std::fs::File;
            let mut file = File::create("test.html").unwrap();
            std::io::Write::write_all(&mut file, resp.as_bytes()).unwrap();
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
