extern crate futures;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;

use std::io;
use std::net::ToSocketAddrs;
use std::borrow::Cow;

use std::io::prelude::*;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter};

use futures::Future;
use native_tls::TlsConnector;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_tls::TlsConnectorExt;

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
  // buffer.write(b"some bytes").unwrap();

  // stream.
  // buffer.flush().unwrap();
  // let mut buffer = String::new();
  // f.read_to_string(&mut buffer).unwrap();

  // println!("{}", buffer);

  // buffer = buffer + "hoge";

  // println!("{}", buffer);

  // let mut buffer = File::create("aaa.txt").unwrap();


  // let five = String::from(buffer);
  // println!(stringify!(buffer));

  // println!("{}", buffer.iter().fold(String::new(), |acc, &arg| acc + arg));

  // println!("{:?}", buffer.to_string());

  // let mut buffer = File::create("foo.txt").unwrap();
  // buffer.write_all(b"some bytes").unwrap();

  // File.
}

// assert!(resp.status.is_success());

  // let f = File::open("input.txt").unwrap();
  // let mut buf_reader = BufReader::new(&f);
  // let mut buffer = Vec::new();
  // buf_reader.read_to_end(&mut buffer).unwrap();
  // println!("{}", std::str::from_utf8(&buffer).unwrap());
