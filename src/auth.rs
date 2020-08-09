use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
/// structs and methods for oauth2 authentication flow
use serde::{Deserialize, Serialize};
use serde_json;
use serde_urlencoded;
use std::iter;
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

const CODE_CHALLENGE_LENGTH: usize = 128;

#[derive(Clone, Serialize, Deserialize)]
struct Auth {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleint_secret: Option<String>,
    pub redirect_url: String,
    // pub user_agent: String,
    pub challenge: String,
    pub auth_code: Option<String>,
    pub token: Option<TokenWrapper>,
}

impl Auth {
    /// Start of a new oauth2 flow
    /// # Parameters
    /// * `user`
    pub fn new<A: ToString>(
        user_agent: A,
        client_id: A,
        client_secret: Option<A>,
        redirect_url: A,
    ) -> Self {
        Auth {
            client_id: client_id.to_string(),
            cleint_secret: if let Some(cs) = client_secret {
                Some(cs.to_string())
            } else {
                None
            },
            redirect_url: redirect_url.to_string(),
            // user_agent: user_agent.to_string,
            challenge: Self::new_challenge(CODE_CHALLENGE_LENGTH),
            auth_code: None,
            token: None,
        }
    }

    /// Generates a new base64-encoded SHA-256 PKCE code
    /// # Panic
    /// `len` needs to be a value between 48 and 128
    fn new_challenge(len: usize) -> String {
        // Check whether the len in in between the valid length for a
        // PKCE code (43 chars - 128 chars)
        if len < 48 || len > 128 {
            panic!("len is not in between 48 and 128");
        }
        let mut rng = thread_rng();
        let challenge: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(len)
            .collect();
        challenge
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_challenge() {
        let challenge = Auth::new_challenge(CODE_CHALLENGE_LENGTH);

        assert!(challenge.len() == CODE_CHALLENGE_LENGTH);
        println!("{}", challenge);
        println!(
            "len: {}, CODE_CHALLENGE_LEN: {}",
            challenge.len(),
            CODE_CHALLENGE_LENGTH
        );
    }
    #[test]
    #[should_panic(expected = "len is not in between 48 and 128")]
    fn test_challenge_len() {
        // should panic
        let challenge = Auth::new_challenge(5);
    }
}
