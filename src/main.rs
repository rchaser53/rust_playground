extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate job_scheduler;

mod git_serve;

use std::io::{self, Write};

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

use hyper::{Client, Body, Method, StatusCode};
use hyper::client::{Client as HyperClient, HttpConnector};
use tokio_core::reactor::Core;

use hyper::header::{UserAgent};
use hyper::server::{Http, Request, Response, Service};
use hyper_tls::HttpsConnector;

use futures::{Stream, Future};

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        create_request_object("https://api.github.com/repos/rchaser53/vue-table-playground/commits");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}

struct HttpRequestHandler {
    client: hyper::Client<HttpsConnector<HttpConnector>>
}
impl HttpRequestHandler {
    fn new(core: &Core) -> Self {
        let handle = core.handle();
        let client = Client::configure()
                        .connector(HttpsConnector::new(4, &handle).unwrap())
                        .build(&handle);

        HttpRequestHandler {
            client: client
        }
    }
}

fn create_request_object(path: &'static str) {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = HttpRequestHandler::new(&core).client;

    let mut req = Request::new(Method::Get, path.parse().unwrap());
    req.headers_mut().set(UserAgent::new("todo"));

    let work = client.request(req)
                    .map((|resp| {
                        if resp.status() != StatusCode::Ok {
                            println!(222);
                            process::exit(1);
                        }
                        resp
                    }))
                    .map_err(|_err| ())
                    .and_then(|resp| {
                        resp.body().concat2().map_err(|_err| ()).map(|chunk| {
                            let v = chunk.to_vec();
                            println!("{}", String::from_utf8_lossy(&v).to_string());
                        })
                    });
    core.run(work);
}