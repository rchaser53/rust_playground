extern crate futures;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;
extern crate future_by_example;

use std::net::ToSocketAddrs;
use std::borrow::Cow;

use std::io::prelude::*;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter};

use futures::Future;
use future_by_example::new_example_future;
use futures::future::FutureResult;

// use native_tls::TlsConnector;
// use tokio_core::net::TcpStream;
// use tokio_core::reactor::Core;
// use tokio_tls::TlsConnectorExt;

use std::error::Error;
use std::{thread, time};

fn wait_return_future() -> FutureResult<i16, Box<Error>> {
  let three_thousand = time::Duration::from_millis(3000);
  let now = time::Instant::now();
  thread::sleep(three_thousand);

  futures::future::ok(28)
}

fn main() {
  let future = wait_return_future();
  println!("{}", future.wait().unwrap());
}