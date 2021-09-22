use hello_macro::HelloMacro;
use attr_macro::route;
use sql::sql;

#[derive(HelloMacro, Debug)]
struct Demo();

#[route(GET, "/")]
fn f1(){
    println!("hello, i'm f1")
}

fn never_ret() -> !{
    loop {
    }
}

fn main() {
    let d = Demo();
    let s = sql!(select * from data where id = 1);
    println!("{}", s);
    d.hello_macro();
    // never_ret();
    println!("Hello, world!");
    f1();

}
