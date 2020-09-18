//! Rmal is a wrapper for the myanimelist api
#[cfg(feature = "blocking")]
pub mod blocking;
pub mod client;
pub mod model;
pub mod oauth2;
pub mod senum;
pub mod util;
