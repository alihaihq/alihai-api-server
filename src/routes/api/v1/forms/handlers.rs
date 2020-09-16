use crate::prelude::*;
use crate::routes::api::v1::{forms::controllers, helpers, types::ContactUsInput};
use hyper::{Body, Request, Response};

pub async fn contact_us_form_post(req: Request<Body>) -> crate::Result<Response<Body>> {
    let contact_us_input = match helpers::parse_req_body_as_json::<ContactUsInput>("Contact Us Form", req).await {
        Ok(input) => input,
        Err(err) => {
            return resp_400!("{}", err);
        }
    };

    dbg!(&contact_us_input);

    controllers::send_contact_us_form(contact_us_input)
        .await
        .context("Failed to send mail")?;

    resp_200!("Form submitted!")
}
