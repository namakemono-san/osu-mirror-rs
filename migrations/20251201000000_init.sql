
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE IF NOT EXISTS beatmapsets (
    id BIGINT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    title_unicode VARCHAR(255),
    artist VARCHAR(255) NOT NULL,
    artist_unicode VARCHAR(255),
    creator VARCHAR(255) NOT NULL,
    creator_id BIGINT DEFAULT 0,
    source VARCHAR(255),
    tags TEXT,
    status VARCHAR(20) NOT NULL,

    ranked_date TIMESTAMPTZ,
    submitted_date TIMESTAMPTZ,
    last_updated TIMESTAMPTZ,

    bpm DOUBLE PRECISION,
    video BOOLEAN DEFAULT FALSE,
    storyboard BOOLEAN DEFAULT FALSE,
    nsfw BOOLEAN DEFAULT FALSE,

    favourite_count INTEGER DEFAULT 0,
    play_count INTEGER DEFAULT 0,

    availability_download_disabled BOOLEAN DEFAULT FALSE,

    genre_id INTEGER DEFAULT 0,
    language_id INTEGER DEFAULT 0,
    rating DOUBLE PRECISION DEFAULT 0,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    deleted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS beatmaps (
    id BIGINT PRIMARY KEY,
    beatmapset_id BIGINT NOT NULL REFERENCES beatmapsets(id) ON DELETE CASCADE,

    version VARCHAR(255) NOT NULL,
    mode VARCHAR(10) NOT NULL,
    mode_int INTEGER NOT NULL,

    difficulty_rating DOUBLE PRECISION,
    ar DOUBLE PRECISION,
    cs DOUBLE PRECISION,
    drain DOUBLE PRECISION,
    accuracy DOUBLE PRECISION,

    bpm DOUBLE PRECISION,

    total_length INTEGER,
    hit_length INTEGER,
    max_combo INTEGER,

    count_circles INTEGER,
    count_sliders INTEGER,
    count_spinners INTEGER,

    checksum VARCHAR(32),

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    deleted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS cache_metadata (
    beatmapset_id BIGINT PRIMARY KEY,
    file_size BIGINT NOT NULL,
    storage_path VARCHAR(500) NOT NULL,
    storage_backend VARCHAR(20) NOT NULL,
    no_video BOOLEAN DEFAULT FALSE,

    last_updated TIMESTAMPTZ,
    last_accessed TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sync_cursors (
    id VARCHAR(50) PRIMARY KEY,
    cursor_string TEXT,
    last_sync TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_status ON beatmapsets(status);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_ranked_date ON beatmapsets(ranked_date DESC);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_last_updated ON beatmapsets(last_updated DESC);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_favourite_count ON beatmapsets(favourite_count DESC);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_play_count ON beatmapsets(play_count DESC);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_creator ON beatmapsets(creator);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_artist ON beatmapsets(artist);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_title ON beatmapsets(title);

CREATE INDEX IF NOT EXISTS idx_beatmaps_beatmapset_id ON beatmaps(beatmapset_id);
CREATE INDEX IF NOT EXISTS idx_beatmaps_mode ON beatmaps(mode_int);
CREATE INDEX IF NOT EXISTS idx_beatmaps_difficulty ON beatmaps(difficulty_rating);
CREATE INDEX IF NOT EXISTS idx_beatmaps_ar ON beatmaps(ar);
CREATE INDEX IF NOT EXISTS idx_beatmaps_cs ON beatmaps(cs);
CREATE INDEX IF NOT EXISTS idx_beatmaps_checksum ON beatmaps(checksum);

CREATE INDEX IF NOT EXISTS idx_cache_metadata_last_accessed ON cache_metadata(last_accessed);
CREATE INDEX IF NOT EXISTS idx_cache_metadata_backend ON cache_metadata(storage_backend);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_title_trgm 
    ON beatmapsets USING gin (title gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_artist_trgm 
    ON beatmapsets USING gin (artist gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_creator_trgm 
    ON beatmapsets USING gin (creator gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_tags_trgm 
    ON beatmapsets USING gin (tags gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_status_ranked_date 
    ON beatmapsets(status, ranked_date DESC);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_status_favourite 
    ON beatmapsets(status, favourite_count DESC);
