use std::ops::AddAssign;

// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arch;

// add from libraries 
use rand::Rng; // import single
// use rand::{thread_rng, Rng}; // import multiple
// use rand::{}; // import all
// extern crate rand; // import all

// Don't forget to add the following line to the top of main.rs:
mod archive;

#[allow(unused_variables)]
fn main() {
    println!("Multiple modules are grouped in crates");
    println!("Crates have two types 1. Binary 2. Library");

    // arch_file("Some file");
    // arch("Some file");
    arch("Some file");


    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut rng = rand::thread_rng();
    
    let random_number: i32 = rng.gen();
    println!("Random number: {}, secrate numer {}", random_number, secret_number);

    println!("Random double: {}", rng.gen::<f64>());
    println!("Random bool: {}", rng.gen::<bool>());
    println!("Random char: {}", rng.gen::<char>());
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());

    println!("Random string: {}", generate_random_string());

}


// fn generate_random_string(size: usize) -> String {
//     let mut rng = rand::thread_rng();
//     let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
//     (0..size).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
// }

// fn generate_random_string2(size: usize) -> String {
//     let mut rng = rand::thread_rng();
//     let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
//     let mut result = String::new();
//     for _ in 0..size {
//         result.push(chars[rng.gen_range(0..chars.len())]);
//     }
//     result
// }


// #[allow(unused_variables)]
// fn generate_random_string() -> String {
//     let mut rng = rand::thread_rng();
//     let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
//     let mut result = String::new();
//     for _ in 0..10 {
//         let random_char = chars[rng.gen_range(0..chars.len())];
//         result.push(random_char);
//     }
//     result
// }