use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Beatmapset {
    pub id: i64,
    pub title: String,
    pub title_unicode: Option<String>,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub creator: String,

    pub creator_id: Option<i64>,
    pub genre_id: Option<i32>,
    pub language_id: Option<i32>,
    pub rating: Option<f64>,

    pub source: Option<String>,
    pub tags: Option<String>,
    pub status: String,
    pub ranked_date: Option<DateTime<Utc>>,
    pub submitted_date: Option<DateTime<Utc>>,
    pub last_updated: Option<DateTime<Utc>>,
    pub bpm: Option<f64>,
    pub video: bool,
    pub storyboard: bool,
    pub nsfw: bool,
    pub favourite_count: i32,
    pub play_count: i32,
    pub availability_download_disabled: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    #[sqlx(skip)]
    pub beatmaps: Option<Vec<Beatmap>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Beatmap {
    pub id: i64,
    pub beatmapset_id: i64,
    pub version: String,
    pub mode: String,
    pub mode_int: i32,
    pub difficulty_rating: Option<f64>,
    pub ar: Option<f64>,
    pub cs: Option<f64>,
    pub drain: Option<f64>,
    pub accuracy: Option<f64>,
    pub bpm: Option<f64>,
    pub total_length: Option<i32>,
    pub hit_length: Option<i32>,
    pub max_combo: Option<i32>,
    pub count_circles: Option<i32>,
    pub count_sliders: Option<i32>,
    pub count_spinners: Option<i32>,
    pub checksum: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
