use std::error::Error;

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

fn main() {
  nyan().unwrap().map(|a| {
    println!("{}", a);
  });
}
