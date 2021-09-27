use std::ptr::slice_from_raw_parts;

unsafe fn f1(x: *mut i32) {
    *x += 1;
}

use libc;
use std::ops::{Add, Deref};
use std::fmt::{self, Display, Formatter, Debug};

extern "C" {
    fn abs(input: i32) -> i32;
    fn malloc(size: usize) -> usize;
    fn free(ptr: usize);
    fn fork();
}

extern "C" {
    fn hello_demo(d: Demo);
}


#[derive(Debug)]
struct Demo {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Display for Demo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

trait OutlinePrint: ToString {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl <T: ToString> OutlinePrint for T{}

struct Wrapper<T: Display>(Vec<T>);

impl<T: Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|i| i.to_string()).reduce(|i, j| {format!("{}, {}", i, j)}).unwrap_or(String::new()))
    }
}

impl<T: Display> Deref for Wrapper<T>{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    let demo = Demo { a: 1, b: 2, c: 4, d: 5 };
    demo.outline_print();
    println!("{}", demo);
    unsafe {
        hello_demo(demo);
        println!("Absolute value of -3 according to C: {}", abs(-3));
        let x = malloc(8);
        let p = x as *mut i64;
        *p = 1;
        println!("data: {}", *p);
        free(x);
        println!("data: {}", *p);
    }

    let x = 1;
    let z = 4;
    let y = &x as *const i32 as usize as *mut i32;
    unsafe {
        // println!("x: {}", *y);
        *y = 3;
        f1(y);
        *((y as usize + 4usize) as *mut i32) = 5;
    }

    println!("x: {}", x);
    println!("z: {}", z);
    let mut v = vec![1, 2, 3, 4, 5];
    // let r = &mut v[0..2];
    let (a, b) = v.split_at_mut(3);
    // println!("{}", r[0]);
    println!("{}", a[0]);
    println!("{}", b[0]);
    let demo = Demo { a: 1, b: 2, c: 3, d: 4 };
    let uptr = &demo as *const Demo as usize;
    unsafe {
        let s = slice_from_raw_parts(uptr as *const i32, 4);
        let s = &*s;
        for i in s.iter() {
            println!("{}", i);
        }
    }
}
