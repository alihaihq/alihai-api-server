use crate::prelude::*;
use crate::routes::api::v1::{forms::controllers, helpers, types::ContactUsInput};
use crate::{constants, http_client, utils};
use hyper::{Body, Request, Response};
use serde_json::{json, Value};

pub async fn contact_us_form_post(req: Request<Body>) -> crate::Result<Response<Body>> {
    let client_ip = utils::extract_client_ip_from_req(&req);

    let contact_us_input = match helpers::parse_req_body_as_json::<ContactUsInput>("Contact Us Form", req).await {
        Ok(input) => input,
        Err(err) => {
            return resp_400!("{}", err);
        }
    };

    let geo_location = controllers::fetch_geo_location_from_ip(client_ip)
        .await
        .context("Failed to fetch Geo location from IP")?;

    let data = json!( {
        "type": "contactus",
        "payload": {
            "mobile": contact_us_input.mobile,
            "email": contact_us_input.email,
            "region": geo_location.region,
            "city": geo_location.city,
            "pincode": geo_location.pin_code,
            "ip": client_ip.to_string(),
            "message": contact_us_input.message
        }
    });

    let resp_data = http_client::client()
        .post(utils::env_crit(constants::env::GOOGLE_APPS_SCRIPT_URL).as_str())
        .json(&data)
        .send()
        .await
        .context("Failed to send request to Google Apps Script")?
        .shake("Request to Google Apps Script")
        .await?
        .json::<Value>()
        .await
        .context("Failed to decode Google Apps Script response to JSON")?;

    resp_200!(resp_data)
}
