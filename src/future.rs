extern crate futures;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;

use std::net::ToSocketAddrs;
use std::borrow::Cow;

use std::io::prelude::*;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter};

use futures::Future;
use futures::future::FutureResult;

// use native_tls::TlsConnector;
// use tokio_core::net::TcpStream;
// use tokio_core::reactor::Core;
// use tokio_tls::TlsConnectorExt;

use std::error::Error;
use std::{thread, time};

fn wait_return_future(a: i32) -> FutureResult<i32, Box<Error>> {
  let three_thousand = time::Duration::from_millis(1000);
  let now = time::Instant::now();
  thread::sleep(three_thousand);

  futures::future::ok(a)
}

fn add<'a, A, B>(a: A, b: B) -> Box<Future<Item=i32, Error=A::Error> + 'a>
    where A: Future<Item=i32> + 'a,
          B: Future<Item=i32, Error=A::Error> + 'a,
{
    Box::new(a.join(b).map(|(a, b)| a + b))
}

fn main() {
  let futureA = wait_return_future(1);
  let futureB = wait_return_future(2);
  println!("{}", add(futureA, futureB).wait().unwrap());
}