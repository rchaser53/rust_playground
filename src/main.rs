extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;

use hyper::{Client, Method, StatusCode};
use tokio_core::reactor::Core;

use hyper::header::{ContentLength, UserAgent};
use hyper::server::{Http, Request, Response, Service};

use futures::future::Future;

pub struct GitService;
impl Service for GitService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .keep_alive(true)
            .build(&handle);

        match (req.method(), req.path()) {
            (&Method::Get, "/nya-n") => {
                response.set_body("nya-n");
            },
            (&Method::Get, "/git") => {
                let mut req = Request::new(Method::Get, "https://api.github.com/repos/rchaser53/vue-table-playground/commits".parse().unwrap());
                req.headers_mut().set(UserAgent::new("todo"));
                let work = client.request(req);
                let _res = core.run(work).unwrap();
                response.set_body(_res.body());
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };
        Box::new(futures::future::ok(response))
    }
}

pub fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(GitService)).unwrap();
    server.run().unwrap();
}