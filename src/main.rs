// #![feature(duration_extras)]
extern crate futures;
extern crate crossbeam;
extern crate num_cpus;
extern crate image;

// extern crate rand;
// extern crate time;
// #![feature(get_type_id)]

// use core::result;
// use std::cell::RefCell;
// use std::any::{Any, TypeId};
// use std::error::Error;
// use futures::future::{FutureResult, ok};
// use futures::future::FutureResult;

// use rand::Rng;
// use std::{thread};
// use std::time::Duration;
// use crossbeam::{Scope};
// use time::{SteadyTime};
// use std::fs::File;

use futures::{executor, Future, select_all, future};
use futures::future::{Map, lazy, FutureResult};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::cmp;
use image::*;
use std::fs::File;

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
    // let arr = &[-4, 1, 10, 25];
    // let max = find_max(arr, 0, arr.len());
    // assert_eq!(25, max);

    let mut img = image::open("nyan.png").unwrap();
    let (width, height) = hoge(&mut img);
    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for x in 0..(width) {
        for y in 0..(height) {
            let px = img.get_pixel(x, y).map(|v| v);
            buffer.put_pixel(x, y, px);
        }
    }
    let ref mut fout = File::create("hogyahogya.png").unwrap();
    ImageRgba8(buffer).save(fout, PNG).unwrap();
}

fn hoge<G>(image: &mut G) -> (u32, u32)
    where G: GenericImage<Pixel = Rgba<u8>> {
  image.dimensions()
}




// enum hoge {
//   nyan(num_cpus::get())
// }


fn find_max(arr: &[i32], start: usize, end: usize) -> i32 {
    // Perform sequential computation if there are only a few elements.
    let THRESHOLD: usize = num_cpus::get();
    if end - start <= THRESHOLD {
        return *arr.iter().max().unwrap();
    }

    let mid = start + (end - start) / 2;
    crossbeam::scope(|scope| {
        let left = scope.spawn(|| find_max(arr, start, mid));
        let right = scope.spawn(|| find_max(arr, mid, end));

        cmp::max(left.join(), right.join())
    })
}
// fn futureB() -> Box<Fn() -> FutureResult<i16, ()>> {
//   return Box::new(|| {
//     thread::sleep(Duration::from_millis(1500));
//     println!("{}", "b");
//     future::ok(2)
//   });
// }

// fn hoge () -> Box<FnOnce() -> Result<usize, ()>> {
//   Box::new(move || {
//     thread::sleep(Duration::from_millis(1200));
//     println!(2);
//     Ok(2)
//   })
// }

// #[derive(Default)]
// struct SomeOptions {
//     foo: i32,
//     bar: f32,
// }

// fn main() {
//     let options: SomeOptions = Default::default();

//     println!("{}", options.foo);
// }

// let a = executor::spawn(lazy(|| {
//   thread::sleep(Duration::from_millis(1600));
//   future::ok::<i16, ()>(2)
// }));
// .wait_future();
