/// Anime API endpoints
/// User animelist API endpoints
/// manga API endpoints
/// User mangalist API endpoints
/// User API endpoints
/// Forum API endpoints
use crate::auth::Auth;
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

pub(crate) fn apply_general_headers(
    req: reqwest::blocking::RequestBuilder,
) -> reqwest::blocking::RequestBuilder {
    req.header(reqwest::header::ACCEPT, "application/json")
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
}

pub(crate) fn apply_auth_headers(
    req: reqwest::blocking::RequestBuilder,
    auth: &Auth,
) -> Result<reqwest::blocking::RequestBuilder, Error> {
    if let Some(token) = auth.token() {
        Ok(req.header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token.token.access_token),
        ))
    } else {
        Err(Error::NoAuth)
    }
}

pub(crate) fn get<U: reqwest::IntoUrl>(url: U, auth: &Auth) -> Result<ApiResponse, Error> {
    let request = reqwest::blocking::ClientBuilder::new()
        .user_agent(auth.user_agent())
        .build()?
        .get(url);
    let request = apply_auth_headers(apply_general_headers(request), auth)?;
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

pub(crate) fn patch<U: reqwest::IntoUrl, B: Serialize>(
    url: U,
    auth: &Auth,
    body: &B,
) -> Result<ApiResponse, Error> {
    let request = reqwest::blocking::ClientBuilder::new()
        .user_agent(auth.user_agent())
        .build()?
        .patch(url)
        .body(serde_urlencoded::to_string(body)?);
    let request = apply_auth_headers(apply_general_headers(request), auth)?;
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

pub(crate) fn delete<U: reqwest::IntoUrl>(url: U, auth: &Auth) -> Result<ApiResponse, Error> {
    let request = reqwest::blocking::ClientBuilder::new()
        .user_agent(auth.user_agent())
        .build()?
        .delete(url);
    let request = apply_auth_headers(apply_general_headers(request), auth)?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
