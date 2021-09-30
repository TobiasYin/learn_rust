#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use rand::Rng;
use std::{thread, time};
use futures::join;
async fn download_async(_url: &str) {
    let sleep: u64 = rand::thread_rng().gen_range(100..3000);
    thread::sleep(time::Duration::from_millis(sleep));
}
async fn get_two_sites_async() {
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");
    {
        use ::futures_util::__private as __futures_crate;
        {
            let mut _fut0 = __futures_crate::future::maybe_done(future_one);
            let mut _fut1 = __futures_crate::future::maybe_done(future_two);
            __futures_crate::future::poll_fn(
                move |__cx: &mut __futures_crate::task::Context<'_>| {
                    let mut __all_done = true;
                    __all_done &= __futures_crate::future::Future::poll(
                        unsafe { __futures_crate::Pin::new_unchecked(&mut _fut0) },
                        __cx,
                    )
                    .is_ready();
                    __all_done &= __futures_crate::future::Future::poll(
                        unsafe { __futures_crate::Pin::new_unchecked(&mut _fut1) },
                        __cx,
                    )
                    .is_ready();
                    if __all_done {
                        __futures_crate::task::Poll::Ready((
                            unsafe { __futures_crate::Pin::new_unchecked(&mut _fut0) }
                                .take_output()
                                .unwrap(),
                            unsafe { __futures_crate::Pin::new_unchecked(&mut _fut1) }
                                .take_output()
                                .unwrap(),
                        ))
                    } else {
                        __futures_crate::task::Poll::Pending
                    }
                },
            )
            .await
        }
    };
}
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Hello, world!\n"],
            &match () {
                () => [],
            },
        ));
    };
    let res = get_two_sites_async();
    let a = async { Ok(1) };
}
