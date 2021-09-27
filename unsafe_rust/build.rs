fn main() {
    println!("cargo:rustc-link-lib=static=demo");
    println!("cargo:rustc-link-search=native=/Users/tobias/projects/rust-demos/unsafe_rust/src/c_src/");
}

// gcc -c demo_lib.c
// ar rcs libdemo.a demo_lib.o