use crate::consts::TIMEOUT;

use crate::error_messages::{
    API_RATE_LIMIT_EXEEDED, CLIENT_ERROR, IS_BODY, IS_BUILDER, IS_CONNECT, IS_DECODE, IS_REDIRECT,
    IS_REQUEST, IS_TIMEOUT, SERVER_ERROR, UNKNOWN_HTTP_ERROR, UNKNOWN_NETWORK_ERROR,
};

pub fn map_reqwest_error(err: reqwest::Error) -> String {
    let err = err.without_url();

    let error_msg: String;

    if err.is_timeout() {
        error_msg = IS_TIMEOUT.replace("%(timeout)s", &(TIMEOUT / 1000).to_string());
    } else if err.is_connect() {
        error_msg = IS_CONNECT.into();
    } else if err.is_request() {
        error_msg = IS_REQUEST.into();
    } else if err.is_builder() {
        error_msg = IS_BUILDER.into();
    } else if err.is_body() {
        error_msg = IS_BODY.into();
    } else if err.is_redirect() {
        error_msg = IS_REDIRECT.into();
    } else if err.is_decode() {
        error_msg = IS_DECODE.into();
    } else if let Some(status) = err.status() {
        match status {
            s if s.is_client_error() && s.as_u16() == 429 => {
                error_msg = API_RATE_LIMIT_EXEEDED.into();
            }
            s if s.is_client_error() => {
                let reason = s
                    .canonical_reason()
                    .map(|reason| format!(": {}", reason))
                    .unwrap_or("".into());

                error_msg = CLIENT_ERROR
                    .replace("%(status_code)d", s.as_str())
                    .replace("%(reason)s", &reason);
            }
            s if s.is_server_error() => {
                let reason = s
                    .canonical_reason()
                    .map(|reason| format!(": {}", reason))
                    .unwrap_or("".into());

                error_msg = SERVER_ERROR
                    .replace("%(status_code)d", s.as_str())
                    .replace("%(reason)s", &reason);
            }

            s => {
                let reason = s
                    .canonical_reason()
                    .map(|reason| format!(": {}", reason))
                    .unwrap_or("".into());

                error_msg = UNKNOWN_HTTP_ERROR
                    .replace("%(status_code)d", s.as_str())
                    .replace("%(reason)s", &reason);
            }
        }
    } else {
        {
            error_msg = UNKNOWN_NETWORK_ERROR.into();
        }
    };

    return error_msg;
}
