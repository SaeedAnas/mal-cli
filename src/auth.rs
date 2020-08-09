use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
/// structs and methods for oauth2 authentication flow
use serde::{Deserialize, Serialize};
use serde_json;
use serde_urlencoded;
use std::iter;
use std::str::FromStr;
use std::time;

const USER_AGENT: &str = "mal-cli";
const AUTHORIZE_URL: &str = "https://myanimelist.net/v1/oauth2/authorize";
const TOKEN_URL: &str = "https://myanimelist.net/v1/oauth2/token";
// const REDIRECT_URL: &str = "https://myanimelist.net";

#[derive(Clone, Debug)]
pub enum AuthError {
    UnknownError,
    NetworkTimeout,
    InvalidResponse(String),
    AuthNotPresent,
    TokenNotPresent,
}

impl From<reqwest::Error> for AuthError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            AuthError::NetworkTimeout
        } else {
            AuthError::UnknownError
        }
    }
}

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
    pub client_secret: Option<String>,
    pub redirect_url: String,
    // pub user_agent: String,
    pub challenge: String,
    pub state: String,
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
            client_secret: if let Some(cs) = client_secret {
                Some(cs.to_string())
            } else {
                None
            },
            redirect_url: redirect_url.to_string(),
            // user_agent: user_agent.to_string,
            challenge: Self::new_challenge(CODE_CHALLENGE_LENGTH),
            state: "AUTHSTART".to_string(),
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
        // needs to be url safe so we use Alphanumeric
        let challenge: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(len)
            .collect();
        challenge
    }

    /// Creates a new authorization url
    pub fn get_auth_url(&self) -> url::Url {
        #[derive(Serialize, Debug)]
        struct AuthQuery {
            response_type: String,
            client_id: String,
            code_challenge: String,
            state: String,
            redirect_url: String,
            code_challenge_method: String,
        }

        let auth_query = AuthQuery {
            response_type: "code".to_string(),
            client_id: self.client_id.clone(),
            code_challenge: self.challenge.clone(),
            state: self.state.to_string(),
            redirect_url: self.redirect_url.clone(),
            // mal only supports plain
            code_challenge_method: "plain".to_string(),
        };

        url::Url::from_str(&format!(
            "{}?{}",
            AUTHORIZE_URL,
            serde_urlencoded::to_string(auth_query).unwrap()
        ))
        .unwrap()
    }

    /// Parses redirection url
    pub fn parse_redirect_query_string(&mut self, query_string: &str) -> Result<(), AuthError> {
        #[derive(Deserialize, Debug)]
        struct AuthResponse {
            code: String,
            state: String,
        }

        let auth_response = match serde_urlencoded::from_str::<AuthResponse>(query_string) {
            Ok(r) => r,
            Err(e) => {
                return Err(AuthError::InvalidResponse(e.to_string()));
            }
        };

        if auth_response.state != self.state {
            return Err(AuthError::InvalidResponse("State Mismatch".to_string()));
        }

        self.auth_code = Some(auth_response.code);
        Ok(())
    }

    /// Creates a new url to get the token
    pub fn get_token_query_string(&self) -> Result<String, AuthError> {
        #[derive(Serialize, Debug)]
        struct TokenRequest {
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            code: String,
            code_verifier: String,
            grant_type: String,
        }

        if self.auth_code.is_none() {
            return Err(AuthError::AuthNotPresent);
        }

        let query = TokenRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: self.auth_code.as_ref().unwrap().clone(),
            code_verifier: self.challenge.clone(),
            grant_type: "authorization_code".to_string(),
        };

        Ok(serde_urlencoded::to_string(query).unwrap())
    }

    /// Get access token
    pub fn get_access_token(&mut self) -> Result<(), AuthError> {
        let request = reqwest::blocking::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?
            .post(TOKEN_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(self.get_token_query_string()?);

        let response = request.send()?;
        let success = response.status().is_success();
        let body = response.text()?;
        self.handle_response(success, &body)
    }

    /// Refresh the token (async)
    pub async fn get_access_token_async(&mut self) -> Result<(), AuthError> {
        let request = reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?
            .post(TOKEN_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(self.get_token_query_string()?);

        let response = request.send().await?;
        let success = response.status().is_success();
        let body = response.text().await?;
        self.handle_response(success, &body)
    }

    /// Handle a repsonse for get_access_token()
    pub fn handle_response(&mut self, success: bool, body: &str) -> Result<(), AuthError> {
        if success {
            match serde_json::from_str::<Token>(body) {
                Ok(result) => {
                    self.token = Some(TokenWrapper::new(result));
                    Ok(())
                }
                Err(e) => Err(AuthError::InvalidResponse(e.to_string())),
            }
        } else {
            Err(AuthError::UnknownError)
        }
    }

    /// Get a token reference
    pub fn token(&self) -> Option<&TokenWrapper> {
        self.token.as_ref()
    }

    pub fn get_token_refresh_query_string(&self) -> Result<String, AuthError> {
        #[derive(Serialize, Debug)]
        struct TokenRequest {
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            code: String,
            code_verifier: String,
            grant_type: String,
            refresh_token: String,
        }

        if self.auth_code.is_none() {
            return Err(AuthError::AuthNotPresent);
        }
        if self.token.is_none() {
            return Err(AuthError::TokenNotPresent);
        }

        let query = TokenRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: self.auth_code.as_ref().unwrap().clone(),
            code_verifier: self.challenge.clone(),
            grant_type: "refresh_token".to_string(),
            refresh_token: self.token().unwrap().token.refresh_token.clone(),
        };

        Ok(serde_urlencoded::to_string(query).unwrap())
    }

    /// Refresh the token
    pub fn refresh(&mut self) -> Result<(), AuthError> {
        let request = reqwest::blocking::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?
            .post(TOKEN_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(self.get_token_refresh_query_string()?);

        let response = request.send()?;
        let success = response.status().is_success();
        let body = response.text()?;
        self.handle_response(success, &body)
    }

    /// Refresh the token (async)
    pub async fn refresh_async(&mut self) -> Result<(), AuthError> {
        let request = reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?
            .post(TOKEN_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(self.get_token_refresh_query_string()?);

        let response = request.send().await?;
        let success = response.status().is_success();
        let body = response.text().await?;
        self.handle_response(success, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_test() {
        // client_id
        let client_id = "f071ff1547728d5a0c6863e359ef3f61";

        // redirect_url
        let redirect_url = "127.0.0.1:7878";

        let auth = Auth::new(client_id, redirect_url);

        // construct auth url and open
        auth.get_auth_url().open().unwrap();

        // Get the redirect from the web browser
        // for now i'll use a localhost server

        // start redirect server and get auth code
        let mut auth = redirect_server::Server::new(auth).wait().unwrap();

        // get access token
        auth.get_access_token().unwrap();

        // get refresh token
        auth.refresh_token().unwrap();
        println!("{}", serde_json::to_string);
    }

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
