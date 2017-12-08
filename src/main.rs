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

use hyper::{Client, Method, StatusCode};
use tokio_core::reactor::Core;

use hyper::header::{UserAgent};
use hyper::server::{Http, Request, Response, Service};
use hyper_tls::HttpsConnector;

use futures::Stream;
 use futures::Future;

fn main() {
    let mut sched = JobScheduler::new();

    // sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        create_request_object("nyan");
    // }));

    // loop {
    //     sched.tick();

    //     std::thread::sleep(Duration::from_millis(500));
    // }
}

fn create_request_object(path: &'static str) {
    // let mut core = Core::new().unwrap();
    // let client = Client::new(&core.handle());

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
                    .connector(HttpsConnector::new(4, &handle).unwrap())
                    .build(&handle);


    // let uri = "https://api.github.com/repos/rchaser53/vue-table-playground/commits".parse().unwrap();
    let uri = "https://connpass.com".parse().unwrap();

    let f = client.get(uri).map_err(|_err| ()).and_then(|resp| {
        resp.body().concat2().map_err(|_err| ()).map(|chunk| {
            let v = chunk.to_vec();
            let hoge = String::from_utf8_lossy(&v).to_string();
            println!("{}", hoge);
            hoge
        })
    });
    core.run(f);


    

    // let hoe = res.body().then(|result| {
    //     match result {
    //         Ok(e) => {
    //             Ok(e)
    //         },
    //         Err(e) => Err(e)
    //     }
    // }).concat2();

    // .map(|_res| {
    //     println!("Response: {:?}", _res);
    // });




    // let work = client.get(uri).map(|res| {
    //     println!("Response: {}", res.status());
    // });
}