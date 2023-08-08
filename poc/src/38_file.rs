use std::fs::{File, OpenOptions, remove_file};
use std::io::{Write, Read};

#[allow(unused_variables)]
#[allow(unused_assignments)]

// create, read, delete and append content in a file 

fn main() {
    // create file
    // let mut file = File::create("src/example.txt").expect("create failed");
    // file.write_all("Hello World!\n".as_bytes()).expect("write failed");

    // append content to the file
    // let mut file = OpenOptions::new().append(true)
    //     .open("src/example.txt")
    //     .expect("cannot open file");
    // file.write_all("Adding content to the file.\n".as_bytes()).expect("write failed");

    // read file
    let mut file = File::open("src/example.txt").unwrap();
    let mut contents = String::new();

    // read_to_string is a method of the Read trait
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

    // deleate file
    remove_file("src/example.txt").expect("delete failed");


}