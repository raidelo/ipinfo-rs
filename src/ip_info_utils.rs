use core::net::Ipv4Addr;
use core::time::Duration;

use crate::{ENDPOINT, HOST, TIMEOUT};

pub enum IpTarget {
    Specific(Ipv4Addr),
    OwnAddress,
}

pub async fn get_info(ip: &IpTarget) -> Result<reqwest::Response, reqwest::Error> {
    let mut url = reqwest::Url::parse(HOST).unwrap();
    let mut segments = url.path_segments_mut().unwrap();

    segments.push(ENDPOINT);

    if let IpTarget::Specific(ip_addr) = ip {
        segments.push(&ip_addr.to_string());
    }

    drop(segments);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(TIMEOUT))
        .build()?;

    client.get(url).send().await?.error_for_status()
}
