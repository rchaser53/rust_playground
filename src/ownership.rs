/* 配列はプリミティブ。Vecは非プリミティブ */

// pub fn copy_and_assert_vec() {
//     let tempVecA: Vec<i16> = Vec::new();
//     let tempVecB: Vec<i16> = tempVecA;
//     assert_eq!(tempVecA, tempVecB);
// }

pub fn copy_and_assert_array() {
    let tempArrayA: [i16; 3] = [1, 2, 3];
    let tempArrayB: [i16; 3] = tempArrayA;
    assert_eq!(tempArrayA, tempArrayB);
}