/// structs and methods for oauth2 authentication flow
use serde::{Deserialize, Serialize};
use serde_json;
use serde_urlencoded;
use std::str::FromStr;
use std::time;

const AUTHORIZE_URL: &str = "https://myanimelist.net/v1/oauth2/authorize";
const TOKEN_URL: &str = "https://myanimelist.net/v1/oauth2/token";
// const REDIRECT_URL: &str = "https://myanimelist.net";

/// An Authorization Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Token Type
    pub token_type: String,
    /// When the token will expire relative to when it was created in seconds
    pub expires_in: u64,
    /// Access token for api requests
    pub access_token: String,
    /// Refresh token for refreshing the access token when it expires
    pub refresh_token: String,
}

/// Holds token and timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenWrapper {
    /// The token
    pub token: Token,
    /// The time that the token was generated
    pub generate_time: u64,
}

impl TokenWrapper {
    /// Returns seconds since the unix epoch
    fn sec_since_epoch() -> u64 {
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    /// Creates a new TokenWrapper
    pub fn new(token: Token) -> Self {
        TokenWrapper {
            token,
            generate_time: Self::sec_since_epoch(),
        }
    }
    /// Check if the token is expired
    pub fn expired(&self) -> bool {
        let now = Self::sec_since_epoch();
        now >= self.generate_time + self.token.expires_in
    }

    /// Get seconds until expiry (None if already expired)
    pub fn expires_in_secs(&self) -> Option<u64> {
        let now = Self::sec_since_epoch();
        let expires_in = self.generate_time + self.token.expires_in;
        if now >= expires_in {
            None
        } else {
            Some(expires_in - now)
        }
    }
    /// Get the time that the token will expire (None if already expired)
    pub fn expire_time(&self) -> Option<time::SystemTime> {
        if let Some(secs) = self.expires_in_secs() {
            Some(time::SystemTime::now() + time::Duration::from_secs(secs))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
