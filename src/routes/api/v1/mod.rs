use hyper::Body;
use routerify::Router;

mod forms;
mod helpers;
mod types;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .post("/contactus", forms::handlers::contact_us_form_post)
        .build()
        .unwrap()
}
