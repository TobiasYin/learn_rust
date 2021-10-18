use rand::Rng;
use std::{thread, time};
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::executor::block_on;
use futures::{Future, join};
use futures::future::join_all;

static mut now_id: i32 = 0;

struct SimpleFuture<T: FnOnce() -> O + 'static, O> {
    func: Option<T>,
    times: i32,
    id: i32,
}

impl<T: FnOnce() -> O + 'static, O> SimpleFuture<T, O> {
    fn new(f: T) -> SimpleFuture<T, O> {
        let id  = unsafe {
            now_id += 1;
            now_id
        };
        return SimpleFuture{
            func: Some(f),
            times: 0,
            id
        };
    }
}

impl<T: FnOnce() -> O + 'static, O> Future for SimpleFuture<T, O> {
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = unsafe {
            self.get_unchecked_mut()
        };
        let thread_id = thread::current().id();
        s.times += 1;
        println!("call poll on {}, times: {}, thread: {}", s.id, s.times, s.id);
        if s.times <= 5{
            cx.waker().clone().wake();
            Poll::Pending
        }else {
            Poll::Ready((s.func.take().unwrap())())
        }
    }
}

fn download_async( url: &str) {
    println!("start task: {}", url);
    let sleep: u64 = rand::thread_rng().gen_range(100..3000);
    thread::sleep(time::Duration::from_millis(sleep));
}

async fn get_two_sites_async() {
    // Create two different "futures" which, when run to completion,
    // will asynchronously download the webpages.
    let future_one = SimpleFuture::new(|| download_async("https://www.foo.com"));
    let future_two = SimpleFuture::new(|| download_async("https://www.bar.com"));

    let mut futures:Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(future_one), Box::pin(future_two)];
    for i in 0..10{
        futures.push(Box::pin(SimpleFuture::new(|| download_async("https://www.bar.com"))));
    }

    // Run both futures to completion at the same time.
    join_all(futures);
}

fn main() {
    println!("Hello, world!");
    let res = get_two_sites_async();
    let a = async {
        Ok(1)
    };
    block_on(res);
    let r: Result<i32, &str> = block_on(a);
    println!("{:?}", r)
    // let res = join!(res, a);
}
