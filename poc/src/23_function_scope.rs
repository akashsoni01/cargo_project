// No need to deallocate variable, it will be done automatically when it goes out of scope
// Global variable can be declared outside of any function but they are unsafe
// to access global variable we need to use unsafe block
// define global variable with static keyword e.g. static mut GLOBAL_VAR: i32 = 10;


#[allow(unused_variables)]
#[allow(unused_assignments)]

static mut R: i32 = 0;

fn main() {
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a);

    unsafe {
        R = 4;
        println!("R = {}", R);
    }

    unsafe {
        println!("R = {}", R);
    }
}