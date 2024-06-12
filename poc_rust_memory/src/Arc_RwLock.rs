use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let counter = Arc::new(RwLock::new(0)); // Arc is a thread-safe reference-counting pointer

    let mut handles = vec![]; // A vector to store the JoinHandle of each thread

    // Spawn threads for writing
    for i in 0..5 {
        let counter = Arc::clone(&counter); // Clone the Arc to move it into the thread

        let handle = thread::spawn(move || { // Spawn a thread
            let mut num = counter.write().unwrap(); // Acquire a write lock
            *num += 1; // Dereference the RwLockGuard to modify the data
            println!("Thread {}: Counter: {}", i + 1, *num);
        });

        handles.push(handle); // Store the JoinHandle of the thread
    }

    // Wait for all threads to finish
    for handle in handles { // Join each thread
        handle.join().unwrap(); // JoinHandle::join() returns a thread::Result
    }

    // Read the counter value safely
    println!("Final counter value: {}", *counter.read().unwrap()); // Acquire a read lock
}


/*
Thread 1: Counter: 1
Thread 2: Counter: 2
Thread 3: Counter: 3
Thread 4: Counter: 4
Thread 5: Counter: 5
Final counter value: 5

*/