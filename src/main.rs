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
    let (tx, rx) = mpsc::channel::<()>();
    for &x in data.iter() {
      let tx = tx.clone();
      let lock_clone = lock.clone();
      thread::spawn(move || {
        let tempVal = rand::thread_rng().gen_range(100, 500);

        // thread::sleep(Duration::from_millis(tempVal));
        // lock_clone.fetch_and(false, Ordering::SeqCst);
        if (tempVal % 2 == 0) {
          println!("true");
          lock_clone.store(true, Ordering::Relaxed);
          // lock_clone.fetch_and(true, Ordering::SeqCst);
          return tx.send(());
        }
        lock_clone.store(false, Ordering::Relaxed);
        println!("false");
        return tx.send(());
      });
    }

    // thread::sleep(Duration::from_millis(50));

    while lock.load(Ordering::Acquire) {
    // while lock.load(Ordering::Relaxed) {
      println!("aaa");
      rx.recv().unwrap();
      // lock.fetch_and(rx.recv().unwrap(), Ordering::Relaxed);
    }


    // broke out of the loop, so we successfully acquired the lock!

    // ... scary data accesses ...

    // ok we're done, release the lock
    lock.store(false, Ordering::Release);
}