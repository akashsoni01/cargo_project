use std::error::Error;
use std::io;
use std::fs::File;
use std::io::Read;

#[allow(unused_variables)]
#[allow(unused_assignments)]

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("src/username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// below code is same as above code but it is more concise
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("src/username.txt")?;
    let mut s = String::new();
    // here ? operator is used to return the error value from the current function for the caller to handle
    // if the value is Ok, the value inside the Ok will get returned from this expression, and the program will continue.
    // else if the value is Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
    f.read_to_string(&mut s)?; // ? operator can be used in functions that have a return type of Result
    // if the value is Ok, the value inside the Ok will get returned from this expression, and the program will continue.
    Ok(s)
}

fn main() {
    let a = read_username_from_file();
    println!("{:?}", a);
}