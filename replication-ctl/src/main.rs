mod cli;
use crate::cli::cli;

pub mod replicate {
    tonic::include_proto!("replicate");
}

use replicate::{replicate_service_client::ReplicateServiceClient, ReplicateRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tonic::transport::Endpoint;

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

    match response.output_type.as_str().into() {
        ResponseType::Image => {
            let payload = response.payload;

            if let Some(string) = payload.into_iter().next() {
                let image_data = reqwest::get(&string).await?.bytes().await?;

                std::fs::write("received_image.png", &image_data)?;

                println!("Image saved as received_image.png");
            } else {
                eprintln!("Nothing received in the response.");
            }

            println!("\nReceived response from replication service:\n");
        }
        ResponseType::Text => unimplemented!(),
        ResponseType::Unrecognized => eprintln!("Error! Unrecognized response type!"),
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct IntentConfig {
    model: String,
    input: Value,
    input_type: String,
    output_type: String,
}

impl From<IntentConfig> for ReplicateRequest {
    fn from(
        IntentConfig {
            model,
            input,
            input_type,
            output_type,
        }: IntentConfig,
    ) -> Self {
        ReplicateRequest {
            model,
            input: input.to_string(),
            input_type,
            output_type,
        }
    }
}

enum ResponseType {
    Image,
    Text,
    Unrecognized,
}

impl From<&str> for ResponseType {
    fn from(s: &str) -> Self {
        match s {
            "image" => Self::Image,
            "text" => Self::Text,
            _ => Self::Unrecognized,
        }
    }
}
