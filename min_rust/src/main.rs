#![no_std]
#![no_main]
#![feature(rustc_private)]
#![feature(lang_items)]

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Since we are passing a C string the final null character is mandatory.
    const HELLO: &'static str = "Hello, world!\n\0";
    // unsafe {
    //     libc::printf(HELLO.as_ptr() as *const _);
    // }
    -1
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}