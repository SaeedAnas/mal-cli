use super::objects::*;
use super::Error;
use super::{get, handle_response, API_URL};
use crate::auth::Auth;
use serde::Serialize;

/// Get Anime List Request
#[derive(Clone, Debug, Serialize)]
pub struct GetAnimeListQuery {
    pub q: String,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub fn get_anime_list(
    query: &GetAnimeListQuery,
    auth: &Auth,
) -> Result<PageableData<Vec<Node<Anime>>>, Error> {
    let response = get(
        &format!("{}/anime?{}", API_URL, serde_urlencoded::to_string(query)?),
        auth,
    )?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetAnimeDetailsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    pub nsfw: bool,
}

pub fn get_anime_details(
    anime_id: u64,
    query: &GetAnimeDetailsQuery,
    auth: &Auth,
) -> Result<Anime, Error> {
    let response = get(
        &format!(
            "{}/anime/{}?{}",
            API_URL,
            anime_id,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetAnimeRankingQuery {
    pub ranking_type: AnimeRankingType,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub fn get_anime_ranking(
    query: &GetAnimeRankingQuery,
    auth: &Auth,
) -> Result<PageableData<Vec<RankingAnimePair>>, Error> {
    let response = get(
        &format!(
            "{}/anime/ranking?{}",
            API_URL,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetSeasonalAnimeQuery {
    pub sort: Option<SortStyle>,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub fn get_seasonal_anime(
    season: &AnimeSeason,
    query: &GetSeasonalAnimeQuery,
    auth: &Auth,
) -> Result<PageableData<Vec<Node<Anime>>>, Error> {
    let season_name: &'static str = season.season.clone().into();
    let response = get(
        &format!(
            "{}/anime/season/{}/{}?{}",
            API_URL,
            season.year,
            season_name,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetSuggestedAnimeQuery {
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub fn get_suggested_anime(
    query: &GetSuggestedAnimeQuery,
    auth: &Auth,
) -> Result<PageableData<Vec<Node<Anime>>>, Error> {
    let response = get(
        &format!(
            "{}/anime/suggestions?{}",
            API_URL,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )?;
    handle_response(&response)
}
