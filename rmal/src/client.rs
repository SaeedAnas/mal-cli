////! MAL api endpoints
//// 3rd Party crates
//use log::{error, trace};
//use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
//use reqwest::{Client, Method, Response, StatusCode};
//use serde::Deserialize;
//use serde_json::map::Map;
//use serde_json::{json, Value};
//use thiserror::Error;

//// Crate
//use super::oauth2::MALClientCredentials;

//pub const API_URL: &str = "https://api.myanimelist.net/v2";

///// All errors returned from `rmal` client.
//#[derive(Debug, Error)]
//pub enum ClientError {
//    #[error("request unauthorized")]
//    Unauthorized,
//    #[error("exceeded request limit")]
//    RateLimited(Option<usize>),
//    #[error("MAL error: {0}")]
//    Api(#[from] ApiError),
//    #[error("request error: {0}")]
//    ParseJSON(#[from] serde_json::Error),
//    #[error("request error: {0}")]
//    Request(#[from] reqwest::Error),
//    #[error("status code: {0}")]
//    StatusCode(StatusCode),
//}

//impl ClientError {
//    async fn from_response(response: Response) -> Self {
//        match response.status() {
//            StatusCode::UNAUTHORIZED => Self::Unauthorized,
//            StatusCode::TOO_MANY_REQUESTS => Self::RateLimited(
//                response
//                    .headers()
//                    .get(reqwest::header::RETRY_AFTER)
//                    .and_then(|header| header.to_str().ok())
//                    .and_then(|duration| duration.parse().ok()),
//            ),
//            status @ StatusCode::FORBIDDEN | status @ StatusCode::NOT_FOUND => response
//                .json::<ApiError>()
//                .await
//                .map(Into::into)
//                .unwrap_or_else(|_| status.into()),
//        }
//    }
//}

//impl From<StatusCode> for ClientError {
//    fn from(code: StatusCode) -> Self {
//        Self::StatusCode(code)
//    }
//}

//type ClientResult<T> = Result<T, ClientError>;

///// MAL API object
//#[derive(Debug, Clone)]
//pub struct MAL {
//    client: Client,
//    pub access_token: Option<String>,
//    pub client_credentials_manager: Option<MALClientCredentials>,
//}

//impl MAL {
//    pub fn default() -> Self {
//        Self {
//            client: Client::new(),
//            access_token: None,
//            client_credentials_manager: None,
//        }
//    }

//    pub fn access_token(mut self, access_token: &str) -> MAL {
//        self.access_token = Some(access_token.to_owned());
//        self
//    }

//    pub fn client_credentials_manager(
//        mut self,
//        client_credentials_manager: MALClientCredentials,
//    ) -> MAL {
//        self.client_credentials_manager = Some(client_credentials_manager);
//        self
//    }

//    pub fn build(self) -> Self {
//        if self.access_token.is_none() && self.client_credentials_manager.is_none() {
//            panic!("access token and credentials are not found!");
//        }
//        self
//    }
//}
