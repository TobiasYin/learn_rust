use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

struct State {
    counter: Arc<AtomicI32>,
    last_print: Arc<AtomicI32>,
    handlers: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl State {
    fn new() -> State {
        let handlers = Arc::new(Mutex::new(Vec::<JoinHandle<()>>::new()));
        let counter = Arc::new(AtomicI32::new(0));
        let last_print = Arc::new(AtomicI32::new(0));
        State {
            counter,
            handlers,
            last_print,
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            handlers: self.handlers.clone(),
            counter: self.counter.clone(),
            last_print: self.last_print.clone(),
        }
    }
}

fn open_threads(i: i32, s: State) {
    if i <= 0 {
        let c = s.counter.load(Ordering::SeqCst);
        if c % 100 == 0 {
            let r = s.last_print.swap(c, Ordering::SeqCst);
            if r == c {
                return;
            }
            println!("now: {}", c);
        }
        return;
    }
    s.counter.fetch_add(1, Ordering::SeqCst);
    let mut v = Vec::new();
    for _ in 0..i {
        let clone_state = s.clone();
        let j = i - 10;
        let h = thread::spawn(move || {
            open_threads(j, clone_state);
        });
        v.push(h);
    }
    // {
    //     let l = s.handlers.lock();
    //     let mut mv = l.unwrap();
    //     v.into_iter().for_each(|h| {
    //         mv.push(h);
    //     });
    // }

    let c = s.counter.fetch_sub(1, Ordering::SeqCst);
    if c % 100 == 0 {
        let r = s.last_print.swap(c, Ordering::SeqCst);
        if r == c {
            return;
        }
        println!("now: {}", c);
    }
}

fn main() {
    let mut v = Vec::new();
    for i in 0..10 {
        let h = thread::spawn(move || {
            for j in 0..i + 10 {
                println!("{}: {}", i, j);
            }
        });
        v.push(h)
    }
    println!("Hello, world!");
    for h in v {
        h.join().unwrap_or_else(|e| {
            println! {"err: {:?}", e};
        })
    }

    println!("start open thread...");
    let state = State::new();
    open_threads(45, state.clone());
    loop {
        let n = state.counter.load(Ordering::SeqCst);
        if n == 0 {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }

    // loop {
    //     let handler;
    //     {
    //         let l = state.handlers.lock();
    //         let item = l.unwrap().pop();
    //         handler = match item {
    //             Some(i) => i,
    //             None => break,
    //         }
    //     }
    //     handler.join().unwrap();
    // }
}
