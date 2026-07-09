use std::thread;
use std::{sync::{Arc, Mutex}};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut thread_handlers = vec![];
    const MAX_COUNT: u32 = 5;

    for _ in 1..=MAX_COUNT {
       let counter_clone = Arc::clone(&counter);
       let thread = thread::spawn(move || {
           for i in 1..=1000 {
                let mut counter = counter_clone.lock().unwrap();
                *counter += 1;
                println!("Thread {:?} incremented counter to: {}", thread::current().id(), *counter);
            }

        });
       thread_handlers.push(thread);
    }
    
    for thread in thread_handlers {
        thread.join().unwrap();
    }
    println!("Final counter value: {}", counter.lock().unwrap());
}