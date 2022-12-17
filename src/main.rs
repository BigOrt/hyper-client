use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{ stdout, AsyncWriteExt as _ };
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let https = HttpsConnector::new();

    //GET
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = "https://am.i.mullvad.net/json".parse()?;

    let mut resp = client.get(uri).await?;

    println!("Response: {:?}", resp.body());
    // END GET

    while let Some(chunk) = resp.body_mut().data().await {
            stdout().write_all(&chunk?).await?;
    }

    Ok(())
}

