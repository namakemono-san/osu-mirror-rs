use bytes::Bytes;
use std::path::PathBuf;
use tokio::fs;

#[derive(Clone)]
pub struct LocalStorage {
    base_dir: PathBuf,
}

impl LocalStorage {
    pub fn new(base_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&base_dir).ok();
        Self { base_dir }
    }

    fn get_path(&self, set_id: i64, no_video: bool) -> PathBuf {
        let id = if no_video { -set_id } else { set_id };
        let dir1 = set_id.abs() / 1000;
        let dir2 = set_id.abs() % 1000;
        self.base_dir
            .join(format!("{}", dir1))
            .join(format!("{}", dir2))
            .join(format!("{}.osz", id))
    }

    pub async fn get(&self, set_id: i64, no_video: bool) -> anyhow::Result<Option<Bytes>> {
        let path = self.get_path(set_id, no_video);
        match fs::read(&path).await {
            Ok(data) => Ok(Some(Bytes::from(data))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn put(&self, set_id: i64, no_video: bool, data: Bytes) -> anyhow::Result<()> {
        let path = self.get_path(set_id, no_video);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, &data).await?;
        fs::rename(&temp_path, &path).await?;
        Ok(())
    }

    pub async fn exists(&self, set_id: i64, no_video: bool) -> anyhow::Result<bool> {
        Ok(self.get_path(set_id, no_video).exists())
    }

    pub async fn delete(&self, set_id: i64, no_video: bool) -> anyhow::Result<()> {
        let path = self.get_path(set_id, no_video);
        fs::remove_file(&path).await?;
        Ok(())
    }
}
