extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper_native_tls;

use std::io;
use std::io::Read;
use hyper::{Client};
use tokio_core::reactor::Core;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use hyper::header::{UserAgent};
use hyper::server::{Request, Response};

use futures::future::Future;

// pub struct GitService;
// impl Service for GitService {
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;
//     type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

//     fn call(&self, req: Request) -> Self::Future {
//         let mut response = Response::new();
//         let mut core = Core::new().unwrap();
//         let handle = core.handle();
//         // let client = Client::configure()
//         //     // .keep_alive(true)
//         //     .build(&handle);

//         let ssl = NativeTlsClient::new().unwrap();
//         let connector = HttpsConnector::new(ssl);
//         let client = Client::Client::with_connector(connector);

//         match (req.method(), req.path()) {
//             (&Method::Get, "/nya-n") => {
//                 response.set_body("nya-n");
//             },
//             (&Method::Get, "/git") => {
//                 // let uri: hyper::Uri = "https://api.github.com/repos/rchaser53/vue-table-playground/commits".parse().unwrap();
//                 let uri: hyper::Uri = "http://google.com".parse().unwrap();
//                 // println!(1);
//                 // let mut req = Request::new(Method::Get, uri);
//                 // req.headers_mut().set(UserAgent::new("todo"));
//                 let mut resp = client.get(uri).send().unwrap();
//                 let mut body = vec![];
//                 resp.read_to_end(&mut body).unwrap();
//                 // let result = work.wait().unwrap();
//                 // let result = work.and_then(|x| { Ok(x) }).unwrap();


//                 // let _res = core.run(work).unwrap();
//                 response.set_body(String::from_utf8_lossy(&body));
//                 // response.set_body("gyaosu");
//                 // Box::new(futures::future::ok(Response::new().with_body(result.body())))
//             },
//             _ => {
//                 response.set_status(StatusCode::NotFound);
//             },
//         };
//         Box::new(futures::future::ok(response))
//     }
// }

pub fn main() {
    // let addr = "127.0.0.1:3000".parse().unwrap();
    // let server = Http::new().bind(&addr, || Ok(GitService)).unwrap();
    // server.run().unwrap();


    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut resp = client.get("https://google.com").send().unwrap();
    assert!(resp.status.is_success());
    let mut body = vec![];
    resp.read_to_end(&mut body).unwrap();
}