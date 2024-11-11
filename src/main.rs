use azure_core::{
    error::{ErrorKind, ResultExt},
    Url,
};
use azure_storage::prelude::*;
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

    Ok(())
}
