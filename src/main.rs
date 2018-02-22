// #![feature(duration_extras)]
extern crate futures;
extern crate rand;
extern crate crossbeam;
extern crate futures_cpupool;
extern crate time;

// #![feature(get_type_id)]

// use core::result;
use futures::{executor, Future, select_all, future};
// use std::cell::RefCell;
// use std::any::{Any, TypeId};
use rand::Rng;
// use std::error::Error;
// use futures::future::{FutureResult, ok};
use std::{thread};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::time::Duration;
use crossbeam::{Scope};
use futures::future::{Map, lazy, FutureResult};
use time::{SteadyTime};
// use futures::future::FutureResult;

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
  thread::sleep(Duration::from_millis(3000));
  println!(55);
  return future::ok(1);
}
fn futureB() -> FutureResult<i16, ()> {
  thread::sleep(Duration::from_millis(1800));
  println!(66);
  return future::ok(1);
}

// fn futureB() -> Box<Fn() -> FutureResult<i16, ()>> {
//   return Box::new(|| {
//     thread::sleep(Duration::from_millis(1500));
//     println!("{}", "b");
//     future::ok(2)
//   });
// }
use futures_cpupool::CpuPool;

fn main() {
  let start = SteadyTime::now();
  // let a = futureA();
  // let b = futureB();
  // let pair = a().join(b());
  // let aa = pair.then(|x| {
  //   println!(222);
  //   future::ok::<i16, ()>(1)
  // });

  // let a = lazy(|| {
  //   thread::sleep(Duration::from_millis(1200));
  //   println!(0);
  //   future::ok::<i16, ()>(1)
  // });

  let thread_pool = CpuPool::new(4);

  // let a = executor::spawn(lazy(|| {
  //   thread::sleep(Duration::from_millis(1600));
  //   println!(0);
  //   future::ok::<i16, ()>(2)
  // }));
  // .wait_future();
  println!(0);
  let a = thread_pool.spawn_fn(move || -> Result<usize, ()> {
            thread::sleep(Duration::from_millis(1200));
            println!(2);
            Ok(2)
          });
  let b = thread_pool.spawn_fn(move || -> Result<usize, ()> {
            thread::sleep(Duration::from_millis(200));
            println!(3);
            Ok(2)
          });

  let hoge = vec![a, b];
  // let hoge = vec![futureA(), futureB()];
  // let one = select_all(hoge);
  let mut one = future::select_all(hoge);
  while let Ok((value, _idx, remaining)) = one.wait() {
      // println!("Future #{} finished", value);
      if remaining.is_empty() {
          break;
      }
      one = future::select_all(remaining);
  }

  // let pair = a.join(b);
  println!(1);
  thread::sleep(Duration::from_millis(1500));
  println!("{}", SteadyTime::now() - start);
  // println!("{:?}", pair.wait().unwrap());
}