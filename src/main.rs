use std::mem;

fn main() {}

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