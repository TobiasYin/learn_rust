#![feature(const_for)]
#![feature(const_mut_refs)]

#[inline(never)]
const fn f1(i: i32) -> i32 {
    1 + 3 * i
}

#[inline(never)]
const fn a(i: i32) -> i32 {
    let data = f1(i);
    data + i
}

fn main() {
    let data1 = a(3);
    const DATA2: i32 = a(5);
    
    println!("Hello, world!{}, {}", data1, DATA2);
}
