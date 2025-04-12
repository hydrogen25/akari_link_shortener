use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct ShortenLinkResponse {
    data: LinkData,
    success: bool,
    status: u16,
}

#[allow(unused)]
#[derive(Deserialize, Serialize, Debug)]
pub struct LinkData {
    link: String,
    short_code: String,
    long_url: String,
    delete_hash: String,
    created_at: String,
    last_visited: Option<String>,
    clicks: u32,
    extension: Option<String>,
    private_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkInfoResponse {
    data: LinkData,
    success: bool,
    status: u16,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShortenLinkRequest<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
}

#[allow(dead_code)]
struct Client<'a> {
    api_key: &'a str,
}

#[allow(unused)]
impl Client<'_> {
    pub async fn new(api_key: &str) -> Client {
        Client { api_key }
    }
    pub async fn shorten_a_link(
        &self,
        url: &str,
        custom_code: Option<&str>,
        private: Option<bool>,
    ) -> Result<String, String> {
        let client = reqwest::Client::new();
        let mut request_body = ShortenLinkRequest {
            url,
            custom_code: None,
            private: None,
        };

        if let Some(val) = custom_code {
            request_body.custom_code = custom_code;
        };

        if let Some(val) = private {
            request_body.private = private;
        };

        let mut headers = HeaderMap::new();

        let api_key = format!("API-key {}", self.api_key);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());

        let r = client
            .post("https://api.waa.ai/v2/links")
            .headers(headers)
            .json(&request_body)
            .send()
            .await;

        match r {
            Ok(r) => {
                if r.status().is_success() {
                    let json_resp: ShortenLinkResponse = r.json().await.unwrap();
                    Ok(json_resp.data.link)
                } else {
                    Err(r.text().await.unwrap())
                }
            }

            Err(r) => Err(r.to_string()),
        }
    }

    pub async fn get_link_info(&self, short_code: &str) -> Result<LinkInfoResponse, String> {
        let response = reqwest::get(format!("https://api.waa.ai/v2/links/{}", short_code)).await;
        match response {
            Ok(val) => {
                if val.status().is_success() {
                    let re: LinkInfoResponse = val.json().await.unwrap();
                    Ok(re)
                } else {
                    Err(val.text().await.unwrap())
                }
            }
            Err(val) => Err(val.to_string()),
        }
    }

    pub async fn shorten_a_link_all(
        &self,
        url: &str,
        custom_code: Option<&str>,
        private: Option<bool>,
    ) -> Result<ShortenLinkResponse, String> {
        let client = reqwest::Client::new();
        let mut request_body = ShortenLinkRequest {
            url,
            custom_code: None,
            private: None,
        };

        if let Some(val) = custom_code {
            request_body.custom_code = custom_code;
        };

        if let Some(val) = private {
            request_body.private = private;
        };

        let mut headers = HeaderMap::new();

        let api_key = format!("API-key {}", self.api_key);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());

        let r = client
            .post("https://api.waa.ai/v2/links")
            .headers(headers)
            .json(&request_body)
            .send()
            .await;

        match r {
            Ok(r) => {
                if r.status().is_success() {
                    let json_resp: ShortenLinkResponse = r.json().await.unwrap();
                    Ok(json_resp)
                } else {
                    Err(r.text().await.unwrap())
                }
            }

            Err(r) => Err(r.to_string()),
        }
    }
}
