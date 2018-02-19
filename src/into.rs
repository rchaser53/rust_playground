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