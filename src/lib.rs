/// Authorization
pub mod auth;

/// API request functions
pub mod api;

/// UI
pub mod ui;

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
}
