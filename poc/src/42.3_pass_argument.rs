// Using thread arguments
// You can pass arguments to the thread closure using move semantics to transfer ownership.
use std::thread;

fn main() {
    let name = "Alice".to_string();

    // Create a thread with an argument
    let handle = thread::spawn(move || {
        println!("Hello, {}!", name);
    });

    handle.join().unwrap();
}
