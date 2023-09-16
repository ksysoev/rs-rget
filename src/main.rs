use clap::Parser;
use colored_json::prelude::*;
use http::Method;
use reqwest::Client;
use yansi::Paint;

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
    #[arg(short, long, default_value = "false")]
    show_headers: bool,
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
    let headers = resp.headers();
    if args.show_headers {
        headers.iter().for_each(|(k, v)| {
            println!("{}: {}", Paint::blue(k).bold(), v.to_str().unwrap_or(""));
        });

        println!();
    }

    let default_content_type = http::header::HeaderValue::from_static("text/plain");
    let content_type_header = headers.get("content-type").unwrap_or(&default_content_type);
    let is_json_response = content_type_header
        .to_str()
        .unwrap_or("")
        .contains("application/json");

    let data = resp.text().await?;

    if is_json_response {
        println!("{}", data.to_colored_json_auto()?);
    } else {
        println!("{}", data);
    }

    Ok(())
}
