use hyper::Body;
use routerify::Router;

mod forms;
mod helpers;
mod types;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder().scope("/forms", forms::router()).build().unwrap()
}
