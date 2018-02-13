use std::error::Error;
use std::process::exit;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Aaa<T> {
  a(T),
  b
}

fn getAaa <T>(nyan: Aaa<T>) -> Option<T> {
  match nyan {
    Aaa::a(t) => Some(t),
    Aaa::b => {
      None
    }
  }
}

fn hoge () {
  // println!("{:?}", nyan().unwrap());
  println!("{}", getAaa(Aaa::a(2)).unwrap());
  // println!("{}", getAaa::<i16>(Aaa::b).expect("nya-n"));
  println!("{:?}", getAaa::<i16>(Aaa::b).ok_or("nyanyanya"));
}