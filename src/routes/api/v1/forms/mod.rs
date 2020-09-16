use hyper::Body;
use routerify::Router;

mod controllers;
mod handlers;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .post("/contactus", handlers::contact_us_form_post)
        .build()
        .unwrap()
}
