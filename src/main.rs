use core::fmt::Display;
use core::net::IpAddr;
use std::error::Error;
use std::process::exit;

use ipinfo_rs::{TableStyle, get_table_formatted};

mod cli;
use cli::parse_args;

mod consts;
use consts::{ENDPOINT, HOST, TIMEOUT};

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

    let body: String = response.text().await?;

    let info: serde_json::Value = serde_json::from_str(&body)?;

    let object = info.as_object().ok_or_else(|| {
        let response_type = match info {
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
            "Invalid API response: expected object, got: {}",
            response_type
        )
    })?;

    if object.is_empty() {
        return Err("API returned empty response. Possible causes:\n\
              • Invalid IP address format
              • No fields specified in API request
              • Temporary server issue"
            .into());
    };

    let table_formatted = get_table_formatted(object, style);

    print!("{}", table_formatted);

    Ok(())
}

fn print_error<E: Display>(err: E) {
    eprintln!("\x1b[1;31merror\x1b[0m: {}", err);
}

fn map_reqwest_error(err: reqwest::Error) -> String {
    let err = err.without_url();

    let general_hint = "Check your internet connection or firewall settings";

    if err.is_timeout() {
        format!(
            "\u{231b} Timeout after {}ms: Check your connection",
            TIMEOUT
        )
    } else if err.is_connect() {
        format!(
            "\u{01f50c} Connection failed: Server unreachable. {}",
            general_hint
        )
    } else if err.is_decode() {
        "\u{01f4e6} Data error: Failed to process server response".to_string()
    } else if let Some(status) = err.status() {
        match status {
            s if s.is_client_error() && s.as_u16() == 429 => format!(
                "\u{01f40c} API limit exceeded: You've reached ip-api.com's free tier limit (45 reqs/min)"
            ),
            s if s.is_client_error() => format!(
                "\u{01f464} Client error ({}){}",
                s,
                if let Some(c_reason) = s.canonical_reason() {
                    format!(": {}", c_reason)
                } else {
                    "".into()
                }
            ),
            s if s.is_server_error() => format!("\u{01f310} Server error ({}): Try again later", s),
            _ => "\u{26a0}\u{fe0f} HTTP error".into(),
        }
    } else {
        "\u{274c} Network error: Unknown request failure".to_string()
    }
}
