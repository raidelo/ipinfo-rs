use core::net::IpAddr;
use core::time::Duration;

use crate::{ENDPOINT, HOST, TIMEOUT};

pub enum IpTarget {
    Specific(IpAddr),
    OwnAddress,
}

pub async fn get_info(ip: &IpTarget) -> Result<reqwest::Response, reqwest::Error> {
    let mut url = reqwest::Url::parse(HOST).expect("Valid hardcoded URL");

    {
        let mut segments = url
            .path_segments_mut()
            .expect("Valid hardcoded URL. Should perfectly be a base");

        segments.push(ENDPOINT);

        if let IpTarget::Specific(ip_addr) = ip {
            segments.push(&ip_addr.to_string());
        }
    }

    let client = reqwest::ClientBuilder::new()
        .user_agent(format!("ipinfo-rs/{}", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_millis(TIMEOUT))
        .build()?;

    client.get(url).send().await?.error_for_status()
}
