use rand::Rng;
use std::{thread, time};
use futures::join;

async fn download_async(_url: &str){
    let sleep: u64 = rand::thread_rng().gen_range(100..3000);
    thread::sleep(time::Duration::from_millis(sleep));
}

async fn get_two_sites_async() {
    // Create two different "futures" which, when run to completion,
    // will asynchronously download the webpages.
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // Run both futures to completion at the same time.
    join!(future_one, future_two);
}

fn main() {
    println!("Hello, world!");
    let res = get_two_sites_async();
    let a = async {
        Ok(1)
    };
    // let res = join!(res, a);

}
