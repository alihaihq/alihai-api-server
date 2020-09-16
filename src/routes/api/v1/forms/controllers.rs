use crate::constants;
use crate::http_client;
use crate::prelude::*;
use crate::routes::api::v1::types::ContactUsInput;
use crate::utils;
use serde_json::{json, Value};

pub async fn send_mail(config: &Value) -> crate::Result<()> {
    let resp = http_client::client()
        .post(constants::SENDGRID_API_ENDPOINT)
        .bearer_auth(utils::env_crit(constants::env::SENDGRID_API_KEY))
        .header("Content-Type", "application/json")
        .json(config)
        .send()
        .await
        .context("Failed to send mail request to SendGrid API server")?;

    let is_error = resp.error_for_status_ref().is_err();

    if is_error {
        let status_code = resp.status();
        let err_msg = resp
            .text()
            .await
            .context("Couldn't extract send mail error response as text")?;

        Err(crate::Error::new(format!(
            "Failed to send mail: {}, Status: {}",
            err_msg, status_code
        )))
    } else {
        Ok(())
    }
}

pub async fn send_contact_us_form(data: ContactUsInput) -> crate::Result<()> {
    let mail_text = format!(
        r"
        <html>
        <body>
            Email: {},
            <br/>
            Mobile: {},
            <br/>
            Message: {}
        </body>
        </html>
    ",
        &data.email, &data.mobile, &data.message
    );

    let mail_config = json!({
        "personalizations": [
            {
                "to": [
                    {
                        "email": "rousanali786@gmail.com"
                    },
                    {
                        "email": "alihaistore@gmail.com"
                    }
                ]
            }
        ],
        "from": {
            "email": data.email
        },
        "subject": "Alihai Customer Message",
        "content": [
            {
                "type": "text/html",
                "value": mail_text
            }
        ]
    });

    send_mail(&mail_config).await
}
