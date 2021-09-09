struct A {
    A: i32,
}

impl std::cmp::PartialOrd for A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.A.partial_cmp(&other.A)
    }
}

impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let a = Box::new(1);
    println!("{}", a);
    let a = Cons(0, &Nil);
    let b = Cons(1, &a);
    let c = Cons(2, &b);
    println!("{:?}", c);

    let a = RcCons(0, Rc::new(RcNil));
    let a = Rc::new(a);
    let e = a.clone();
    let b = RcCons(1, a.clone());
    let c = RcCons(2, a.clone());
    println!("{:?}", c);
    println!("{:?}", b);
    thread_local! {}
}
use std::rc::Rc;

use List::{Cons, Nil};
#[derive(Debug)]
enum List<'a, T> {
    Cons(T, &'a List<'a, T>),
    Nil,
}

use RcList::{RcCons, RcNil};
#[derive(Debug)]
enum RcList<T> {
    RcCons(T, Rc<RcList<T>>),
    RcNil,
}
