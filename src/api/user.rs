use super::objects::*;
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
mod test {
    use super::*;
}
