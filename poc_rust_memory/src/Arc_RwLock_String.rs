use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let shared_string = Arc::new(RwLock::new(String::from("Hello"))); // Arc is a thread-safe reference-counting pointer

    let mut handles = vec![]; // A vector to store the JoinHandle of each thread

    // Spawn threads for writing
    for i in 0..5 {
        let shared_string = Arc::clone(&shared_string); // Clone the Arc to move it into the thread

        let handle = thread::spawn(move || { // Spawn a thread
            let mut string = shared_string.write().unwrap(); // Acquire a write lock
            string.push_str(&format!(" from thread {}", i + 1)); // Modify the string
            println!("Thread {}: {}", i + 1, string);
        });

        handles.push(handle); // Store the JoinHandle of the thread
    }

    // Wait for all threads to finish
    for handle in handles { // Join each thread
        handle.join().unwrap(); // JoinHandle::join() returns a thread::Result
    }

    // Read the final string value safely
    println!("Final string value: {}", *shared_string.read().unwrap()); // Acquire a read lock
}


/*
Thread 1: Hello from thread 1
Thread 2: Hello from thread 1 from thread 2
Thread 3: Hello from thread 1 from thread 2 from thread 3
Thread 4: Hello from thread 1 from thread 2 from thread 3 from thread 4
Thread 5: Hello from thread 1 from thread 2 from thread 3 from thread 4 from thread 5
Final string value: Hello from thread 1 from thread 2 from thread 3 from thread 4 from thread 5
*/