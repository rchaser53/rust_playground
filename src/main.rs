#![feature(get_type_id)]
use std::error::Error;
// use core::result;
// use {Future, Poll, Async};
use std::{thread, time};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

pub type Poll<T, E> = Result<Async<T>, E>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Async<T> {
    Ready(T),
    NotReady,
}

impl<T> Async<T> {
    pub fn map<F, U>(self, f: F) -> Async<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Async::Ready(t) => Async::Ready(f(t)),
            Async::NotReady => Async::NotReady,
        }
    }

    pub fn is_ready(&self) -> bool {
        match *self {
            Async::Ready(_) => true,
            Async::NotReady => false,
        }
    }

    pub fn is_not_ready(&self) -> bool {
        !self.is_ready()
    }
}

impl<T> From<T> for Async<T> {
    fn from(t: T) -> Async<T> {
        Async::Ready(t)
    }
}

fn nyan() -> Poll<i16, Box<Error>> {
    Ok(Async::Ready(11))
}

struct Client {
    nyan: i32,
}

impl Client {
    pub fn hoge(&self, a: i32) -> i32 {
        return self.nyan + a;
    }
}

struct Runner {
    client: Arc<Client>,
}

impl Runner {
    fn run(&self, data: Vec<i32>) {
        let (tx, rx) = mpsc::channel();
        for &x in data.iter() {
            let mut client = self.client.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                tx.send(client.hoge(x));
            });
        }

        for i in 0..data.len() {
            println!("{:?}", rx.recv());
        }
    }
}

use std::cell::RefCell;

// fn main() {
//   thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

//   FOO.with(|f| {
//       assert_eq!(*f.borrow(), 1);
//       *f.borrow_mut() = 2;
//   });

//   thread::spawn(move|| {
//       FOO.with(|f| {
//           assert_eq!(*f.borrow(), 1);
//           *f.borrow_mut() = 3;

//           println!("{}", f.borrow())    // 3
//       });
//   });

//   FOO.with(|f| {
//     assert_eq!(*f.borrow(), 2);
//   });
// }

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


use std::any::{Any, TypeId};

fn is_string(s: &Any) -> bool {
    TypeId::of::<String>() == s.get_type_id()
}

fn is_A<T: 'static>(s: &Any) -> bool {
    TypeId::of::<T>() == s.get_type_id()
}

fn main() {
  // println!(
    // "{:?}",
    // convert_vec_u8("hello".to_string())
  //   convert_vec_u8("hello")
  // );
  // let hoge = 21;
  let hoge = 21;
  println!(
    "{}", is_A::<i32>(&hoge)
  );

  // let mut abc = Nyan{hoge: "aa"};
  // abc.poll_stream_notify(&Nyantwo{ hoge: 11 });
}

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