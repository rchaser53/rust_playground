pub fn coercions_slice_by_reference() {
  let tempArrayA = [1, 2, 3];

  // sliceに型を強制変換
  for i in &tempArrayA {
    println!("{:?}", i);
  }
}

pub fn coercions_slice_by_method() {
  let tempArrayA = [1, 2, 3];

  // sliceに型を強制変換
  for i in tempArrayA.iter() {
    println!("{:?}", i);
  }
}