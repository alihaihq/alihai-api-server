use crate::constants;
use crate::types::AppInfo;
use crate::utils;
use hyper::header::{self, HeaderValue};
use hyper::{Body, Request, Response};
use routerify::RequestInfo;
use routerify::{Middleware, Router};

mod api;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .middleware(Middleware::post_with_info(cors_middleware)) // Use customized cors middleware.
        .get("/", home_get)
        .scope("/api", api::router())
        .err_handler(error_handler)
        .build()
        .unwrap()
}

async fn cors_middleware(mut res: Response<Body>, req_info: RequestInfo) -> crate::Result<Response<Body>> {
    let headers = res.headers_mut();

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET,HEAD,POST,PUT,DELETE,CONNECT,OPTIONS,TRACE,PATCH"),
    );

    if let Some(requested_headers) = req_info.headers().get(header::ACCESS_CONTROL_REQUEST_HEADERS) {
        headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, requested_headers.clone());
    } else {
        headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    }

    Ok(res)
}

async fn logger_middleware(req: Request<Body>) -> crate::Result<Request<Body>> {
    info!(
        "{} {} {}",
        utils::extract_client_ip_from_req(&req),
        req.method(),
        req.uri()
    );
    Ok(req)
}

async fn home_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    resp_200!(AppInfo {
        name: constants::APP_NAME,
        version: constants::APP_VERSION,
        description: constants::APP_DESCRIPTION,
    })
}

async fn error_handler(err: routerify::Error) -> Response<Body> {
    error!("{}", err);
    resp_500!("{}", err).expect("Couldn't create a response while handling the server error")
}
