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
            println!("thread 2 {}", *data);

        }
    });

    // wait for both thread to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    let x = counter.lock().unwrap();
    let y = *counter.lock().unwrap();
    // final value of shared state
    println!("Final value of shared state = {}", *counter.lock().unwrap());

}

/*
fn main() {
    alien::test();
}
*/

/*
output 

thread 1 1
thread 1 2
thread 1 3
thread 1 4
thread 1 5
thread 2 6
thread 2 7
thread 2 8
thread 2 9
thread 2 10
Final value of shared state = 10
*/ 


/*
or it can also print whatever thread finishes first.

thread 2 1
thread 2 2
thread 2 3
thread 2 4
thread 2 5
thread 1 6
thread 1 7
thread 1 8
thread 1 9
thread 1 10
Final value of shared state = 10

*/