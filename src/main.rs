use clap::Parser;
use colored_json::prelude::*;
use http::Method;
use reqwest::Client;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(index = 1, required = true)]
    url: String,
    #[arg(short, long, default_value = "GET")]
    method: String,
    #[arg(short, long, default_value = "")]
    json: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let method = Method::from_bytes(&args.method.to_uppercase().as_bytes())?;

    let client = Client::builder().user_agent("rget/0.1").build()?;
    let mut req = client.request(method, &args.url);

    if args.json != "" {
        req = req.header("Content-Type", "application/json");
        req = req.body(args.json);
    }

    let resp = req.send().await?;
    let data = resp.text().await?;

    println!("{}", data.to_colored_json_auto()?);
    Ok(())
}
