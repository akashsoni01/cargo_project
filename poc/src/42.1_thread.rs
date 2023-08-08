use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[allow(unused_variables)]
#[allow(unused_assignments)]

// Threads - Run code in parallel with threads
// ownership and borrowing give us mamory safety, thread safety and no data race conditions
// Rust threads are similar to OS threads
// Rust threads are "green" threads
// Rust threads are lightweight
// Rust threads are preemptive
// Rust threads are not tied to OS threads
// Rust threads are scoped
// Rust threads are joinable
// Rust threads are message passing
// Rust threads are not shared memory
// Rust threads are not thread safe


// how to create a thread,
// and how to wait for it to finish,
// and how to pass data to it,
// and how to get data back from it,
// and how to handle errors in threads
// https://doc.rust-lang.org/book/ch16-01-threads.html
// https://doc.rust-lang.org/std/thread/
fn main() {
    let mut threads = vec![];
    for i in 0..10 {
        let th = thread::spawn(move || { // move keyword is used to transfer ownership of the variable i to the thread
            sleep(Duration::from_millis(i * 1000)); // sleep for i seconds
            println!("new thread {}", i); // print the thread number
        });
        threads.push(th); // push the thread to the vector
    }

    for t in threads { // join each thread
        t.join(); // join() returns a thread::Result (wait for the thread to finish)
    }

    println!("Main thread");
}

/*
new thread 0
new thread 1
new thread 2
new thread 3
new thread 4
new thread 5
new thread 6
new thread 7
new thread 8
new thread 9
Main thread
*/