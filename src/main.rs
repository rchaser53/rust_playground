// #![feature(duration_extras)]
extern crate futures;
extern crate rand;
extern crate crossbeam;

// #![feature(get_type_id)]

// use core::result;
// use futures::{Future};
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

fn main() {
  let outer = Outer{
    client: Arc::new(Mutex::new(Client{
      hoge: [0, 0, 0]
    }))
  };
  println!("{:?}", nyan(&outer).client.lock().unwrap().hoge);
}