/*
Sharing data between threads:
Rust enforces strict ownership and borrowing rules to prevent data races.
To share data between threads, you need to use thread-safe containers 
like Arc (Atomic Reference Counting) or synchronization primitives like Mutex (Mutual Exclusion).
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Arc is a thread-safe reference-counting pointer

    let mut handles = vec![]; // A vector to store the JoinHandle of each thread

    for _ in 0..5 {
        let counter = Arc::clone(&counter); // Clone the Arc to move it into the thread

        let handle = thread::spawn(move || { // Spawn a thread
            // Lock the mutex to modify the shared data
            let mut num = counter.lock().unwrap(); // MutexGuard
            *num += 1; // Dereference the MutexGuard to modify the data
            println!("Thread {}: Counter: {}", thread::current().name().unwrap(), *num);
        });

        handles.push(handle); // Store the JoinHandle of the thread
    }

    // Wait for all threads to finish
    for handle in handles { // Join each thread
        handle.join().unwrap(); // JoinHandle::join() returns a thread::Result
    }

    // The counter will be modified by multiple threads safely
    println!("Final counter value: {}", *counter.lock().unwrap()); // MutexGuard
}


/*
outupt 
Thread 1: Counter: 1
Thread 2: Counter: 2
Thread 3: Counter: 3
Thread 4: Counter: 4
Thread 5: Counter: 5

Final counter value: 5

*/