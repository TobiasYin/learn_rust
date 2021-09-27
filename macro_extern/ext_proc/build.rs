fn main() {
    println!("cargo:rustc-link-lib=dylib=demo");
    println!("cargo:rustc-link-search=native=/Users/tobias/projects/rust-demos/macro_extern/target/ext_c/");
}