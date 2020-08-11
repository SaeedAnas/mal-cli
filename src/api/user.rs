use super::data::*;
use super::Error;
use super::{get, handle_response, API_URL};
use crate::auth::Auth;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct GetUserInformationQuery {
    pub fields: Option<String>,
}

pub fn get_my_user_information<U: ToString>(
    user: U,
    query: &GetUserInformationQuery,
    auth: &Auth,
) -> Result<UserInfo, Error> {
    let response = get(
        &format!(
            "{}/users/{}?{}",
            API_URL,
            user.to_string(),
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )?;
    handle_response(&response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_information() {
        let auth = crate::auth::tests::get_auth();
        let query = GetUserInformationQuery {
            fields: Some(ALL_USER_FIELDS.to_string()),
        };
        let result = get_my_user_information("@me", &query, &auth).unwrap();
        println!("{:#?}", result);
    }
}
