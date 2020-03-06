use hyper::{Body, Request, Response};
use std::convert::Infallible;

const PHRASE: &str = "Hello, World!";

pub async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(PHRASE.into()))
}
