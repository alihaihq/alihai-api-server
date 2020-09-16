use crate::constants;
use crate::http_client;
use crate::prelude::*;
use crate::routes::api::v1::types::ClientGeoLocation;
use serde_json::Value;
use std::net::IpAddr;
use url::Url;

pub async fn fetch_geo_location_from_ip(ip: IpAddr) -> crate::Result<ClientGeoLocation> {
    let api_url = constants::IP_API_ENDPOINT
        .parse::<Url>()
        .unwrap()
        .join(&ip.to_string())
        .context("Failed to parse the IP")?;

    let resp = http_client::client()
        .get(api_url)
        .send()
        .await
        .context("Failed to send request to IP server")?
        .shake("Fetch Geo Location From IP")
        .await?
        .json::<Value>()
        .await
        .context("Failed to decode response to JSON")?;

    let status = resp["status"]
        .as_str()
        .ok_or_else(|| crate::Error::new("No status field"))?;

    if status == "fail" {
        return Err(crate::Error::new(resp["message"].as_str().unwrap_or("")));
    }

    Ok(ClientGeoLocation {
        country: resp["country"].as_str().unwrap_or("").to_owned(),
        region: resp["regionName"].as_str().unwrap_or("").to_owned(),
        city: resp["city"].as_str().unwrap_or("").to_owned(),
        pin_code: resp["zip"].as_str().unwrap_or("").to_owned(),
    })
}
