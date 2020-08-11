/// Anime related structs
pub mod anime;
pub use anime::*;
/// Manga related structs
pub mod manga;
pub use manga::*;
/// User related structs
pub mod user;
pub use user::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Debug;
use std::str::FromStr;
use strum::AsStaticRef;
use strum_macros::{AsStaticStr, EnumString, IntoStaticStr};
use time::{Date, PrimitiveDateTime, Time};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Paging {
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageableData<D: Clone + Debug> {
    pub data: D,
    pub paging: Paging,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node<N: Clone + std::fmt::Debug> {
    pub node: N,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Picture {
    pub large: Option<String>,
    pub medium: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlternativeTitles {
    pub synonyms: Option<Vec<String>>,
    pub en: Option<String>,
    pub jp: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
    Other(String),
}

#[derive(Clone, Debug)]
pub struct TimeWrapper {
    pub time: Time,
}
#[derive(Clone, Debug)]
pub struct DateWrapper {
    pub date: Date,
}
#[derive(Clone, Debug)]
pub struct DateTimeWrapper {
    pub datetime: PrimitiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Broadcast {
    pub day_of_the_week: String,
    pub start_time: Option<TimeWrapper>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Studio {
    pub id: u64,
    pub name: String,
}

pub const ALL_ANIME_AND_MANGA_FIELDS: &str = "id,title,main_picture,alternative_titles,start_date,end_date,synopsis,mean,rank,popularity,num_list_users,num_scoring_users,nsfw,genres,create_at,updated_at,media_type,status,my_list_status,num_episodes,broadcast,source,average_episode_duration,rating,pictures,background,related_anime,related_manga,recommendations,studios,statistics,num_volumes,num_chapters,authors";
pub const ALL_USER_FIELDS: &str =
    "id,name,picture,gender,birthday,location,joined_at,anime_statistics,time_zone,is_supporter";

/// Utility to convert a list of fields to a string (in the format expected by query objects)
pub fn fields_to_string(fields: &[AnimeField]) -> String {
    fields
        .iter()
        .map(|field| field.into())
        .collect::<Vec<&str>>()
        .join(",")
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum NSFW {
    White,
    Gray,
    Black,
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingInfo {
    pub rank: u64,
    pub previous_rank: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    id: u64,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonRole {
    node: Person,
    role: String,
}

macro_rules! impl_serialize_deserialize {
    (for $( $t:ty ),+) => {
        $(
        impl Serialize for $t {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.into())
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match Self::from_str(s.as_str()) {
                    Ok(n) => Ok(n),
                    Err(_) => Ok(Self::Other(s)),
                }
            }

            fn deserialize_in_place<D>(
                deserializer: D,
                place: &mut Self,
            ) -> Result<(), <D as Deserializer<'de>>::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                *place = match Self::from_str(s.as_str()) {
                    Ok(n) => n,
                    Err(_) => Self::Other(s),
                };
                Ok(())
            }
        }
        )*
    };
}

impl_serialize_deserialize!(
    for
    NSFW,
    AnimeMediaType,
    AnimeStatus,
    UserWatchStatus,
    AnimeRankingType,
    MangaRankingType,
    Season,
    UserStatus,
    SortStyle,
    UserReadStatus,
    MangaMediaType,
    MangaStatus
);

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Self::from_str(s.as_str()) {
            Ok(n) => Ok(n),
            Err(_) => Ok(Self::Other),
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        *place = match Self::from_str(s.as_str()) {
            Ok(n) => n,
            Err(_) => Self::Other,
        };
        Ok(())
    }
}

impl Serialize for TimeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.time.format("%H:%M:%S"))
    }
}

impl<'de> Deserialize<'de> for TimeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        let re = regex::Regex::new(r"([0-9]+):([0-9]+)").unwrap();
        if let Some(caps) = re.captures(&s) {
            let hour = caps.get(1).unwrap();
            let minute = caps.get(2).unwrap();
            let hour = match hour.as_str().parse::<u8>() {
                Ok(hour) => hour,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let minute = match minute.as_str().parse::<u8>() {
                Ok(minute) => minute,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let time = match Time::try_from_hms(hour, minute, 0) {
                Ok(time) => time,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(TimeWrapper { time })
        } else {
            Err(D::Error::custom("Could not parse time"))
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        match Time::parse(&s, "%H:%M:%S") {
            Ok(time) => {
                place.time = time;
                return Ok(());
            }
            Err(_) => (),
        };
        let re = regex::Regex::new(r"([0-9]+):([0-9]+)").unwrap();
        if let Some(caps) = re.captures(&s) {
            let hour = caps.get(1).unwrap();
            let minute = caps.get(2).unwrap();
            let hour = match hour.as_str().parse::<u8>() {
                Ok(hour) => hour,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let minute = match minute.as_str().parse::<u8>() {
                Ok(minute) => minute,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            place.time = match Time::try_from_hms(hour, minute, 0) {
                Ok(time) => time,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(())
        } else {
            Err(D::Error::custom("Could not parse time"))
        }
    }
}

impl Serialize for DateWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.date.format("%Y-%m-%d"))
    }
}

impl<'de> Deserialize<'de> for DateWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        match Date::parse(&s, "%Y-%m-%d") {
            Ok(date) => return Ok(DateWrapper { date }),
            Err(_) => (),
        };
        let re = regex::Regex::new(r"([0-9]+)-?([0-9]+)?").unwrap();
        if let Some(caps) = re.captures(&s) {
            let year = caps.get(1).unwrap();
            let year = match year.as_str().parse::<i32>() {
                Ok(year) => year,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let month = if let Some(month) = caps.get(2) {
                match month.as_str().parse::<u8>() {
                    Ok(month) => month,
                    Err(e) => return Err(serde::de::Error::custom(e.to_string())),
                }
            } else {
                1
            };
            let date = match Date::try_from_ymd(year, month, 1) {
                Ok(date) => date,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(DateWrapper { date })
        } else {
            Err(D::Error::custom("Could not parse date"))
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        match Date::parse(&s, "%Y-%m-%d") {
            Ok(date) => {
                place.date = date;
                return Ok(());
            }
            Err(_) => (),
        };
        let re = regex::Regex::new(r"([0-9]+)-?([0-9]+)?").unwrap();
        if let Some(caps) = re.captures(&s) {
            let year = caps.get(1).unwrap();
            let year = match year.as_str().parse::<i32>() {
                Ok(year) => year,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let month = if let Some(month) = caps.get(2) {
                match month.as_str().parse::<u8>() {
                    Ok(month) => month,
                    Err(e) => return Err(serde::de::Error::custom(e.to_string())),
                }
            } else {
                1
            };
            place.date = match Date::try_from_ymd(year, month, 1) {
                Ok(date) => date,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(())
        } else {
            Err(D::Error::custom("Could not parse date"))
        }
    }
}

impl Serialize for DateTimeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.datetime.format("%Y-%m-%dT%H:%M:%S"))
    }
}

impl<'de> Deserialize<'de> for DateTimeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        match PrimitiveDateTime::parse(&s, "%Y-%m-%dT%H:%M:%S") {
            Ok(datetime) => Ok(DateTimeWrapper { datetime }),
            Err(e) => Err(D::Error::custom(e.to_string())),
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        match PrimitiveDateTime::parse(&s, "%Y-%m-%dT%H:%M:%S") {
            Ok(datetime) => {
                place.datetime = datetime;
                Ok(())
            }
            Err(e) => Err(D::Error::custom(e.to_string())),
        }
    }
}
