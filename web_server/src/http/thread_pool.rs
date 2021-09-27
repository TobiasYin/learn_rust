use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        atomic::{AtomicUsize},
        {Mutex, Arc}
    },
    collections::{
        hash_set::HashSet,
        hash_map::HashMap,
        vec_deque::VecDeque,
    },
    sync::atomic::Ordering,
    thread::{
        self,
        sleep
    },
    time::Duration,
    error::Error
};
use crate::http::Res;

pub trait Ret: Send + 'static {}

impl<T> Ret for T
    where T: Send + 'static {}

pub trait Bound<R: Ret>: FnOnce() -> R + Send + 'static {}

impl<T, R> Bound<R> for T
    where T: FnOnce() -> R + Send + 'static, R: Ret {}

pub enum Signal<T: Bound<R>, R: Ret> {
    Job(T),
    Return(R),
    Quit,
}

pub struct Chan<T: Bound<R>, R: Ret> {
    recv: Receiver<Signal<T, R>>,
    send: Sender<Signal<T, R>>,
}

impl<T: Bound<R>, R: Ret> Chan<T, R> {
    fn new() -> (Chan<T, R>, Chan<T, R>) {
        let chan1 = channel();
        let chan2 = channel();
        (Chan { recv: chan1.1, send: chan2.0 }, Chan { recv: chan2.1, send: chan1.0 })
    }
}

struct Ptr<T: Bound<R>, R: Ret>(*mut Pool<T, R>);

unsafe impl<T: Bound<R>, R: Ret> Send for Ptr<T, R> {}

#[allow(dead_code)]
pub struct Pool<T: Bound<R>, R: Ret> {
    pool_size: usize,
    max_pool_size: usize,
    // TODO
    now_size: AtomicUsize,
    top_id: AtomicUsize,
    channels: HashMap<usize, Chan<T, R>>,
    running: HashSet<usize>,
    waits: VecDeque<usize>,
    mutex: Option<Arc<Mutex<Ptr<T, R>>>>,
    has_wait: Mutex<bool>,
    send: Sender<T>
}

impl<T: Bound<R>, R: Ret> Pool<T, R> {
    pub fn new(size: usize) -> Self {
        let (send, recv) = channel();
        let mut s = Pool {
            pool_size: size,
            max_pool_size: size * 3,
            now_size: Default::default(),
            top_id: Default::default(),
            channels: Default::default(),
            running: Default::default(),
            waits: Default::default(),
            mutex: None,
            has_wait: Mutex::new(false),
            send
        };
        s.mutex = Some(Arc::new(Mutex::new(Ptr(&mut s as *mut Self))));

        let this = s.mutex.as_ref().unwrap().clone();

        thread::spawn(move ||{
            for job in recv.iter(){
                Self::borrow_ref_from_arc(&this, |s|{
                    s.add_job_inner(job);
                })
            }
        });

        for i in 0..size {
            s.new_thread(i);
            s.waits.push_back(i)
        }
        s.top_id.store(size, Ordering::SeqCst);
        s
    }

    fn borrow_ref_from_arc<F: FnOnce(&mut Pool<T, R>)>(this: &Arc<Mutex<Ptr<T, R>>>, f: F) {
        unsafe {
            let pool = this.lock().unwrap();
            f(pool.0.as_mut().unwrap());
        }
    }

    #[allow(unused_must_use)]
    fn add_job_inner(&mut self, job: T) -> usize {
        let id = self.poll_thread();
        println!("new job, thread: {}", id);
        self.running.insert(id);

        let chan = &self.channels[&id];
        chan.recv.try_recv();
        chan.send.send(Signal::Job(job)).unwrap();

        println!("thread num {}", self.now_size.load(Ordering::Relaxed));

        id
    }

    pub fn add_job(&mut self, job: T) -> Res<()>{
        self.send.send(job)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn wait_res(&mut self, id: usize) -> Result<R, Box<dyn Error>> {
        if !self.running.contains(&id) {
            Err("not found".into())
        } else {
            let recv = self.channels[&id].recv.recv()?;
            match recv {
                Signal::Return(r) => Ok(r),
                _ => Err("no recv".into())
            }
        }
    }

    fn return_thread(&mut self, id: usize) {
        self.running.remove(&id);
        if self.now_size.load(Ordering::SeqCst) > self.pool_size {
            if !*self.has_wait.lock().unwrap() {
                self.kill_thread(id);
                return;
            }
        }
        self.waits.push_back(id);
    }

    fn kill_thread(&mut self, id: usize) {
        println!("kill: {}", id);
        let c = &self.channels[&id];
        c.send.send(Signal::Quit).unwrap();
        self.now_size.fetch_sub(1, Ordering::SeqCst);
        self.channels.remove(&id);
    }

    fn new_thread(&mut self, id: usize) {
        self.now_size.fetch_add(1, Ordering::SeqCst);
        let (chan1, chan2) = Chan::new();
        self.channels.insert(id, chan1);
        let refs = self.mutex.as_ref().unwrap().clone();
        thread::spawn(move || {
            let chan = chan2;
            loop {
                let sig = match chan.recv.recv() {
                    Ok(s) => s,
                    Err(_) => break
                };
                let job = match sig {
                    Signal::Job(j) => j,
                    _ => break,
                };
                let res = job();

                Self::borrow_ref_from_arc(&refs, |this| {
                    this.return_thread(id);
                });

                if let Err(_) = chan.send.send(Signal::Return(res)) {
                    break;
                }
            }
        });
    }

    fn poll_thread(&mut self) -> usize {
        loop {
            let res = match self.waits.pop_back() {
                Some(v) => v,
                None => {
                    if self.now_size.load(Ordering::SeqCst) > self.max_pool_size {
                        *self.has_wait.lock().unwrap() = true;
                        sleep(Duration::from_millis(5));
                        continue;
                    }
                    let id = self.top_id.fetch_add(1, Ordering::SeqCst);
                    self.new_thread(id);
                    id
                }
            };
            *self.has_wait.lock().unwrap() = false;
            return res
        }
    }
}

impl<T: Bound<R>, R: Ret> Drop for Pool<T, R> {
    fn drop(&mut self) {
        let mut kill_list = vec![];
        kill_list.extend(self.waits.iter());
        kill_list.extend(self.running.iter());
        kill_list.iter().for_each(|i| { self.kill_thread(*i) })
    }
}