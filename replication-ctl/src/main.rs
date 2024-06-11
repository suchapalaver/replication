mod cli;
use crate::cli::cli;

pub mod image_svc {
    tonic::include_proto!("image");
}

use image_svc::{image_service_client::ImageServiceClient, ImageRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tonic::transport::Endpoint;

#[derive(Debug, Serialize, Deserialize)]
struct IntentConfig {
    model: String,
    input: Value,
}

impl From<IntentConfig> for ImageRequest {
    fn from(IntentConfig { model, input }: IntentConfig) -> Self {
        ImageRequest {
            model,
            input: input.to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = cli().get_matches();

    let file = std::fs::read_to_string(matches.get_one::<String>("intent").unwrap())?;

    let request: ImageRequest = serde_yml::from_str::<IntentConfig>(&file)?.into();

    let endpoint: Endpoint =
        format!("http://[::]:{}", matches.get_one::<String>("port").unwrap()).try_into()?;

    let mut client = ImageServiceClient::connect(endpoint).await?;

    println!("\nSending request to replication image service ...");

    let response = client.process_intent(request).await?.into_inner();

    let img_urls = response.img_urls;

    if let Some(url) = img_urls.get(0) {
        // Download the image data from the first URL
        let image_data = reqwest::get(url).await?.bytes().await?;

        // Save the image to a file
        std::fs::write("received_image.png", &image_data)?;

        println!("Image saved as received_image.png");
    } else {
        println!("No URLs received in the response.");
    }

    println!("\nReceived image response from 'replication' via Replicate:\n");

    Ok(())
}
