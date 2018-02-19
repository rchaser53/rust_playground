#![feature(duration_extras)]
extern crate futures;
extern crate rand;

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

fn main() {
    let a = Arc::new(AtomicUsize::new(0));
    let a_clone = a.clone();

    let b = Arc::new(AtomicUsize::new(0));
    let b_clone = b.clone();

    let c = Arc::new(AtomicUsize::new(0));
    let c_clone = c.clone();

    let d = Arc::new(AtomicUsize::new(0));
    let d_clone = d.clone();

    thread::spawn(move || {
      // thread::sleep(Duration::from_nanos(1));
      
      // a_clone.store(1, Ordering::Relaxed);
      // c.store(b.load(Ordering::Relaxed), Ordering::Relaxed);
      a_clone.store(1, Ordering::Release);
      c.store(b.load(Ordering::Acquire), Ordering::Release);
    });
    thread::spawn(move || {
      // b_clone.store(1, Ordering::Relaxed);
      // d.store(a.load(Ordering::Relaxed), Ordering::Relaxed);
      b_clone.store(1, Ordering::Release);
      d.store(a.load(Ordering::Acquire), Ordering::Release);
    });

    thread::sleep(Duration::from_millis(500));
    println!("c:{} d:{}", c_clone.load(Ordering::Acquire), d_clone.load(Ordering::Acquire))
    // println!("c:{} d:{}", c_clone.load(Ordering::Relaxed), d_clone.load(Ordering::Relaxed))
}