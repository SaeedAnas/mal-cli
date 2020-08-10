use super::Auth;
use crate::config::*;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

pub fn cache_auth(auth: &Auth) {
    let auth_path = AppConfig::get_paths().unwrap().auth_cache_path;

    let mut auth_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(auth_path)
        .unwrap();

    let cached_auth = serde_json::to_string(auth).unwrap();

    write!(auth_file, "{}", cached_auth).unwrap();
}

pub fn load_cached_auth() -> Option<Auth> {
    let auth_path = AppConfig::get_paths().unwrap().auth_cache_path;

    let mut auth_file = OpenOptions::new().read(true).open(auth_path).unwrap();

    let mut cached_string = String::new();

    auth_file.read_to_string(&mut cached_string).unwrap();

    let cached_auth: Auth = match serde_json::from_str(&cached_string) {
        Ok(s) => s,
        Err(_) => return None,
    };

    Some(cached_auth)
}
