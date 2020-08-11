use super::objects::*;
use super::Error;
use super::{delete, get, handle_response, patch, API_URL};
use crate::auth::Auth;
use serde::Serialize;

/// Update specified anime in animelist
#[derive(Clone, Debug, Serialize)]
pub struct UpdateUserAnimeListStatusQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserWatchStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rewatching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_watched_episodes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_times_rewatched: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewatch_value: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

pub fn update_anime_list_status(
    anime_id: u64,
    update: &UpdateUserAnimeListStatusQuery,
    auth: &Auth,
) -> Result<UserAnimeListStatus, Error> {
    let response = patch(
        &format!("{}/anime/{}/my_list_status", API_URL, anime_id,),
        auth,
        update,
    )?;
    handle_response(&response)
}

pub fn delete_anime_from_list(anime_id: u64, auth: &Auth) -> Result<(), Error> {
    let response = delete(
        &format!("{}/anime/{}/my_list_status", BASE_URL, anime_id),
        auth,
    )?;
    if response.status.is_success() {
        Ok(())
    } else {
        Err(Error::HttpError(response.status))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GetUserAnimeListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortStyle>,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
}

pub fn get_user_anime_list<U: ToString>(
    user: U,
    query: &GetUserAnimeListQuery,
    auth: &Auth,
) -> Result<PageableData<Vec<Node<Anime>>>, Error> {
    let response = get(
        &format!(
            "{}/users/{}/animelist?{}",
            API_URL,
            user.to_string(),
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )?;
    handle_response(&response)
}