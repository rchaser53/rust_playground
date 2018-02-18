fn is_string(s: &Any) -> bool {
    TypeId::of::<String>() == s.get_type_id()
}

fn is_A<T: 'static>(s: &Any) -> bool {
    TypeId::of::<T>() == s.get_type_id()
}

fn main() {
  let hoge = 21;
  println!(
    "{}", is_A::<i32>(&hoge)
  );
}

