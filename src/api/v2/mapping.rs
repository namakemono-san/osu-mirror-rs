use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::db::models::{Beatmap, Beatmapset};

const OSU_PREVIEW_BASE: &str = "//b.ppy.sh/preview";
const OSU_ASSETS_BASE_URL: &str = "https://assets.ppy.sh/beatmaps";
const OSU_BEATMAP_URL: &str = "https://osu.ppy.sh/beatmaps";

#[derive(Debug, Clone, Serialize)]
pub struct BeatmapV2 {
    pub beatmapset_id: i64,
    pub difficulty_rating: f64,
    pub id: i64,
    pub mode: String,
    pub status: String,
    pub total_length: i32,
    pub user_id: i64,
    pub version: String,
    pub accuracy: f64,
    pub ar: f64,
    pub bpm: f64,
    pub convert: bool,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub cs: f64,
    pub deleted_at: Option<DateTime<Utc>>,
    pub drain: f64,
    pub hit_length: i32,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    pub passcount: Option<i32>,
    pub playcount: i32,
    pub ranked: i32,
    pub url: String,
    pub checksum: Option<String>,
    pub mode_int: i32,
    pub max_combo: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoversV2 {
    pub cover: String,
    #[serde(rename = "cover@2x")]
    pub cover_2x: String,
    pub card: String,
    #[serde(rename = "card@2x")]
    pub card_2x: String,
    pub list: String,
    #[serde(rename = "list@2x")]
    pub list_2x: String,
    pub slimcover: String,
    #[serde(rename = "slimcover@2x")]
    pub slimcover_2x: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailabilityV2 {
    pub download_disabled: bool,
    pub more_information: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BeatmapsetV2 {
    pub anime_cover: bool,
    pub artist: String,
    pub artist_unicode: String,
    pub covers: CoversV2,
    pub creator: String,
    pub favourite_count: i32,
    pub genre_id: Option<i32>,
    pub hype: Option<serde_json::Value>,
    pub id: i64,
    pub language_id: Option<i32>,
    pub nsfw: bool,
    pub offset: i32,
    pub play_count: i32,
    pub preview_url: String,
    pub source: Option<String>,
    pub spotlight: bool,
    pub status: String,
    pub title: String,
    pub title_unicode: String,
    pub track_id: Option<i64>,
    pub user_id: Option<i64>,
    pub video: bool,
    pub bpm: f64,
    pub can_be_hyped: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub is_scoreable: bool,
    pub last_updated: Option<DateTime<Utc>>,
    pub legacy_thread_url: Option<String>,
    pub nominations_summary: Option<serde_json::Value>,
    pub ranked: i32,
    pub ranked_date: Option<DateTime<Utc>>,
    pub rating: Option<f64>,
    pub storyboard: bool,
    pub submitted_date: Option<DateTime<Utc>>,
    pub tags: Option<String>,
    pub availability: AvailabilityV2,
    pub beatmaps: Vec<BeatmapV2>,
    pub pack_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchMetaV2 {
    pub sort: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CursorV2 {
    pub approved_date: Option<i64>,
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResponseV2 {
    pub beatmapsets: Vec<BeatmapsetV2>,
    pub search: SearchMetaV2,
    pub recommended_difficulty: Option<f64>,
    pub error: Option<String>,
    pub total: i64,
    pub cursor: Option<CursorV2>,
    #[serde(rename = "cursor_string")]
    pub cursor_string: Option<String>,
}

fn status_to_ranked_int(status: &str) -> i32 {
    match status {
        "graveyard" => -2,
        "wip" => -1,
        "pending" => 0,
        "ranked" => 1,
        "approved" => 2,
        "qualified" => 3,
        "loved" => 4,
        _ => 0,
    }
}

fn normalize_status(status: &str) -> String {
    match status {
        "approved" | "ranked" => "ranked".to_string(),
        "qualified" => "qualified".to_string(),
        "loved" => "loved".to_string(),
        "pending" | "wip" => "pending".to_string(),
        "graveyard" => "graveyard".to_string(),
        _ => "pending".to_string(),
    }
}

fn is_scoreable(status: &str) -> bool {
    matches!(status, "ranked" | "approved" | "qualified" | "loved")
}

fn covers_for(id: i64) -> CoversV2 {
    let base = format!("{}/{}/covers", OSU_ASSETS_BASE_URL, id);
    CoversV2 {
        cover: format!("{}/cover.jpg", base),
        cover_2x: format!("{}/cover@2x.jpg", base),
        card: format!("{}/card.jpg", base),
        card_2x: format!("{}/card@2x.jpg", base),
        list: format!("{}/list.jpg", base),
        list_2x: format!("{}/list@2x.jpg", base),
        slimcover: format!("{}/slimcover.jpg", base),
        slimcover_2x: format!("{}/slimcover@2x.jpg", base),
    }
}

fn map_diff(set: &Beatmapset, map: &Beatmap) -> BeatmapV2 {
    let bpm = map.bpm.or(set.bpm).unwrap_or(0.0);
    let total_length = map.total_length.unwrap_or(map.hit_length.unwrap_or(0));
    let hit_length = map.hit_length.unwrap_or(0);
    let ranked = status_to_ranked_int(&set.status);
    let status = normalize_status(&set.status);
    BeatmapV2 {
        beatmapset_id: map.beatmapset_id,
        difficulty_rating: map.difficulty_rating.unwrap_or(0.0),
        id: map.id,
        mode: map.mode.clone(),
        status,
        total_length,
        user_id: set.creator_id.unwrap_or(0),
        version: map.version.clone(),
        accuracy: map.accuracy.unwrap_or(0.0),
        ar: map.ar.unwrap_or(0.0),
        bpm,
        convert: false,
        count_circles: map.count_circles.unwrap_or(0),
        count_sliders: map.count_sliders.unwrap_or(0),
        count_spinners: map.count_spinners.unwrap_or(0),
        cs: map.cs.unwrap_or(0.0),
        deleted_at: None,
        drain: map.drain.unwrap_or(0.0),
        hit_length,
        is_scoreable: is_scoreable(&set.status),
        last_updated: map.updated_at,
        passcount: None,
        playcount: set.play_count,
        ranked,
        url: format!("{}/{}", OSU_BEATMAP_URL, map.id),
        checksum: map.checksum.clone(),
        mode_int: map.mode_int,
        max_combo: map.max_combo.unwrap_or(0),
    }
}

pub fn map_set_v2(set: Beatmapset) -> BeatmapsetV2 {
    let artist_unicode = set
        .artist_unicode
        .clone()
        .unwrap_or_else(|| set.artist.clone());
    let title_unicode = set
        .title_unicode
        .clone()
        .unwrap_or_else(|| set.title.clone());
    let bpm = set.bpm.unwrap_or(0.0);
    let ranked = status_to_ranked_int(&set.status);
    let status = normalize_status(&set.status);
    let preview_url = format!("{}/{}.mp3", OSU_PREVIEW_BASE, set.id);
    let availability = AvailabilityV2 {
        download_disabled: set.availability_download_disabled,
        more_information: None,
    };
    let beatmaps_vec = if let Some(ref maps) = set.beatmaps {
        maps.iter().map(|b| map_diff(&set, b)).collect()
    } else {
        Vec::new()
    };
    BeatmapsetV2 {
        anime_cover: false,
        artist: set.artist,
        artist_unicode,
        covers: covers_for(set.id),
        creator: set.creator,
        favourite_count: set.favourite_count,
        genre_id: set.genre_id,
        hype: None,
        id: set.id,
        language_id: set.language_id,
        nsfw: set.nsfw,
        offset: 0,
        play_count: set.play_count,
        preview_url,
        source: set.source,
        spotlight: false,
        status,
        title: set.title,
        title_unicode,
        track_id: None,
        user_id: set.creator_id,
        video: set.video,
        bpm,
        can_be_hyped: false,
        deleted_at: None,
        discussion_enabled: false,
        discussion_locked: false,
        is_scoreable: is_scoreable(&set.status),
        last_updated: set.last_updated,
        legacy_thread_url: None,
        nominations_summary: None,
        ranked,
        ranked_date: set.ranked_date,
        rating: set.rating,
        storyboard: set.storyboard,
        submitted_date: set.submitted_date,
        tags: set.tags,
        availability,
        beatmaps: beatmaps_vec,
        pack_tags: Vec::new(),
    }
}
