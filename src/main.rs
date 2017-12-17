use std::mem;

fn main() {
    let a: [i16; 3] = [1 , 2, 3];
    hoge(&a);

    for i in &a[1..2] {
        println!("{}", i);
    }

}

fn hoge(a: &[i16]) -> [i16; 2] {
    [a[0], a[2]]
}