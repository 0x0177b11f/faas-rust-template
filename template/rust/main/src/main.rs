use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use std::{convert::Infallible, net::SocketAddr};

use handler;

#[tokio::main]
pub async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler::handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
