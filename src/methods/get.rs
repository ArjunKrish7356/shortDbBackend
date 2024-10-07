use proto::basic_client::BasicClient;
use proto::GetRequest;
use std::error::Error;
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("commands");
}

pub async fn get(
    shortCode: String,
) -> Result<(String), Box<dyn Error>> {
    let getFormat = GetRequest {
        key: shortCode.to_string(),
    };
    let url = "http://[::1]:50051";
    let mut client = BasicClient::connect(url).await.unwrap();

    let getRequest = tonic::Request::new(getFormat);

    let getResponse = client.get(getRequest).await?;

    Ok(getResponse.get_ref().value.clone())
}
