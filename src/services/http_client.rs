use async_trait::async_trait;
use crate::services::error::ServiceError;

#[async_trait]
pub trait HttpClient {
    async fn get(&self, url: &str) -> Result<String, ServiceError>;
    async fn post(&self, url: &str, body: &str) -> Result<String, ServiceError>;
    async fn put(&self, url: &str, body: &str) -> Result<String, ServiceError>;
    async fn delete(&self, url: &str) -> Result<String, ServiceError>;
}

pub struct ReqwestClient;

impl ReqwestClient {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn get(&self, url: &str) -> Result<String, ServiceError> {
        let response = reqwest::get(url)
            .await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to get request from {}. {}", url, e.to_string())
                )
            )?;


        let body = response.text()
            .await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to get get-response body from {}. {}", url, e.to_string())
                )
            )?;

        Ok(body)
    }

    async fn post(&self, url: &str, body: &str) -> Result<String, ServiceError> {
        let response = reqwest::Client::new()
            .post(url)
            .body(body.to_string())
            .send()
            .await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to post request to {}. {}", url, e.to_string())
                )
            )?;

        let body = response.text().await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to get post-response body from {}. {}", url, e.to_string())
                )
            )?;

        Ok(body)
    }

    async fn put(&self, url: &str, body: &str) -> Result<String, ServiceError> {
        let response = reqwest::Client::new()
            .put(url)
            .body(body.to_string())
            .send()
            .await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to put request to {}. {}", url, e.to_string())
                )
            )?;

        let body = response.text().await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to get put-response body from {}. {}", url, e.to_string())
                )
            )?;

        Ok(body)
    }

    async fn delete(&self, url: &str) -> Result<String, ServiceError> {
        let response = reqwest::Client::new()
            .delete(url)
            .send()
            .await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to delete request to {}. {}", url, e.to_string())
                )
            )?;

        let body = response.text().await
            .map_err(|e|
                ServiceError::HttpClient(
                    format!("Failed to get delete-response body from {}. {}", url, e.to_string())
                )
            )?;

        Ok(body)
    }
}