extern crate futures;
extern crate rand;


// #![feature(get_type_id)]

// use core::result;
// use futures::{Future};
// use std::cell::RefCell;
// use std::any::{Any, TypeId};
use rand::Rng;
use std::error::Error;
use futures::future::{FutureResult, ok};
use std::{thread, time};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::time::Duration;

fn main() {
    let lock = Arc::new(AtomicBool::new(true)); // value answers "am I locked?"

    let data = vec![1,2,3];
    let (tx, rx) = mpsc::channel::<i16>();
    for &x in data.iter() {
      let tx = tx.clone();

      thread::spawn(move || {
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(100, 500)));
        println!("{}", x);
        return tx.send(1);
      });
    }

    for i in 0..data.len() {
      let _ = rx.recv().unwrap();
    }

    lock.fetch_and(false, Ordering::SeqCst);
    while lock.compare_and_swap(false, true, Ordering::Acquire) {
    // while lock.compare_and_swap(false, true, Ordering::Relaxed) {
      rx.recv().unwrap();
      println!("aaa");
    }


    // aa.wait();
    // broke out of the loop, so we successfully acquired the lock!

    // ... scary data accesses ...

    // ok we're done, release the lock
    lock.store(false, Ordering::Release);
}