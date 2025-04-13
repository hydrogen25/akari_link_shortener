use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WaaAiError {
    #[error("Request failed: {0}")]
    RequestError(String),
    #[error("Response parsing failed: {0}")]
    ParseError(String),
    #[error("API responded with error: {0}")]
    ApiError(String),
}

#[derive(Deserialize, Debug)]
pub struct ShortenResponse {
    pub data: LinkData,
    pub success: bool,
    pub status: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LinkData {
    pub link: String,
    pub short_code: String,
    pub long_url: String,
    pub delete_hash: String,
    pub created_at: String,
    pub last_visited: Option<String>,
    pub clicks: u32,
    pub extension: Option<String>,
    pub private_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkInfoResponse {
    pub data: LinkData,
    pub success: bool,
    pub status: u16,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShortenRequest<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteData {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteResponse {
    pub data: DeleteData,
    pub success: bool,
    pub status: u16,
}

pub struct WaaAiClient<'a> {
    api_key: &'a str,
    http_client: reqwest::Client,
}

impl<'a> WaaAiClient<'a> {
    pub fn new(api_key: &'a str) -> Self {
        Self {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn shorten_link(
        &self,
        url: &str,
        custom_code: Option<&str>,
        private: Option<bool>,
    ) -> Result<String, WaaAiError> {
        let full_response = self.shorten_link_full(url, custom_code, private).await?;
        Ok(full_response.data.link)
    }

    pub async fn shorten_link_full(
        &self,
        url: &str,
        custom_code: Option<&str>,
        private: Option<bool>,
    ) -> Result<ShortenResponse, WaaAiError> {
        let body = ShortenRequest {
            url,
            custom_code,
            private,
        };

        let mut headers = HeaderMap::new();
        let auth = format!("API-key {}", self.api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth).map_err(|e| WaaAiError::RequestError(e.to_string()))?,
        );

        let res = self
            .http_client
            .post("https://api.waa.ai/v2/links")
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| WaaAiError::RequestError(e.to_string()))?;

        let status = res.status();

        if status.is_success() {
            res.json::<ShortenResponse>()
                .await
                .map_err(|e| WaaAiError::ParseError(e.to_string()))
        } else {
            let msg = res
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".into());
            Err(WaaAiError::ApiError(msg))
        }
    }

    pub async fn get_link_info(&self, short_code: &str) -> Result<LinkInfoResponse, WaaAiError> {
        let res = self
            .http_client
            .get(format!("https://api.waa.ai/v2/links/{}", short_code))
            .send()
            .await
            .map_err(|e| WaaAiError::RequestError(e.to_string()))?;

        let status = res.status();

        if status.is_success() {
            res.json::<LinkInfoResponse>()
                .await
                .map_err(|e| WaaAiError::ParseError(e.to_string()))
        } else {
            let msg = res
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".into());
            Err(WaaAiError::ApiError(msg))
        }
    }
    pub async fn get_links(&self) -> Result<ShortenResponse, WaaAiError> {
        let auth = format!("API-key {}", self.api_key);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth).map_err(|e| WaaAiError::RequestError(e.to_string()))?,
        );
        let c = Client::new();

        let res = c
            .get("https://api.waa.ai/v2/links")
            .headers(headers)
            .send()
            .await
            .map_err(|e| WaaAiError::RequestError(e.to_string()))?;

        let status = res.status();

        if status.is_success() {
            res.json::<ShortenResponse>()
                .await
                .map_err(|e| WaaAiError::ParseError(e.to_string()))
        } else {
            let msg = res
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".into());
            Err(WaaAiError::ApiError(msg))
        }
    }

    pub async fn delete_link(&self, short_code: &str) -> Result<DeleteResponse, WaaAiError> {
        let auth = format!("API-key {}", self.api_key);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth).map_err(|e| WaaAiError::RequestError(e.to_string()))?,
        );
        let c = Client::new();
        let res = c
            .delete(format!("https://api.waa.ai/v2/links/{}", short_code))
            .headers(headers)
            .send()
            .await
            .map_err(|e| WaaAiError::RequestError(e.to_string()))?;

        let status = res.status();
        if status.is_success() {
            res.json::<DeleteResponse>()
                .await
                .map_err(|e| WaaAiError::ParseError(e.to_string()))
        } else {
            let msg = res
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".into());
            Err(WaaAiError::ApiError(msg))
        }
    }

    pub async fn delete_link_unauthed(
        &self,
        short_code: &str,
        delete_hash: &str,
    ) -> Result<DeleteResponse, WaaAiError> {
        let c = Client::new();
        let res = c
            .delete(format!(
                "https://api.waa.ai/v2/links/{}/{}",
                short_code, delete_hash
            ))
            .send()
            .await
            .map_err(|e| WaaAiError::RequestError(e.to_string()))?;

        let status = res.status();
        if status.is_success() {
            res.json::<DeleteResponse>()
                .await
                .map_err(|e| WaaAiError::ParseError(e.to_string()))
        } else {
            let msg = res
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".into());
            Err(WaaAiError::ApiError(msg))
        }
    }
}
