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
use error_messages::{
    API_RESPONSE_JSON_PARSING_ERROR, BODY_READING_ERROR, EMPTY_RESPONSE_FROM_SERVER,
    INVALID_API_RESPONSE_TYPE,
};

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

    let body: String = response
        .text()
        .await
        .map_err(|_| BODY_READING_ERROR.to_string())?;

    if body.is_empty() {
        return Err(EMPTY_RESPONSE_FROM_SERVER.into());
    };

    let info: serde_json::Value = serde_json::from_str(&body).map_err(|err| {
        let detail = match err.classify() {
            serde_json::error::Category::Syntax => "Invalid JSON syntax",
            serde_json::error::Category::Data => "Data type mismatch",
            serde_json::error::Category::Eof => "Unexpected end of data",
            serde_json::error::Category::Io => "Unknown parsing error",
        };

        API_RESPONSE_JSON_PARSING_ERROR.replace("%(detail)s", detail)
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

        INVALID_API_RESPONSE_TYPE.replace("%(received)s", received)
    })?;

    if object.is_empty() {
        return Err(EMPTY_RESPONSE_FROM_SERVER.into());
    };

    let table_formatted = get_table_formatted(object, style);

    print!("{}", table_formatted);

    Ok(())
}

fn print_error<E: Display>(err: E) {
    eprintln!("\x1b[1;31merror\x1b[39m:\x1b[0m {}", err);
}
