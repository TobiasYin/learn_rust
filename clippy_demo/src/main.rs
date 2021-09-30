#![feature(negative_impls)]

use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::pin::{Pin};
use std::marker::Unpin;

struct PinWrapper<T> (T);

impl<T: Display> Display for PinWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return self.deref().fmt(f)
    }
}

impl<T> PinWrapper<T> {
    fn new(data: T) -> PinWrapper<T>{
        return PinWrapper(data);
    }
}

impl<T> Deref for PinWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0
    }
}

impl<T> DerefMut for PinWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0
    }
}

impl<T> !Unpin for PinWrapper<T> {}



fn main() {
    let mut data = PinWrapper::new(String::from("hello"));
    let mut p = Pin::new(&mut data);
    let mut data1 = PinWrapper::new(String::from("hell2"));
    let mut p1 = Pin::new(&mut data1);
    p.deref_mut();
    std::mem::swap(&mut p, &mut p1);
    println!("{}, {}", p, p1);
    std::mem::forget()
}
