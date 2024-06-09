use std::error::Error;
use std::fs::File;


/*
usecase of ? operator to unwrap from result
*/
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello,txt")?; // File
    let f = File::open("hello,txt"); // Result<File>
    Ok(())
}