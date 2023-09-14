use clap::Parser;
use colored_json::prelude::*;
use http::Method;
use reqwest::Client;
use std::str::FromStr;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(index = 1, required = true)]
    url: String,
    #[arg(short, long, default_value = "GET")]
    method: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let method = Method::from_str(&args.method)?;

    let client = Client::new();
    let req = client.request(method, &args.url);
    let resp = req.send().await?.text().await?;

    println!("{}", resp.to_colored_json_auto()?);
    Ok(())
}
