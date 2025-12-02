pub mod local;
pub mod s3;

use bytes::Bytes;
pub use local::LocalStorage;
pub use s3::S3Storage;

#[derive(Clone)]
pub enum BeatmapStorage {
    Local(LocalStorage),
    S3(S3Storage),
}

impl BeatmapStorage {
    pub async fn get(&self, set_id: i64, no_video: bool) -> anyhow::Result<Option<Bytes>> {
        match self {
            Self::Local(s) => s.get(set_id, no_video).await,
            Self::S3(s) => s.get(set_id, no_video).await,
        }
    }

    pub async fn put(&self, set_id: i64, no_video: bool, data: Bytes) -> anyhow::Result<()> {
        match self {
            Self::Local(s) => s.put(set_id, no_video, data).await,
            Self::S3(s) => s.put(set_id, no_video, data).await,
        }
    }

    pub async fn exists(&self, set_id: i64, no_video: bool) -> anyhow::Result<bool> {
        match self {
            Self::Local(s) => s.exists(set_id, no_video).await,
            Self::S3(s) => s.exists(set_id, no_video).await,
        }
    }

    pub async fn delete(&self, set_id: i64, no_video: bool) -> anyhow::Result<()> {
        match self {
            Self::Local(s) => s.delete(set_id, no_video).await,
            Self::S3(s) => s.delete(set_id, no_video).await,
        }
    }
}
