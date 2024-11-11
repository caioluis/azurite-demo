use azure_core::Url;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    dotenvy::dotenv().unwrap();

    let blob_service_client = match env::var("env") {
        Ok(env) => match env.as_str() {
            "azurite" => ClientBuilder::emulator().blob_service_client(),
            "azure" => {
                let account_name =
                    env::var("AZURE_STORAGE_ACCOUNT_NAME").expect("Missing env for account name");
                let account_key =
                    env::var("AZURE_STORAGE_ACCESS_KEY").expect("Missing env for account key");

                let credential = StorageCredentials::access_key(&account_name, account_key);

                BlobServiceClient::new(&account_name, credential)
            }
            _ => panic!("Invalid env"),
        },
        Err(_) => panic!("env not found"),
    };

    let mut container_client = blob_service_client.list_containers().into_stream();

    while let Some(containers) = container_client.next().await {
        match containers {
            Ok(containers) => {
                for container in containers.containers {
                    println!("Container: {}", container.name);
                }
            }
            Err(err) => {
                println!("Error listing containers: {}", err);
                break;
            }
        }
    }

    let specific_blob_client = blob_service_client
        .container_client("meow")
        .blob_client("a_cute_cat.png");

    specific_blob_client
        .copy_from_url(Url::parse("https://cdn2.thecatapi.com/images/0XYvRd7oD.jpg").unwrap())
        .await?;

    Ok(())
}
