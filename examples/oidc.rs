use std::env;

use reqsign::{AliyunLoader, AliyunConfig, AliyunOssSigner};
use tokio;
use reqwest::{Client, Url};
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let _ = env_logger::builder().is_test(false).try_init();
    env::set_var("ALIBABA_CLOUD_ROLE_ARN", "acs:ram::5960071133727770:role/databendqueryrole");
    env::set_var("ALIBABA_CLOUD_OIDC_PROVIDER_ARN", "acs:ram::5960071133727770:oidc-provider/ack-rrsa-c2014081101d44af7ad0d3b2a26e52c8a");
    env::set_var("ALIBABA_CLOUD_OIDC_TOKEN_FILE", "/tmp/oidc-token");
    let client = Client::new();
    let config = AliyunConfig::default().from_env();
    let loader = AliyunLoader::new(client.clone(), config);
    let signer = AliyunOssSigner::new("dmp-data-default-bucket");
    // Construct request
    let url = Url::parse("https://dmp-data-default-bucket.oss-accelerate.aliyuncs.com/backups/hudao/si/2023-06-30.csv.gzip")?;
    let mut req = reqwest::Request::new(http::Method::GET, url);
    // Signing request with Signer
    let credential = loader.load().await?.unwrap();
    signer.sign(&mut req, &credential)?;
    // Sending already signed request.
    let resp = client.execute(req).await?;
    println!("resp got status: {}", resp.status());
    Ok(())
}