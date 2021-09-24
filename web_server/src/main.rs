#[macro_use]
extern crate lazy_static;

use http::*;
use std::thread::sleep;
use std::time::Duration;

mod http;

handler!{
    MyHandler {
        |req: &mut Request| -> Res<()> {
            req.set_response_body("hello, form my resp".as_bytes());
            sleep(Duration::from_millis(1000));
            Ok(())
        }
    }
}

fn main() {
    listen("127.0.0.1:8081", MyHandler())
}




