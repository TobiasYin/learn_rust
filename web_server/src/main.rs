#[macro_use]
extern crate lazy_static;

mod buf_reader;
mod http;

use http::*;

handler!{
    MyHandler {
        |req: &mut Request| -> Res<()> {
            req.set_response_body("hello, form my resp".as_bytes());
            Ok(())
        }
    }
}

fn main() {
    listen("127.0.0.1:8081", MyHandler())
}




