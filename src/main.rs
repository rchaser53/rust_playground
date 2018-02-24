// #![feature(duration_extras)]
extern crate futures;
extern crate rand;
extern crate crossbeam;
extern crate futures_cpupool;
extern crate time;

// #![feature(get_type_id)]

// use core::result;
// use std::cell::RefCell;
// use std::any::{Any, TypeId};
// use std::error::Error;
// use futures::future::{FutureResult, ok};
// use futures::future::FutureResult;

use rand::Rng;
use futures::{executor, Future, select_all, future};
use std::{thread};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::time::Duration;
use crossbeam::{Scope};
use futures::future::{Map, lazy, FutureResult};
use time::{SteadyTime};

struct Client {
  hoge: [i32; 3]
}

#[derive(Clone)]
struct Outer {
    client: Arc<Mutex<Client>>
}

fn nyan(outer: &Outer) -> &Outer {
  let a = Arc::new(AtomicUsize::new(0));
  for (i, &num) in vec![1,2,3].iter().enumerate() {
    let mut a_clone = a.clone();
    crossbeam::scope(|scope| {
      scope.spawn(move || {
        outer.client.lock().unwrap().hoge[i] = num * 2;
        a_clone.fetch_add(1, Ordering::Release);
      });
    });
  }
  while a.load(Ordering::Acquire) != 3 {}
  return outer;
}

// fn futureB() -> Box<Fn() -> FutureResult<i16, ()>> {
//   return Box::new(|| {
//     thread::sleep(Duration::from_millis(1500));
//     println!("{}", "b");
//     future::ok(2)
//   });
// }

fn hoge () -> Box<FnOnce() -> Result<usize, ()>> {
  Box::new(move || {
    thread::sleep(Duration::from_millis(1200));
    println!(2);
    Ok(2)
  })
}

#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

fn main() {
    let options: SomeOptions = Default::default();

    println!("{}", options.foo);
}

// let a = executor::spawn(lazy(|| {
//   thread::sleep(Duration::from_millis(1600));
//   future::ok::<i16, ()>(2)
// }));
// .wait_future();
