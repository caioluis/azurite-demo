use azure_core::Url;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let blob_service_client = ClientBuilder::emulator().blob_service_client();

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
