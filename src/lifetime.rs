pub fn return_vec() -> Vec<i16> {
    vec![1,2,3]
}

pub fn return_array() -> [i16; 3] {
    [1, 2, 3]
}

// pub fn return_slice() -> [i16] {
//     &[1, 2, 3]
// }

pub fn return_slice<'a>(slice: &'a [i16]) -> &'a [i16] {
    slice
}

pub fn return_slice_implessly(slice: &[i16]) -> &[i16] {
    slice
}