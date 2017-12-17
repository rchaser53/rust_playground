// #[derive(Debug, PartialEq)]
// struct Hoge {}

fn main() {
let tempHogeA: Vec<i16> = Vec::new();
let tempHogeB: Vec<i16> = tempHogeA;
assert_eq!(tempHogeA, tempHogeB);
}

// fn main() {
//     let tempArrayA: [i16; 3] = [1, 2, 3];
//     let tempArrayB: [i16; 3] = tempArrayA;
//     assert_eq!(tempArrayA, tempArrayB);
// }

fn return_string() -> String {
    "abc".to_string()
}

fn return_static_str() -> &'static str {
    "abc"
}

// fn main() {
    // let mut temp: [i16;3] = [1,2,3];
    // let mut temp = vec![1,2,3];
//     put_array(&mut temp);
//     println!("{:?}", temp);
// }

// lifetimeが指定してやれるのでOK
// fn put_array<'a>(nyan: &'a mut [i16]) -> &'a mut [i16] {
//     nyan[1] = 13;
//     return nyan;
// }

fn mut_and_dereference() {
    let mut a: [i16; 3] = [1 , 2, 3];
    for i in &mut a {
        *i += 5;
    }
    println!("{:?}", a);
}

// fn hoge(a: &[i16]) -> [i16; 2] {
//     [a[0], a[2]]
// }

fn head_ascii(str: &str) -> &str {
  &str[0..1]
}