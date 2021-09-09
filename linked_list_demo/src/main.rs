
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

fn main() {
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
}
