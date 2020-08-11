use super::objects::*;
use super::Error;
use super::{get, handle_response, API_URL};
use crate::auth::Auth;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct GetMangaListQuery {
    pub q: String,
    pub limit: u64,
    pub offset: u64,
    pub nfsw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub fn get_manga_list(
    query: &GetMangaListQuery,
    auth: &Auth,
) -> Result<PageableData<Vec<Node<Manga>>>, Error> {
    let response = get(
        &format! {"{}/manga?{}", API_URL, serde_urlencoded::to_string(query)?},
        auth,
    )?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetMangaDetailQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    pub nsfw: bool,
}

pub fn get_manga_details(
    manga_id: u64,
    query: &GetMangaDetailQuery,
    auth: &Auth,
) -> Result<Manga, Error> {
    let response = get(
        &format!(
            "{}/manga/{}?{}",
            API_URL,
            manga_id,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetMangaRankingQuery {
    pub ranking_type: MangaRankingType,
    pub limut: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub fn get_manga_ranking(
    query: &GetMangaRankingQuery,
    auth: &Auth,
) -> Result<PageableData<Vec<RankingAnimePair>>, Error> {
    let response = get(
        &format!(
            "{}/manga/ranking?{}",
            API_URL,
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
