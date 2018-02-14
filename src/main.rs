use std::error::Error;
// use core::result;
// use {Future, Poll, Async};
use std::{thread, time};
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

pub type Poll<T, E> = Result<Async<T>, E>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Async<T> {
    Ready(T),
    NotReady,
}

impl<T> Async<T> {
    pub fn map<F, U>(self, f: F) -> Async<U>
        where F: FnOnce(T) -> U
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

fn nyan () -> Poll<i16, Box<Error>> {
  Ok(Async::Ready(11))
}

struct Client {
  nyan: i32
}

impl Client {
  pub fn hoge(&self, a: i32) -> i32 {
    return self.nyan + a
  }
}

struct Runner {
    client: Arc<Client>
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

fn main() {
  let runnya = Runner{
    client: Arc::new(Client{ nyan: 13 })
  };

  runnya.run(vec![1, 2, 3])
}
