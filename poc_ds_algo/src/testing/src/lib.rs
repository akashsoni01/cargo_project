mod test;

use std::fmt::Error;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*pub fn problem1_test1() -> Result<(), Error>{
    let mut mutex_variable = Mutex::new(89);
    // let c1 = mutex_variable.cloned // problem the mutex can't be cloned
    // solution - use
    let y = mutex_variable.lock(); // LockResult<MutexGuard<i32>>
    let y = mutex_variable.lock().unwrap(); // MutexGuard<i32>
    let thread = thread::spawn( move || {
        let modification = mutex_variable.lock().unwrap();
        // let the_type = modification; // MutexGuard<i32>
        // let the_type = *modification; // i32
        *modification = 8;
    });

    let thread2 = thread::spawn( move || {
        let modification = mutex_variable.lock().unwrap();
        // let the_type = modification; // MutexGuard<i32>
        // let the_type = *modification; // i32
        *modification = 8;
    });

    thread.join().unwrap();
    thread2.join().unwrap();
    return Ok(());
}*/
/*
pub fn problem2_test1() -> Result<(), Error>{
    let counter = Rc::new(Mutex::new((0)));

    // clone the arc for each thread
    let counter1 = Rc::clone(&counter);
    let counter2 = Rc::clone(&counter);

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

    // let x = counter.lock().unwrap();
    // let y = *counter.lock().unwrap();
    // final value of shared state
    println!("Final value of shared state = {}", *counter.lock().unwrap());
    return Ok(());
}
*/
pub fn test3()  -> Result<(), Error>{
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

    // let x = counter.lock().unwrap();
    // let y = *counter.lock().unwrap();
    // final value of shared state
    println!("Final value of shared state = {}", *counter.lock().unwrap());

    return  Ok(());
}

fn memory_mgmt() {
    let x = 3; // copy, stack

    let string = String::from(""); // heap,


}
#[cfg(test)]
mod tests {
    use std::result;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_wont_works() {
        let result = test3();
    }
}
