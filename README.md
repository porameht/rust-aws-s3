# Rust S3 File Upload and Download Example

This project demonstrates how to upload and download files to AWS S3 bucket using Rust.

## Project Structure

The project consists of the following files:

- **Cargo.toml**: Contains project metadata and dependencies.
- **main.rs**: Contains the main Rust code for uploading and downloading files from S3.

## Dependencies

- **aws-sdk-s3**: AWS SDK for S3, used to interact with S3 services.
- **tokio**: Asynchronous runtime for Rust, used to handle async operations.
- **aws-config**: AWS configuration library, used to configure AWS clients.

## Usage

### Uploading a File

To upload a file to an S3 bucket, the `main.rs` file contains an `upload_file` function that takes the S3 client, bucket name, key, and file path as arguments. The `main` function demonstrates how to use this function to upload a file.

```rust
match upload_file(&client, bucket_name, key, file_path).await {
    Ok(output) => {
        println!("Upload successful: {:?}", output);
    }
    Err(e) => println!("Error: {}", e),
}
```

### Downloading a File

To download a file from an S3 bucket, the `main.rs` file contains a `download_file` function that takes the S3 client, bucket name, key, and file path as arguments. This function uses the `get_object` method from the AWS SDK to retrieve the file.

```rust
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
```

## Running the Project

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository.
3. Navigate to the project directory.
4. Run `cargo build` to build the project.
5. Run `cargo run` to execute the project.

## Configuration

To configure the AWS client, the `main.rs` file uses the `aws-config` crate to load the configuration from environment variables.

```rust
let config = aws_config::from_env()
    .region(Region::new("ap-southeast-1"))
    .load()
    .await;
```

Make sure to set the appropriate AWS credentials and region in your environment variables.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
