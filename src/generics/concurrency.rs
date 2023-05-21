
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::{Builder, JoinHandle};
use actix::prelude::*;
use tokio::reactor::Handle;
use tokio::timer::Delay;
use std::time::Duration;
use std::time::Instant;
use futures::future::Future;
use futures::future;

pub fn first_thread() {
    let child = thread::spawn(|| {
        println!("Thread");
        String::from("much concurrent, such wow")
    });

    println!("Hello");
    let value = child.join().expect("failed to join");

    println!("{}", value)
}


pub fn using_thread_builder() {
    let my_thread = Builder::new().
        name("Worker thread".to_string()).
        stack_size(1024 * 1024 * 1);

    let handle = my_thread.spawn(|| {
        panic!("oops")
    });

    let child_status = handle.unwrap().join();

    println!("child status {:?}", child_status);
}

pub fn thread_communication() {
    let nums = Arc::new(vec![0, 1, 2, 3, 4]);

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for n in 0..5 {
        let nums = Arc::clone(&nums);
        let handle = thread::spawn(move || {
            println!("{}", nums[n])
        });

        handles.push(handle)
    }

    for hand in handles {
        hand.join().unwrap();
    }
}


pub fn thread_mutate() {
    let mut nums = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for n in 0..5 {
        let mut ns = nums.clone();
        let child = thread::spawn(move || {
            let mut v = ns.lock().unwrap();
            v.push(n);
        });

        childs.push(child)
    }

    for ch in childs {
        ch.join().unwrap();
    }

    println!("{:?}", nums)
}


pub fn read_write_lock() {
    let m = RwLock::new(5);
    let c = thread::spawn(move || {
        {
            *m.write().unwrap() += 1;
        }

        let updated = *m.read().unwrap();
        updated
    });

    let updated = c.join().unwrap();
    println!("{}", updated)
}

pub fn async_channel(){
    let (tx,rx) = channel();
    let join_handle = thread::spawn(move ||{
        while let Ok(n)=rx.recv(){
            println!("received{}",n);
        }

    });

    for i in 0..10{
        tx.send(i).unwrap()
    }

    drop(tx); // this is important else rx channel stuck

    join_handle.join().unwrap();
}

struct Add(u32,u32);

impl Message for Add{
    type Result = Result<u32,()>;
}

struct Addr;

impl Actor for Addr{
    type Context = SyncContext<Self>;
}

impl Handler<Add> for Addr{
    type Result = Result<u32,()>;

    fn handle(&mut self, msg: Add, ctx: &mut Self::Context) -> Self::Result {
        let sum =msg.0 +msg.0;
        println!("computed: {} + {} = {}",msg.0,msg.1,sum);
        Ok(msg.0 +msg.1)
    }
}

pub fn actors() -> i32 {
    System::run(||{
        let addr = SyncArbiter::start(3,||Addr);
        for n in 5..10{
            addr.do_send(Add(n,n+1));
        }
        tokio::spawn(futures::lazy(|| {
            Delay::new(Instant::now() + Duration::from_secs(1)).then(|_| {
                System::current().stop();
                future::ok::<(),()>(())
            })
        }));

    })


}