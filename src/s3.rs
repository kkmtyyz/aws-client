use aws_sdk_s3::model::Bucket;
use aws_sdk_s3::Client;
use aws_types::SdkConfig;

pub struct S3 {
    client: Client,
}

impl S3 {
    pub fn new(config: &SdkConfig) -> S3 {
        S3 {
            client: Client::new(config),
        }
    }

    pub async fn list_buckets(&self) -> Vec<Bucket> {
        let resp = self.client.list_buckets().send().await.unwrap();
        resp.buckets().unwrap().to_vec()
    }
}
