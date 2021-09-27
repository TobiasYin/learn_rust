use ext_proc::c;

c!{
    int test(){
        return 1;
    }
}

fn main() {
    unsafe {
        println!("Hello, world! {}", test());
    }
}
