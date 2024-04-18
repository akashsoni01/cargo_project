use std::sync::{Arc, Mutex};
use std::thread;

pub fn test() {
    let counter = Arc::new(Mutex::new((0)));

    // clone the arc for each thread
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    // spqwn two threads

    let thread1 = thread::spawn(move || {
        for _ in 0..5 {
            // lock the mutex to access the shared data
            let mut data = counter1.lock().unwrap();
            *data += 1;
            println!("thread 1 {}", *data);

        }
    });

    let thread2 = thread::spawn(move || {
        for _ in 0..5 {
            // lock the mutex to access the shared data
            let mut data = counter2.lock().unwrap();
            *data += 1;
            println!("thread 1 {}", *data);

        }
    });


    // wait for both thread to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // let x = counter.lock().unwrap();
    // let y = *counter.lock().unwrap();
    // final value of shared state
    println!("Final value of shared state = {}", *counter.lock().unwrap());

}