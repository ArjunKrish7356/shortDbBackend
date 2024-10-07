use proto::basic_client::BasicClient;
use proto::SetRequest;
use std::error::Error;
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("commands");
}

pub async fn set(longLink: String, shortCode: String) -> Result<(String), Box<dyn Error>> {
    // get the url as longLink
    // use the shortcode as key and longLink as value
    // call grpc get and give both values
    // return the new link created by shortcode

    let setFormat = SetRequest {
        key: shortCode.to_string(),
        value: longLink.to_string(),
    };

    let url = "http://[::1]:50051";
    let mut client = BasicClient::connect(url).await.unwrap();

    let setRequest = tonic::Request::new(setFormat);

    let setResponse = client.set(setRequest).await?;

    Ok(shortCode)
}
