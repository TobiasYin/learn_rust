#![feature(negative_impls)]

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::borrow::Borrow;

trait AA {
    fn hello();
}

trait BB{
    fn hello();
}

struct A{

}

impl AA for A{
    fn hello() {
        println!("hello form AA")
    }
}

impl BB for A{
    fn hello() {
        println!("hello form BB")
    }
}

fn main() {
    let (tx, rx) = channel();
    let r = Rc::new(1);
    thread::spawn(move ||{
        tx.send(String::from("hello")).unwrap()
    });
    let res = rx.recv().unwrap();
    println!("{}", res);
    let data = A{};
    <A as AA>::hello();
    //
    // let var1:Box<dyn AA> = Box::new(data);
    // var1.hello()
    //
}
