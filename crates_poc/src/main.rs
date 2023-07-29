// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arch;

// Don't forget to add the following line to the top of main.rs:
mod archive;

fn main() {
    println!("Multiple modules are grouped in crates");
    println!("Crates have two types 1. Binary 2. Library");

    // arch_file("Some file");

    // arch("Some file");

    arch("Some file");

}
