#[allow(unused_variables)]
#[allow(unused_assignments)]

//! Ownership in rust is a way to manage memory. 
//! It is a set of rules that the compiler checks at compile time.
//! One variable can own a piece of memory at a time.
//! When a variable goes out of scope, the memory is freed.
//! for primitive types, the copy is done automatically.
//! for non-primitive types, the copy is done manually. and ownership got transfered. 
//! onece a variable is moved, it cannot be used again. 
//! e.g. v is moved to w, print will show error v cannot be used again.

fn main() {
    let i = 5;
    let j = i;
    println!("{}", j);
    println!("{}", i);

    let v = vec![1, 2, 3, 4, 5];
    // let w = v;
    // println!("{:?}", w);
    // println!("{:?}", v);

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };
    let v = foo(v);
    println!("{:?}", v);
}