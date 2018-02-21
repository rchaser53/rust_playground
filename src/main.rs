// #![feature(duration_extras)]
extern crate futures;
extern crate rand;
extern crate crossbeam;

// #![feature(get_type_id)]

// use core::result;
use futures::{Future, future};
// use std::cell::RefCell;
// use std::any::{Any, TypeId};
use rand::Rng;
// use std::error::Error;
// use futures::future::{FutureResult, ok};
use std::{thread, time};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::time::Duration;
use crossbeam::{Scope};
use futures::future::Map;
use futures::future::FutureResult;

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

fn futureA() -> FutureResult<i16, ()> {
  thread::sleep(Duration::from_millis(1000));
  println!("{}", "a");
  future::ok(1)
}

fn futureB() -> FutureResult<i16, ()> {
  thread::sleep(Duration::from_millis(1500));
  println!("{}", "b");
  future::ok(2)
}

fn main() {

  let a = futureA();
  let b = futureB();
  let pair = a.join(b);
  pair.wait();
  println!(1);
}