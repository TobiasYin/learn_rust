extern crate proc_macro;
extern crate cc;

use proc_macro::TokenStream;
use std::fs;
use quote::quote;
use std::process::Command;

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    fs::create_dir("target/ext_c");
    fs::write("target/ext_c/demo.c", input.to_string()).unwrap();

    // cc::Build::new().file("target/ext_c/demo.c").out_dir("/target/ext_c").compile("libdemo.a");
    let compile = Command::new("gcc").args("-c target/ext_c/demo.c -o target/ext_c/demo.o".split(" ")).output().unwrap();
    let err = String::from_utf8(compile.stderr).unwrap();
    if err.len() != 0{
        panic!("c compile error: {}", err);
    }
    let ar = Command::new("ar").args("rcs target/ext_c/libdemo.a target/ext_c/demo.o".split(" ")).output().unwrap();
    let err = String::from_utf8(ar.stderr).unwrap();
    if err.len() != 0{
        panic!("c compile error: {}", err);
    }

    let r = quote!{
        #[link(name = "demo", kind="static")]
        extern "C" {
            fn test() -> i32;
        }
    };
    r.into()
}