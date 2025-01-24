use aws_sdk_s3::{config::Region, primitives::ByteStream, Client};
use std::path::Path;
use aws_sdk_s3::operation::put_object::{PutObjectOutput, PutObjectError};
use aws_sdk_s3::operation::get_object::{GetObjectOutput, GetObjectError};
use aws_sdk_s3::error::SdkError;

#[tokio::main]
async fn main() {
    let config = aws_config::from_env()
        .region(Region::new("ap-southeast-1"))
        .load()
        .await;

    let client = Client::new(&config);

    let bucket_name = "xxxxxx";
    let key = "xxxxx/xxxx/xxxxx/test-upload.png";
    let file_path = "test.png";

    match upload_file(&client, bucket_name, key, file_path).await {
        Ok(output) => {
            println!("Upload successful: {:?}", output);
        }
        Err(e) => println!("Error: {}", e),
    }
}

async fn download_file(
    client: &Client, 
    bucket_name: &str, 
    key: &str, 
    file_path: &str
) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
    client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
}

async fn upload_file(
    client: &Client, 
    bucket_name: &str, 
    key: &str, 
    file_path: &str
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(Path::new(file_path))
        .await;

    let output = client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await?;

    Ok(output)
}
