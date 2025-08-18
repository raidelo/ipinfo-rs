use core::fmt::Display;
use core::net::IpAddr;
use std::error::Error;
use std::process::exit;

use ipinfo_rs::{TableStyle, get_table_formatted};

mod cli;
use cli::parse_args;

mod consts;
use consts::{ENDPOINT, HOST, TIMEOUT};

mod error_messages;

mod reqwest_error_mapping;
use reqwest_error_mapping::map_reqwest_error;

mod ip_info_utils;
use ip_info_utils::{IpTarget, get_info};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        print_error(err);
        exit(1);
    };
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut args = parse_args();

    let ip = args
        .remove_one::<IpAddr>("ip")
        .map_or(IpTarget::OwnAddress, IpTarget::Specific);

    let style: TableStyle = args
        .get_one::<String>("style")
        .map(|val| match val.as_str() {
            "thin" => TableStyle::Thin,
            "rounded" => TableStyle::ThinRounded,
            "double" => TableStyle::ThinDouble,
            "thick" => TableStyle::Thick,
            "basic" => TableStyle::Basic,
            _ => unreachable!(
                "Impossible style variant - clap validates against: thin|rounded|double|thick|basic"
            ),
        })
        .expect("Style guaranteed by clap's default_value");

    let response: reqwest::Response = get_info(&ip).await.map_err(map_reqwest_error)?;

    let body: String = response.text().await.map_err(|_| {
        "Failed to process API response\n
\x1b[1mTroubleshooting:\x1b[0m
  1. Check your internet connection
  2. Verify API status at https://ip-api.com/
  3. Test connectivity with: curl http://ip-api.com/json
  4. Try again in 1-2 minutes"
            .to_string()
    })?;

    let empty_response_str = "Empty API response\n
\x1b[1mPossible causes:\x1b[0m
  • Invalid IP address format
  • No fields specified in API request
  • Temporary server issue
  • API quota exceeded\n
\x1b[1mTroubleshooting:\x1b[0m
  1. Verify your IP address format
  2. Check API status at https://ip-api.com/
  3. Try again later";

    if body.is_empty() {
        return Err(empty_response_str.into());
    };

    let info: serde_json::Value = serde_json::from_str(&body).map_err(|err| {
        let detail = match err.classify() {
            serde_json::error::Category::Syntax => "Invalid JSON syntax",
            serde_json::error::Category::Data => "Data type mismatch",
            serde_json::error::Category::Eof => "Unexpected end of data",
            serde_json::error::Category::Io => "Unknown parsing error",
        };

        format!(
            "Invalid API response format\n
\x1b[1mDetail:\x1b[0m {}\n
\x1b[1mTroubleshooting:\x1b[0m
  1. The API service may have changed its format
  2. Check API status at https://ip-api.com/
  3. Report this issue to the tool maintainer",
            detail
        )
    })?;

    let object = info.as_object().ok_or_else(|| {
        let received = match info {
            serde_json::Value::Null => "null",
            serde_json::Value::Bool(_) => "bool",
            serde_json::Value::String(_) => "string",
            serde_json::Value::Number(_) => "number",
            serde_json::Value::Array(_) => "array",
            serde_json::Value::Object(_) => {
                unreachable!("If it was an object, .as_object() should have succeeded")
            }
        };

        format!(
            "Invalid API response\n
\x1b[1mExpected:\x1b[0m JSON Object
\x1b[1mReceived:\x1b[0m {}\n
\x1b[1mTroubleshooting:\x1b[0m
  1. The API service may have changed its format
  2. Check API status at https://ip-api.com/
  3. Report this issue to the tool maintainer",
            received
        )
    })?;

    if object.is_empty() {
        return Err(empty_response_str.into());
    };

    let table_formatted = get_table_formatted(object, style);

    print!("{}", table_formatted);

    Ok(())
}

fn print_error<E: Display>(err: E) {
    eprintln!("\x1b[1;31merror\x1b[39m:\x1b[0m {}", err);
}
