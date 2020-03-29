use futures::StreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::io;
use std::net::SocketAddr;

use boulder_bot::process_push_notification;

async fn handle(req: Request<Body>) -> io::Result<Response<Body>> {
    let (_parts, mut body) = req.into_parts();
    let mut raw_msg = Vec::<u8>::with_capacity(2048);
    while let Some(Ok(b)) = body.next().await {
        raw_msg.extend_from_slice(&b);
    }
    let resp_string = process_push_notification(raw_msg);
    let mut respon = Response::new(Body::from(resp_string));
    respon
        .headers_mut()
        .append("content-type", "application/json".parse().unwrap());
    Ok(respon)
    // Ok(Response::new(Body::empty()))
}
#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 15562));

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(make_service);
    server.await.ok();
    io::Result::Ok(())
}
