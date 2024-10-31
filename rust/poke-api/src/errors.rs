use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PokemonError {
    #[error("ApiClient error: '{0}'")]
    ApiClient(#[from] ApiClientError),
    #[error("Serde error: '{0}'")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum ApiClientError {
    #[error("Reqwest error: '{0}'")]
    Reqwest(#[from] reqwest::Error),
    #[error("Status code: '{0}'")]
    StatusCode(reqwest::StatusCode),
    #[error("Serde error: '{0}'")]
    Serde(#[from] serde_json::Error),
}

impl ResponseError for PokemonError {
    fn error_response(&self) -> HttpResponse {
        unimplemented!()

        // match self {
        //     PokemonError::ApiClient(ApiClientError::NotFound) => {
        //         HttpResponse::NotFound().body("The requested resource was not found")
        //     }
        //     _ => HttpResponse::InternalServerError().body("Internal server error"),
        // }
    }
}
