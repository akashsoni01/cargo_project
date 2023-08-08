use std::thread;

/*
In Rust, threads allow you to execute multiple tasks concurrently,
 taking advantage of multiple CPU cores to improve performance. 
 Rust provides built-in support for working with threads through its standard library. 
 You can create threads, spawn them, 
 and communicate between them using various synchronization primitives.
*/

fn main() {
    // Creating a new thread and spawning it
    // the output of the spawned thread is not guaranteed to be printed before the output of the main thread
    let handle = thread::spawn(|| {
        // The code inside this block will be executed in a separate thread
        for i in 1..=5 {
            println!("Hello from thread! Count: {}", i);
        }
    });

    // Wait for the thread to finish executing (optional)
    handle.join().unwrap();

    // Continue with the main thread
    println!("Main thread continues after the spawned thread!");
}
