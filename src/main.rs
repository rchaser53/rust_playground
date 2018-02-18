#![feature(get_type_id)]
use std::error::Error;
// use core::result;
// use {Future, Poll, Async};
use std::{thread, time};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::cell::RefCell;
use std::any::{Any, TypeId};

// fn main() {
//   let pair = Arc::new((Mutex::new(false), Condvar::new()));
//   let pair2 = pair.clone();

//   thread::spawn(move|| {
//       let &(ref lock, ref cvar) = &*pair2;
//       let mut started = lock.lock().unwrap();
//       *started = true;

//       // cvar.notify_one();
//   });

//   let &(ref lock, ref cvar) = &*pair;
//   let mut started = lock.lock().unwrap();
//   while !*started {
//     println!(1);
//     started = cvar.wait(started).unwrap();
//   }
//   println!(2);
// }


// fn main() {
  // println!(
    // "{:?}",
    // convert_vec_u8("hello".to_string())
  //   convert_vec_u8("hello")
  // );
  // let hoge = 21;

  // let mut abc = Nyan{hoge: "aa"};
  // abc.poll_stream_notify(&Nyantwo{ hoge: 11 });
// }

  //  let bytes = b"hello".to_vec();
fn convert_vec_u8<T: Into<Vec<u8>>>(s: T) -> Vec<u8> {
   s.into()
}

pub struct Nyan<T> {
  hoge : T,
}

#[derive(Clone)]
pub struct Nyantwo {
  hoge: i16
}

// impl <T: ?Sized>Nyan<T> {
impl <T>Nyan<T> {
  pub fn poll_stream_notify<N>(&mut self,
                                notify: &N)
                                -> Nyantwo
      where N: Clone + Into<Nyantwo>
  {
      notify.clone().into()
  }
}