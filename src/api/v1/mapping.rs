use crate::db::models::{Beatmap, Beatmapset};
use serde::Serialize;

fn format_dt(dt: chrono::DateTime<chrono::Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn status_to_approved(status: &str) -> i32 {
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

#[derive(Serialize)]
pub struct BeatmapV1 {
    pub beatmapset_id: String,
    pub beatmap_id: String,
    pub approved: String,
    pub total_length: String,
    pub hit_length: String,
    pub version: String,
    pub file_md5: String,
    pub diff_size: String,
    pub diff_overall: String,
    pub diff_approach: String,
    pub diff_drain: String,
    pub mode: String,
    pub count_normal: String,
    pub count_slider: String,
    pub count_spinner: String,
    pub submit_date: Option<String>,
    pub approved_date: Option<String>,
    pub last_update: Option<String>,
    pub artist: String,
    pub artist_unicode: String,
    pub title: String,
    pub title_unicode: String,
    pub creator: String,
    pub creator_id: String,
    pub bpm: String,
    pub source: String,
    pub tags: String,
    pub genre_id: String,
    pub language_id: String,
    pub favourite_count: String,
    pub rating: String,
    pub storyboard: String,
    pub video: String,
    pub download_unavailable: String,
    pub audio_unavailable: String,
    pub playcount: String,
    pub passcount: String,
    pub packs: Option<String>,
    pub max_combo: String,
    pub diff_aim: Option<String>,
    pub diff_speed: Option<String>,
    pub difficultyrating: String,
}

impl BeatmapV1 {
    pub fn from_models(set: &Beatmapset, map: &Beatmap) -> Self {
        let approved = status_to_approved(&set.status).to_string();

        let total_len = map.total_length.or(map.hit_length).unwrap_or(0);

        let hit_len = map.hit_length.unwrap_or(0);

        let artist_unicode = set
            .artist_unicode
            .clone()
            .unwrap_or_else(|| set.artist.clone());

        let title_unicode = set
            .title_unicode
            .clone()
            .unwrap_or_else(|| set.title.clone());

        let bpm = map.bpm.or(set.bpm).unwrap_or(0.0);

        let submit_date = set.submitted_date.map(format_dt);
        let approved_date = set.ranked_date.map(format_dt);
        let last_update = set.last_updated.map(format_dt);

        let storyboard = if set.storyboard { "1" } else { "0" }.to_string();
        let video = if set.video { "1" } else { "0" }.to_string();
        let download_unavailable = if set.availability_download_disabled {
            "1"
        } else {
            "0"
        }
        .to_string();

        let audio_unavailable = "0".to_string();

        let max_combo = map.max_combo.unwrap_or(0);

        let diff = map.difficulty_rating.unwrap_or(0.0);

        let count_normal = map.count_circles.unwrap_or(0);
        let count_slider = map.count_sliders.unwrap_or(0);
        let count_spinner = map.count_spinners.unwrap_or(0);

        let file_md5 = map.checksum.clone().unwrap_or_default();

        let source = set.source.clone().unwrap_or_default();
        let tags = set.tags.clone().unwrap_or_default();

        let favourite_count = set.favourite_count;
        let play_count = set.play_count;

        BeatmapV1 {
            beatmapset_id: set.id.to_string(),
            beatmap_id: map.id.to_string(),
            approved,
            total_length: total_len.to_string(),
            hit_length: hit_len.to_string(),
            version: map.version.clone(),
            file_md5,
            diff_size: map.cs.unwrap_or(0.0).to_string(),
            diff_overall: map.accuracy.unwrap_or(0.0).to_string(),
            diff_approach: map.ar.unwrap_or(0.0).to_string(),
            diff_drain: map.drain.unwrap_or(0.0).to_string(),
            mode: map.mode_int.to_string(),
            count_normal: count_normal.to_string(),
            count_slider: count_slider.to_string(),
            count_spinner: count_spinner.to_string(),
            submit_date,
            approved_date,
            last_update,
            artist: set.artist.clone(),
            artist_unicode,
            title: set.title.clone(),
            title_unicode,
            creator: set.creator.clone(),
            creator_id: set.creator_id.unwrap_or(0).to_string(),
            bpm: bpm.to_string(),
            source,
            tags,
            genre_id: set.genre_id.unwrap_or(0).to_string(),
            language_id: set.language_id.unwrap_or(0).to_string(),
            favourite_count: favourite_count.to_string(),
            rating: set.rating.unwrap_or(0.0).to_string(),
            storyboard,
            video,
            download_unavailable,
            audio_unavailable,
            playcount: play_count.to_string(),
            passcount: "0".to_string(),
            packs: None,
            max_combo: max_combo.to_string(),
            diff_aim: Some("0".to_string()),
            diff_speed: Some("0".to_string()),
            difficultyrating: diff.to_string(),
        }
    }
}
