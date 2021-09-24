use crate::http::request::Request;
use crate::http::result::Res;

pub trait Handler: Copy + Send {
    fn handle(&self, req: &mut Request) -> Res<()>;
}

#[macro_export]
macro_rules! handler {
    ($name:ident  $code:block) => {
        #[derive(Copy, Clone)]
        pub struct $name();
        impl Handler for $name{
            fn handle(&self, req: &mut Request) -> Res<()>{
                let f :fn(req: &mut Request) -> Res<()> = $code;
                f(req)
            }
        }
    };
}

#[derive(Copy, Clone)]
pub struct BaseHandler<T: Handler> {
    pub handler: T
}



impl<T:Handler> Handler for BaseHandler<T>{
    fn handle(&self, req: &mut Request) -> Res<()> {
        self.handler.handle(req)?;
        let version = req.version.clone();
        let r = req.borrow_response_mut();
        let len = r.body.len();
        if len != 0{
            r.headers.insert("Content-Length".to_string(), len.to_string());
        }
        r.version = version;

        Ok(())
    }
}