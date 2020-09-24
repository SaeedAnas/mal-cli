// TODO: FIx this stuff
// use log::{debug, error, trace};
// use rand::{distributions::Alphanumeric, thread_rng, Rng};
// use serde::{Deserialize, Serialize};
// use serde_json;
// use serde_urlencoded;
// use std::env;
// use std::time;
// use std::{io::Error, iter, process::Output, str::FromStr};
// use url::Url;

// use super::util::new_challenge;

// /// Client credentials for MAL client
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MALClientCredentials {
//     pub client_id: String,
//     pub client_secret: Option<String>,
//     pub token_info: Option<TokenInfo>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MALOAuth {
//     pub client_id: String,
//     pub client_secret: Option<String>,
//     pub redirect_uri: String,
//     pub user_agent: String,
//     pub challenge: String,
//     pub state: String,
//     pub auth_code: Option<String>,
//     pub token_info: Option<TokenInfo>,
// }

// /// MAL oauth token info
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct TokenInfo {
//     pub access_token: String,
//     pub token_type: String,
//     pub expires_in: u64,
//     pub expires_at: Option<i64>,
//     pub refresh_token: String,
// }

// impl TokenInfo {
//     pub fn default() -> Self {
//         Self {
//             access_token: String::new(),
//             token_type: String::new(),
//             expires_in: 0u64,
//             expires_at: None,
//             refresh_token: String::new(),
//         }
//     }
//     pub fn access_token(mut self, access_token: &str) -> Self {
//         self.access_token = access_token.to_owned();
//         self
//     }
//     pub fn token_type(mut self, token_type: &str) -> Self {
//         self.token_type = token_type.to_owned();
//         self
//     }
//     pub fn expires_in(mut self, expires_in: u64) -> Self {
//         self.expires_in = expires_in;
//         self
//     }
//     pub fn expires_at(mut self, expires_at: i64) -> Self {
//         self.expires_at = Some(expires_at);
//         self
//     }
//     pub fn refresh_token(mut self, refresh_token: &str) -> Self {
//         self.refresh_token = refresh_token.to_owned();
//         self
//     }
//     pub fn set_expires_at(&mut self, expires_at: i64) {
//         self.expires_at = Some(expires_at);
//     }
//     pub fn set_refresh_token(&mut self, refresh_token: &str) {
//         self.refresh_token = refresh_token.to_owned();
//     }
// }

// impl MALClientCredentials {
//     pub fn default() -> Self {
//         Self {
//             client_id: String::new(),
//             client_secret: String::new(),
//             token_info: None,
//         }
//     }

//     pub fn client_id(mut self, client_id: &str) -> Self {
//         self.client_id = client_id.to_owned();
//         self
//     }

//     pub fn client_secret(mut self, client_secret: &str) -> Self {
//         self.client_secret = client_secret.to_owned();
//         self
//     }

//     pub fn token_info(mut self, token_info: TokenInfo) -> Self {
//         self.token_info = Some(token_info);
//         self
//     }

//     pub fn build(self) -> Self {
//         if self.client_id.is_empty() || self.client_secret.is_empty() {
//             error!("Client ID or Client Secret not found")
//         }
//         self
//     }
// }

// impl MALOAuth {
//     // MAL token example:
//     // {
//     //      "token_type": "Bearer",
//     //      "expires_in": 2000,
//     //      "access_token": "ABC...xyz",
//     //      "refresh_token": "ZYX...cba"
//     // }

//     pub fn default() -> Self {}
// }
