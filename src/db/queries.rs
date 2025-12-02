use super::models::{Beatmap, Beatmapset};
use crate::error::Result;
use sqlx::{PgPool, Row};

pub async fn upsert_beatmapset(pool: &PgPool, set: &Beatmapset) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO beatmapsets (
            id, title, title_unicode, artist, artist_unicode, creator,
            creator_id, genre_id, language_id, rating,
            source, tags, status, ranked_date, submitted_date, last_updated,
            bpm, video, storyboard, nsfw, favourite_count, play_count,
            availability_download_disabled, created_at, updated_at
        )
        VALUES (
            $1,$2,$3,$4,$5,$6,
            $7,$8,$9,$10,$11,
            $12,$13,$14,$15,$16,$17,
            $18,$19,$20,$21,$22,
            $23, NOW(), NOW()
        )
        ON CONFLICT(id) DO UPDATE SET
            title = EXCLUDED.title,
            title_unicode = EXCLUDED.title_unicode,
            artist = EXCLUDED.artist,
            artist_unicode = EXCLUDED.artist_unicode,
            creator = EXCLUDED.creator,
            creator_id = EXCLUDED.creator_id,
            genre_id = EXCLUDED.genre_id,
            language_id = EXCLUDED.language_id,
            rating = EXCLUDED.rating,
            source = EXCLUDED.source,
            tags = EXCLUDED.tags,
            status = EXCLUDED.status,
            ranked_date = EXCLUDED.ranked_date,
            submitted_date = EXCLUDED.submitted_date,
            last_updated = EXCLUDED.last_updated,
            bpm = EXCLUDED.bpm,
            video = EXCLUDED.video,
            storyboard = EXCLUDED.storyboard,
            nsfw = EXCLUDED.nsfw,
            favourite_count = EXCLUDED.favourite_count,
            play_count = EXCLUDED.play_count,
            availability_download_disabled = EXCLUDED.availability_download_disabled,
            updated_at = NOW()
        "#,
        set.id,
        set.title,
        set.title_unicode,
        set.artist,
        set.artist_unicode,
        set.creator,
        set.creator_id,
        set.genre_id,
        set.language_id,
        set.rating,
        set.source,
        set.tags,
        set.status,
        set.ranked_date,
        set.submitted_date,
        set.last_updated,
        set.bpm,
        set.video,
        set.storyboard,
        set.nsfw,
        set.favourite_count,
        set.play_count,
        set.availability_download_disabled
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn upsert_beatmap(pool: &PgPool, m: &Beatmap) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO beatmaps (
            id, beatmapset_id, version, mode, mode_int,
            difficulty_rating, ar, cs, drain, accuracy, bpm,
            total_length, hit_length, max_combo,
            count_circles, count_sliders, count_spinners, checksum,
            created_at, updated_at
        )
        VALUES (
            $1,$2,$3,$4,$5,
            $6,$7,$8,$9,$10,$11,
            $12,$13,$14,
            $15,$16,$17,$18,
            NOW(), NOW()
        )
        ON CONFLICT(id) DO UPDATE SET
            version = EXCLUDED.version,
            mode = EXCLUDED.mode,
            mode_int = EXCLUDED.mode_int,
            difficulty_rating = EXCLUDED.difficulty_rating,
            ar = EXCLUDED.ar,
            cs = EXCLUDED.cs,
            drain = EXCLUDED.drain,
            accuracy = EXCLUDED.accuracy,
            bpm = EXCLUDED.bpm,
            total_length = EXCLUDED.total_length,
            hit_length = EXCLUDED.hit_length,
            max_combo = EXCLUDED.max_combo,
            count_circles = EXCLUDED.count_circles,
            count_sliders = EXCLUDED.count_sliders,
            count_spinners = EXCLUDED.count_spinners,
            checksum = EXCLUDED.checksum,
            updated_at = NOW()
        "#,
        m.id,
        m.beatmapset_id,
        m.version,
        m.mode,
        m.mode_int,
        m.difficulty_rating,
        m.ar,
        m.cs,
        m.drain,
        m.accuracy,
        m.bpm,
        m.total_length,
        m.hit_length,
        m.max_combo,
        m.count_circles,
        m.count_sliders,
        m.count_spinners,
        m.checksum,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_beatmapset(pool: &PgPool, id: i64) -> Result<Option<Beatmapset>> {
    let row = sqlx::query!(
        r#"
        SELECT
            id, title, title_unicode, artist, artist_unicode, creator,
            creator_id, genre_id, language_id, rating,
            source, tags, status, ranked_date, submitted_date,
            last_updated, bpm, video, storyboard, nsfw,
            favourite_count, play_count, availability_download_disabled,
            created_at, updated_at
        FROM beatmapsets WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    let Some(r) = row else {
        return Ok(None);
    };

    let mut set = Beatmapset {
        id: r.id,
        title: r.title,
        title_unicode: r.title_unicode,
        artist: r.artist,
        artist_unicode: r.artist_unicode,
        creator: r.creator,
        creator_id: r.creator_id,
        genre_id: r.genre_id,
        language_id: r.language_id,
        rating: r.rating,
        source: r.source,
        tags: r.tags,
        status: r.status,
        ranked_date: r.ranked_date,
        submitted_date: r.submitted_date,
        last_updated: r.last_updated,
        bpm: r.bpm,
        video: r.video.unwrap_or(false),
        storyboard: r.storyboard.unwrap_or(false),
        nsfw: r.nsfw.unwrap_or(false),
        favourite_count: r.favourite_count.unwrap_or(0),
        play_count: r.play_count.unwrap_or(0),
        availability_download_disabled: r.availability_download_disabled.unwrap_or(false),
        created_at: r.created_at.unwrap_or_else(chrono::Utc::now),
        updated_at: r.updated_at.unwrap_or_else(chrono::Utc::now),
        beatmaps: None,
    };

    let rows = sqlx::query!(
        r#"
        SELECT
            id, beatmapset_id, version, mode, mode_int,
            difficulty_rating, ar, cs, drain, accuracy, bpm,
            total_length, hit_length, max_combo,
            count_circles, count_sliders, count_spinners,
            checksum, created_at, updated_at
        FROM beatmaps
        WHERE beatmapset_id = $1
        ORDER BY id ASC
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    set.beatmaps = Some(
        rows.into_iter()
            .map(|b| Beatmap {
                id: b.id,
                beatmapset_id: b.beatmapset_id,
                version: b.version,
                mode: b.mode,
                mode_int: b.mode_int,
                difficulty_rating: b.difficulty_rating,
                ar: b.ar,
                cs: b.cs,
                drain: b.drain,
                accuracy: b.accuracy,
                bpm: b.bpm,
                total_length: b.total_length,
                hit_length: b.hit_length,
                max_combo: b.max_combo,
                count_circles: b.count_circles,
                count_sliders: b.count_sliders,
                count_spinners: b.count_spinners,
                checksum: b.checksum,
                created_at: b.created_at.unwrap_or_else(chrono::Utc::now),
                updated_at: b.updated_at.unwrap_or_else(chrono::Utc::now),
            })
            .collect(),
    );

    Ok(Some(set))
}

pub async fn search_beatmapsets(
    pool: &PgPool,
    keyword: &str,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Beatmapset>> {
    let mut sql = String::from(
        r#"
        SELECT
            id, title, title_unicode, artist, artist_unicode, creator,
            creator_id, genre_id, language_id, rating,
            source, tags, status, ranked_date, submitted_date,
            last_updated, bpm, video, storyboard, nsfw,
            favourite_count, play_count, availability_download_disabled,
            created_at, updated_at
        FROM beatmapsets
        WHERE 1=1
        "#,
    );

    if !keyword.is_empty() {
        sql.push_str(
            " AND (title ILIKE $1 OR artist ILIKE $1 OR creator ILIKE $1 OR tags ILIKE $1)",
        );
    }

    if let Some(s) = status {
        sql.push_str(&format!(" AND status = '{}'", s));
    }

    sql.push_str(" ORDER BY ranked_date DESC NULLS LAST");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let kw = format!("%{}%", keyword);
    let rows = sqlx::query(&sql).bind(kw).fetch_all(pool).await?;

    Ok(rows
        .into_iter()
        .map(|r| Beatmapset {
            id: r.try_get("id").unwrap(),
            title: r.try_get("title").unwrap(),
            title_unicode: r.try_get("title_unicode").ok(),
            artist: r.try_get("artist").unwrap(),
            artist_unicode: r.try_get("artist_unicode").ok(),
            creator: r.try_get("creator").unwrap(),
            creator_id: r.try_get("creator_id").ok(),
            genre_id: r.try_get("genre_id").ok(),
            language_id: r.try_get("language_id").ok(),
            rating: r.try_get("rating").ok(),
            source: r.try_get("source").ok(),
            tags: r.try_get("tags").ok(),
            status: r.try_get("status").unwrap(),
            ranked_date: r.try_get("ranked_date").ok(),
            submitted_date: r.try_get("submitted_date").ok(),
            last_updated: r.try_get("last_updated").ok(),
            bpm: r.try_get("bpm").ok(),
            video: r.try_get("video").unwrap_or(false),
            storyboard: r.try_get("storyboard").unwrap_or(false),
            nsfw: r.try_get("nsfw").unwrap_or(false),
            favourite_count: r.try_get("favourite_count").unwrap_or(0),
            play_count: r.try_get("play_count").unwrap_or(0),
            availability_download_disabled: r
                .try_get("availability_download_disabled")
                .unwrap_or(false),
            created_at: r
                .try_get("created_at")
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: r
                .try_get("updated_at")
                .unwrap_or_else(|_| chrono::Utc::now()),
            beatmaps: None,
        })
        .collect())
}
