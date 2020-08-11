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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnimeSeason {
    pub year: u64,
    pub season: Season,
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

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeField {
    Id,
    Titel,
    MainPicture,
    AlternativeTitles,
    StartDate,
    EndDate,
    Synopsis,
    Mean,
    Rank,
    Popularity,
    NumListUsers,
    NumScoringUsers,
    NSFW,
    CreatedAt,
    UpdatedAt,
    MediaType,
    Status,
    MyListStatus,
    NumEpisodes,
    Broadcast,
    Source,
    AverageEpisodeDuration,
    Rating,
    Pictures,
    Background,
    RelatedAnime,
    RelatedManga,
    Recommendations,
    Studios,
    Statistics,

    NumVolumes,
    NumChapters,
    Authors,

    Name,
    Picture,
    Gender,
    Birthday,
    Location,
    JoinedAt,
    AnimeStatistics,
    TimeZone,
    IsSupporter,
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

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeMediaType {
    Unknown,
    #[strum(serialize = "tv")]
    TV,
    #[strum(serialize = "ova")]
    OVA,
    Movie,
    Special,
    #[strum(serialize = "ona")]
    ONA,
    Music,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeStatus {
    FinishedAiring,
    CurrentlyAiring,
    NotYetAired,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum UserWatchStatus {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Source {
    Other,
    Original,
    Manga,
    #[strum(serialize = "4_koma_manga")]
    YonKomaManga,
    WebManga,
    DigitalManga,
    Novel,
    LightNovel,
    VisualNovel,
    Game,
    CardGame,
    Book,
    PictureBook,
    Radio,
    Music,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAnimeListStatus {
    pub status: UserWatchStatus,
    pub score: u8,
    pub num_episodes_watched: u64,
    pub is_rewatching: bool,
    pub start_date: Option<DateWrapper>,
    pub finish_date: Option<DateWrapper>,
    pub priority: Option<u8>,
    pub num_times_rewatched: Option<u64>,
    pub rewatch_value: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub comments: Option<String>,
    pub updated_at: DateTimeWrapper,
}

#[derive(Clone, Debug, EnumString, IntoStaticStr)]
pub enum Rating {
    G,
    #[strum(serialize = "pg")]
    PG,
    #[strum(serialize = "pg_13")]
    PG13,
    R,
    #[strum(serialize = "r+")]
    Rp,
    #[strum(serialize = "rx")]
    RX,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Anime {
    pub id: u64,
    pub title: String,
    pub main_picture: Picture,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<DateWrapper>,
    pub end_date: Option<DateWrapper>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u64>,
    pub popularity: Option<u64>,
    pub num_list_users: Option<u64>,
    pub num_scoring_users: Option<u64>,
    pub nsfw: Option<NSFW>,
    pub genres: Option<Vec<Genre>>,
    pub created_at: Option<DateTimeWrapper>,
    pub updated_at: Option<DateTimeWrapper>,
    pub media_type: Option<AnimeMediaType>,
    pub status: Option<AnimeStatus>,
    pub my_list_status: Option<UserAnimeListStatus>,
    pub num_episodes: Option<u64>,
    pub start_season: Option<Season>,
    pub broadcast: Option<Broadcast>,
    pub source: Option<Source>,
    pub average_episode_duration: Option<u64>,
    pub rating: Option<String>,
    pub studios: Option<Vec<Studio>>,
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
}

impl Anime {
    pub fn summary(&self) -> String {
        let title = &self.title;
        let score = self.my_list_status.as_ref().unwrap().score;
        let anime_type: &'static str = self.media_type.as_ref().unwrap().as_static().clone();
        let progress = &self.my_list_status.as_ref().unwrap().num_episodes_watched;
        let total = &self.num_episodes.as_ref().unwrap();
        format!(
            "title: {}, score: {}, type: {}, progress {}/{}",
            title, score, anime_type, progress, total
        )
    }
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeRankingType {
    All,
    Airing,
    Upcoming,
    #[strum(serialize = "tv")]
    TV,
    #[strum(serialize = "ova")]
    OVA,
    Movie,
    Special,
    #[strum(serialize = "bypopularity")]
    ByPopularity,
    Favorite,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum MangaRankingType {
    All,
    Manga,
    Novels,
    #[strum(serialize = "oneshots")]
    OneShots,
    Doujinshi,
    Manhwa,
    Manhua,
    #[strum(serialize = "bypopularity")]
    ByPopularity,
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingInfo {
    pub rank: u64,
    pub previous_rank: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingAnimePair {
    pub node: Anime,
    pub ranking: RankingInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingMangaPair {
    pub node: Manga,
    pub ranking: RankingInfo,
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum UserStatus {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum SortStyle {
    ListScore,
    ListUpdatedAt,
    AnimeTitle,
    AnimeStartDate,
    AnimeId,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum MangaMediaType {
    Unknown,
    Manga,
    Novel,
    OneShot,
    Doujinshi,
    Manhwa,
    Manhua,
    #[strum(serialize = "oel")]
    OEL,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum MangaStatus {
    Finished,
    CurrentlyPublishing,
    NotYetPublished,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum UserReadStatus {
    Reading,
    Completed,
    OnHold,
    Dropped,
    PlanToRead,
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserMangaListStatus {
    pub status: UserReadStatus,
    pub score: u8,
    pub num_volumes_read: u64,
    pub num_chapters_read: u64,
    pub is_rereading: bool,
    pub start_date: Option<DateWrapper>,
    pub finish_date: Option<DateWrapper>,
    pub priority: Option<u8>,
    pub num_times_reread: Option<u64>,
    pub reread_value: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub comments: Option<String>,
    pub updated_at: DateTimeWrapper,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manga {
    pub id: u64,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<DateWrapper>,
    pub end_date: Option<DateWrapper>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u64>,
    pub popularity: Option<u64>,
    pub num_list_users: Option<u64>,
    pub num_scoring_users: Option<u64>,
    pub nsfw: Option<NSFW>,
    pub genres: Option<Vec<Genre>>,
    pub created_at: Option<DateTimeWrapper>,
    pub updated_at: Option<DateTimeWrapper>,
    pub media_type: Option<MangaMediaType>,
    pub status: Option<MangaStatus>,
    pub my_list_status: Option<UserMangaListStatus>,
    pub num_volumes: Option<u64>,
    pub num_chapters: Option<u64>,
    pub authors: Option<Vec<PersonRole>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnimeStatistics {
    pub num_items_watching: u64,
    pub num_items_completed: u64,
    pub num_items_on_hold: u64,
    pub num_items_dropped: u64,
    pub num_items_plan_to_watch: u64,
    pub num_items: u64,
    pub num_days_watched: f64,
    pub num_days_watching: f64,
    pub num_days_completed: f64,
    pub num_days_on_hold: f64,
    pub num_days_dropped: f64,
    pub num_days: f64,
    pub num_episodes: u64,
    pub num_times_rewatched: u64,
    pub mean_score: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub name: String,
    pub picture: String,
    pub gender: Option<String>,
    pub birthday: Option<DateWrapper>,
    pub location: Option<String>,
    pub joined_at: DateTimeWrapper,
    pub anime_statistics: Option<AnimeStatistics>,
    pub time_zone: Option<String>,
    pub is_supporter: Option<bool>,
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
