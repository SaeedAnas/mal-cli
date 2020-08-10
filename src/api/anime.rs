use super::Error;
use super::{get, handle_response, API_URL};
use crate::auth::Auth;
use serde::Serialize;

/// Get Anime List Request
#[derive(Clone, Debug, Serialize)]
pub struct GetAnimeListQuery {
    pub q: String,
    pub limit: u32,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub fn get_anime_list(query: &GetAnimeListQuery, auth: &Auth) -> Result<D> {
    let response = get(
        &format!("{}/anime?{}", API_URL, serde_urlencoded::to_string(query)?),
        auth,
    )?;
    handle_response(&response)
}
