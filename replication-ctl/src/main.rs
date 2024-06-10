mod cli;
use crate::cli::cli;

pub mod image_svc {
    tonic::include_proto!("image");
}

use image_svc::{image_service_client::ImageServiceClient, ImageRequest};
use tonic::{transport::Endpoint, Request};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = cli().get_matches();

    let endpoint: Endpoint =
        format!("http://[::]:{}", matches.get_one::<String>("port").unwrap()).try_into()?;

    let mut client = ImageServiceClient::connect(endpoint).await?;

    let intent = matches.get_one::<String>("intent").unwrap().to_owned();

    let request = Request::new(ImageRequest { intent });

    println!("\nSending request to 'replication' ...");

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
