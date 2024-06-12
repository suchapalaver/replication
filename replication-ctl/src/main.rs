mod cli;
use std::fs;

use crate::cli::cli;

pub mod replicate {
    tonic::include_proto!("replicate");
}

use replicate::{
    replicate_service_client::ReplicateServiceClient, ReplicateRequest, ReplicateResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = cli().get_matches();

    let endpoint: Endpoint =
        format!("http://[::]:{}", matches.get_one::<String>("port").unwrap()).try_into()?;
    let mut client = ReplicateServiceClient::connect(endpoint).await?;

    let file = fs::read_to_string(matches.get_one::<String>("intent").unwrap())?;
    let mut request: ReplicateRequest = serde_yml::from_str::<IntentConfig>(&file)?.into();

    // For now, envisioning three iterations - the initial request, a follow-up request based on the first response,
    // and a final request based on the second response.
    loop {
        request.iteration += 1;

        let mut iterations = request.iteration;

        if request.iteration == 1 {
            println!("\nSending initial request to replicate service ...");
        } else {
            println!("\nSending follow-up request number {iterations} to replicate service ...");
        }

        let response = client.process_intent(request.clone()).await?.into_inner();

        println!("\nReceived response from replication service!");

        match response.into() {
            ResponseType::Image(ImageUrl(image_url)) => {
                let image_data = reqwest::get(&image_url).await?.bytes().await?;
                if iterations > 1 {
                    iterations -= 1;
                }
                let write_path = format!("received_image-{iterations}.png");
                std::fs::write(&write_path, &image_data)?;

                println!("\nReceived image saved as {write_path}");

                if iterations < 2 {
                    let file = fs::read_to_string("./llava.yaml").unwrap();
                    let mut image_reader_config = serde_yml::from_str::<IntentConfig>(&file)?;

                    let input = {
                        let input = image_reader_config.input.as_object().unwrap();
                        let mut prompt = input.get("prompt").unwrap().as_str().unwrap().to_string();
                        let condition = input.get("condition").unwrap().as_str().unwrap();
                        prompt.push_str(condition);
                        serde_json::json!({
                            "image": image_url,
                            "prompt": prompt
                        })
                    };

                    image_reader_config.input = input;
                    image_reader_config.iteration = iterations;

                    request = image_reader_config.into();

                    continue;
                }

                break;
            }
            ResponseType::Text(text) => {
                println!("\nReceived text response: {text:?}");

                let mut config = serde_yml::from_str::<IntentConfig>(&file)?;
                let input = config.input.as_object_mut().unwrap();
                input.insert("prompt".to_string(), Value::String(text));

                config.iteration = iterations;
                request = config.into();

                continue;
            }
            ResponseType::Null => eprintln!("\nError! Unexpected response!"),
        }
    }

    println!("\nAll done! Exiting ...");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct IntentConfig {
    model: String,
    #[serde(default)]
    input: Value,
    #[serde(default)]
    additional: String,
    input_type: String,
    output_type: String,
    #[serde(default)]
    iteration: u32,
}

impl From<IntentConfig> for ReplicateRequest {
    fn from(
        IntentConfig {
            model,
            input,
            input_type,
            output_type,
            iteration,
            ..
        }: IntentConfig,
    ) -> Self {
        ReplicateRequest {
            model,
            input: input.to_string(),
            input_type,
            output_type,
            iteration,
        }
    }
}

struct ImageUrl(String);

enum ResponseType {
    Image(ImageUrl),
    Text(String),
    Null,
}

impl From<ReplicateResponse> for ResponseType {
    fn from(response: ReplicateResponse) -> Self {
        let payload = response.payload;

        match response.output_type.as_str() {
            "image" => match payload.into_iter().next() {
                Some(url) => Self::Image(ImageUrl(url)),
                None => Self::Null,
            },
            "text" => Self::Text(payload.join("")),
            _ => Self::Null,
        }
    }
}
