use super::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Debug;
use std::str::FromStr;
use strum::AsStaticRef;
use strum_macros::{AsStaticStr, EnumString, IntoStaticStr};
use time::{Date, PrimitiveDateTime, Time};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnimeSeason {
    pub year: u64,
    pub season: Season,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingAnimePair {
    pub node: Anime,
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
