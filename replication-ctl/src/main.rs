mod cli;
use crate::cli::cli;

pub mod replicate {
    tonic::include_proto!("replicate");
}

use replicate::{replicate_service_client::ReplicateServiceClient, ReplicateRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tonic::transport::Endpoint;

#[derive(Debug, Serialize, Deserialize)]
struct IntentConfig {
    model: String,
    input: Value,
}

impl From<IntentConfig> for ReplicateRequest {
    fn from(IntentConfig { model, input }: IntentConfig) -> Self {
        ReplicateRequest {
            model,
            input: input.to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = cli().get_matches();

    let file = std::fs::read_to_string(matches.get_one::<String>("intent").unwrap())?;

    let request: ReplicateRequest = serde_yml::from_str::<IntentConfig>(&file)?.into();

    let endpoint: Endpoint =
        format!("http://[::]:{}", matches.get_one::<String>("port").unwrap()).try_into()?;

    let mut client = ReplicateServiceClient::connect(endpoint).await?;

    println!("\nSending request to replicate service ...");

    let response = client.process_intent(request).await?.into_inner();

    let payload = response.payload;

    if let Some(url) = payload.get(0) {
        // Download the image data from the first URL
        let image_data = reqwest::get(url).await?.bytes().await?;

        // Save the image to a file
        std::fs::write("received_image.png", &image_data)?;

        println!("Image saved as received_image.png");
    } else {
        println!("No URLs received in the response.");
    }

    println!("\nReceived response from 'replication' via Replicate:\n");

    Ok(())
}
