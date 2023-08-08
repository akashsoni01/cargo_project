#[allow(unused_variables)]
#[allow(unused_assignments)]

// variable can be borrowed immutably multiple times
// variable can be borrowed mutably only once
// variable can be borrowed immutably and mutably at the same time
// variable can bowrrow othre memory locations
// variable can be borrowed in a function and returned
// variable can be borrowed in a function and not returned
// variable can be borrowed in a function and returned with a lifetime. e.g. 
// fn foo<'a>(v: &'a Vec<i32>) -> &'a Vec<i32> { v }
// we can't borrow a variable mutable if we have defined it immutable before.



fn main() {
    let mut a = 6;
    {
        let b = &mut a;
        println!("{}", *b);
        *b += 2;

        /*
        *a += 2;
        error[E0614]: type `{integer}` cannot be dereferenced
        --> src/main.rs:21:9
        *a += 2;
        */

    }
    println!("{}", a);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
        v.push(6);
    }
}