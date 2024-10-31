use crate::errors::ApiClientError;
use reqwest::{Client, StatusCode};
use tracing::instrument;

#[derive(Debug)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    #[instrument(fields(url=url))]
    pub async fn send_get(&self, url: &str) -> Result<serde_json::Value, ApiClientError> {
        let res = self.client.get(url).send().await?;

        match res.status() {
            StatusCode::OK => {
                let body = res.text().await?;
                let value = serde_json::from_str(&body)?;

                Ok(value)
            }
            _ => Err(ApiClientError::StatusCode(res.status())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_send_get_ok() {
        // Arrange
        let response = r#"{
            "id": 443,
            "name": "wormadan",
            "flavor_text_entries": [
                {
                    "flavor_text": "when the bulb on its...",
                    "language": {
                        "name": "en",
                        "url": "https://pokeapi..."
                    }
                }
            ]
        }"#;

        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.path("/pokemon-species");
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        // Act
        let res = ApiClient::new()
            .send_get(&server.url("/pokemon-species"))
            .await
            .unwrap();

        // Assert
        mock.assert();

        assert_eq!(res, serde_json::Value::from_str(response).unwrap());
    }

    #[tokio::test]
    async fn test_send_get_error() {
        // Arrange
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.path("/any");
            then.status(500);
        });

        // Act
        let res = ApiClient::new().send_get(&server.url("/any")).await;

        // Assert
        mock.assert();

        assert!(res.is_err());
        match res {
            Ok(_) => panic!("Expected error, got Ok result"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "Status code: '500 Internal Server Error'".to_string()
                );
            }
        }
    }
}
