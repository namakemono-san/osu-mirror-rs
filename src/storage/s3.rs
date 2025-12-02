use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

#[derive(Clone)]
pub struct S3Storage {
    client: Client,
    bucket: String,
    prefix: String,
}

impl S3Storage {
    pub async fn new(endpoint: &str, bucket: String, region: &str, prefix: String) -> Self {
        let config = aws_config::defaults(aws_config::BehaviorVersion::v2025_08_07())
            .endpoint_url(endpoint)
            .region(aws_config::Region::new(region.to_string()))
            .load()
            .await;
        let client = Client::new(&config);
        Self {
            client,
            bucket,
            prefix,
        }
    }

    fn get_key(&self, set_id: i64, no_video: bool) -> String {
        let id = if no_video { -set_id } else { set_id };
        let dir1 = set_id.abs() / 1000;
        let dir2 = set_id.abs() % 1000;
        format!("{}/{}/{}/{}.osz", self.prefix, dir1, dir2, id)
    }

    pub async fn get(&self, set_id: i64, no_video: bool) -> anyhow::Result<Option<Bytes>> {
        let key = self.get_key(set_id, no_video);
        match self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await
        {
            Ok(resp) => {
                let data = resp.body.collect().await?.into_bytes();
                Ok(Some(data))
            }
            Err(e) if e.to_string().contains("NoSuchKey") => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn put(&self, set_id: i64, no_video: bool, data: Bytes) -> anyhow::Result<()> {
        let key = self.get_key(set_id, no_video);
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(data))
            .content_type("application/x-osu-beatmap-archive")
            .send()
            .await?;
        Ok(())
    }

    pub async fn exists(&self, set_id: i64, no_video: bool) -> anyhow::Result<bool> {
        let key = self.get_key(set_id, no_video);
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub async fn delete(&self, set_id: i64, no_video: bool) -> anyhow::Result<()> {
        let key = self.get_key(set_id, no_video);
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await?;
        Ok(())
    }
}
