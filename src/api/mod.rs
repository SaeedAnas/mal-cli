/// Anime API endpoints
pub mod anime;
pub use anime::*;
/// User animelist API endpoints
pub mod animelist;
pub use animelist::*;
/// manga API endpoints
pub mod manga;
pub use manga::*;
/// User mangalist API endpoints
pub mod mangalist;
pub use mangalist::*;
/// API objects
pub mod model;
/// User API endpoints
pub mod user;
pub use user::*;

use crate::auth::OAuth;
use serde::{Deserialize, Serialize};

pub const API_URL: &str = "https://api.myanimelist.net/v2";

#[derive(Debug)]
pub enum Error {
    NoAuth,
    TimedOut,
    Unknown,
    NoBody,
    ParseError(serde_json::Error),
    QuerySerializeError(serde_urlencoded::ser::Error),
    HttpError(reqwest::StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Error::TimedOut
        } else {
            Error::Unknown
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseError(e)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(e: serde_urlencoded::ser::Error) -> Self {
        Error::QuerySerializeError(e)
    }
}

#[derive(Debug)]
pub(crate) struct ApiResponse {
    status: reqwest::StatusCode,
    body: Option<String>,
}

pub(crate) fn apply_headers(
    req: reqwest::blocking::RequestBuilder,
    auth: &OAuth,
) -> Result<reqwest::blocking::RequestBuilder, Error> {
    let access_token = match auth.token() {
        Some(token) => &token.token.access_token,
        None => return Err(Error::NoAuth),
    };
    Ok(req
        .header(reqwest::header::ACCEPT, "application/json")
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token),
        ))
}

pub(crate) fn send(
    request: reqwest::blocking::RequestBuilder,
    auth: &OAuth,
) -> Result<ApiResponse, Error> {
    let request = apply_headers(request, auth)?;
    let response = request.send()?;
    let status = response.status();
    Ok(ApiResponse {
        status,
        body: if let Ok(body) = response.text() {
            Some(body)
        } else {
            None
        },
    })
}

pub(crate) fn get<U: reqwest::IntoUrl>(url: U, auth: &OAuth) -> Result<ApiResponse, Error> {
    let request = reqwest::blocking::ClientBuilder::new()
        .user_agent(auth.user_agent())
        .build()?
        .get(url);
    send(request, auth)
}

pub(crate) fn patch<U: reqwest::IntoUrl, B: Serialize>(
    url: U,
    auth: &OAuth,
    body: &B,
) -> Result<ApiResponse, Error> {
    let request = reqwest::blocking::ClientBuilder::new()
        .user_agent(auth.user_agent())
        .build()?
        .patch(url)
        .body(serde_urlencoded::to_string(body)?);
    send(request, auth)
}

pub(crate) fn delete<U: reqwest::IntoUrl>(url: U, auth: &OAuth) -> Result<ApiResponse, Error> {
    let request = reqwest::blocking::ClientBuilder::new()
        .user_agent(auth.user_agent())
        .build()?
        .delete(url);
    send(request, auth)
}

pub(crate) fn handle_response<'a, D: Deserialize<'a>>(res: &'a ApiResponse) -> Result<D, Error> {
    if !res.status.is_success() {
        return Err(Error::HttpError(res.status));
    }
    if let Some(body) = &res.body {
        Ok(serde_json::from_str::<D>(&body)?)
    } else {
        Err(Error::NoBody)
    }
}
