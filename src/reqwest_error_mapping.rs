use crate::consts::TIMEOUT;

use crate::error_messages::{
    API_RATE_LIMIT_EXEEDED, CLIENT_ERROR, IS_BODY, IS_BUILDER, IS_CONNECT, IS_DECODE, IS_REDIRECT,
    IS_REQUEST, IS_TIMEOUT, SERVER_ERROR, UNKNOWN_HTTP_ERROR, UNKNOWN_NETWORK_ERROR,
};

pub fn map_reqwest_error(err: reqwest::Error) -> String {
    let err = err.without_url();

    let emoji: &str;
    let title: String;
    let text: &str;

    if err.is_timeout() {
        emoji = "\u{231b}";
        title = format!("Timeout after {}ms", TIMEOUT);
        text = IS_TIMEOUT;
    } else if err.is_connect() {
        emoji = "\u{01f50c}";
        title = format!("Connection failed");
        text = IS_CONNECT;
    } else if err.is_request() {
        emoji = "\u{01f4dc}";
        title = format!("Invalid request");
        text = IS_REQUEST;
    } else if err.is_builder() {
        emoji = "\u{01f6e0}";
        title = format!("Request builder error");
        text = IS_BUILDER;
    } else if err.is_body() {
        emoji = "\u{01f4e6}";
        title = format!("Invalid response body");
        text = IS_BODY;
    } else if err.is_redirect() {
        emoji = "\u{01f504}";
        title = format!("Too many redirects");
        text = IS_REDIRECT;
    } else if err.is_decode() {
        emoji = "\u{01f4c4}";
        title = format!("Response decoding error");
        text = IS_DECODE;
    } else if let Some(status) = err.status() {
        match status {
            s if s.is_client_error() && s.as_u16() == 429 => {
                emoji = "\u{01f40c}";
                title = format!("API rate limit exceeded");
                text = API_RATE_LIMIT_EXEEDED;
            }
            s if s.is_client_error() => {
                emoji = "\u{01f464}";
                title = format!(
                    "Client error ({}){}",
                    s,
                    s.canonical_reason()
                        .map(|reason| format!(": {}", reason))
                        .unwrap_or("".into())
                );
                text = CLIENT_ERROR;
            }
            s if s.is_server_error() => {
                emoji = "\u{01f310}";
                title = format!(
                    "Server error ({}){}",
                    s,
                    s.canonical_reason()
                        .map(|reason| format!(": {}", reason))
                        .unwrap_or("".into())
                );
                text = SERVER_ERROR;
            }

            _ => {
                emoji = "\u{26a0}\u{fe0f}";
                title = format!("HTTP error",);
                text = UNKNOWN_HTTP_ERROR;
            }
        }
    } else {
        {
            emoji = "\u{01f300}";
            title = format!("Unknown network error");
            text = UNKNOWN_NETWORK_ERROR;
        }
    };

    return format!("{emoji} {title}\n{text}",);
}
