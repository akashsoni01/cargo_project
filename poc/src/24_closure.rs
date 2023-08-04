// closures are anonymus / lambda expression that can be assigned to a variable or passed as an argument to a function 
// closure in rust is a function that can capture the enclosing environment
// closure can be stored in a variable or passed as an argument to another function
// closure can capture values from the scope in which they are defined

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // here we don't need to specify the return 
    let a = |a: i32| a+1; // |a: i32| -> a+1;
    println!("{}", a(6)); // output: 7

    // b is a lambda function that take a i32 i.e variable b and return a i32 i.e variable c
    let b = |b: i32| {
        let c = b + 1;
        c
    };
    println!("{}", b(4));

    // a clouser can be generic. i.e. all the other call must be same data type 
    /*
    un commenting both won't work because the compiler can't infer the type of the variable
    gen(3);
    gen(true);
    */
    let gen = |x| println!("{}", x);
    gen(3);
    // gen(true);
}