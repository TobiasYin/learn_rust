use std::ops::Deref;
use std::fs::read_to_string;
use std::borrow::BorrowMut;
use std::cell::{RefCell, Cell, UnsafeCell};
use std::rc::Rc;

struct MyBox<T> {
    data: T,
    i: RefCell<i32>,
}

impl<T> MyBox<T> {
    pub fn new(data: T) -> Self {
        MyBox { data, i: RefCell::new(0) }
    }

    fn test(&self){
        println!("abc")
    }
}

// impl MyBox<String> {
//     pub fn new(data: String) -> Self {
//         MyBox { data }
//     }
// }


impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("{}", self.i.borrow());
        if *self.i.borrow() >= 5 {
            &self.data
        } else {
            *self.i.borrow_mut() += 1;
            &self
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop call")
    }
}


thread_local! {
     pub static FOO: RefCell<u32> = RefCell::new(1);

     #[allow(unused)]
     static BAR: RefCell<f32> = RefCell::new(1.0);
}
fn main() {
    println!("Hello, world!");
    let a = 1;
    let boxed = MyBox::new(&a);
    let data = *boxed;
    println!("{}", data);
    let b2 = MyBox::new(String::from(&String::from("sd")));
    std::mem::drop(b2);
    let b3 = MyBox::new(Box::new(1));
    println!("{}", *b3);

    let  b = RefCell::new(1);
    *b.borrow_mut() = 1;
    let mut d = b.borrow_mut();
    std::mem::drop(d);
    let c = b.borrow();
    println!("{}", c);

    let e = Rc::new(1);
    let x = Rc::strong_count(&e);

    let mut a = MyBox::new(());
    let r1 = e.clone();
    let r2 = Rc::clone(&e);
    let b = Rc::downgrade(&e);
    b.test();
    b.data = ();
}
