/*
Write code that write code is called metaprogramming.
match a pattern and then execute some code is called pattern matching.
match a expression and then perform some action is called macro.
*/

#[allow(unused_variables)]
#[allow(unused_assignments)]

// below is a macro that prints a string
macro_rules! my_macro { // macro_rules! is the macro keyword
    () => (println!("First macro"))
}

// below is a macro that takes only one expression
// macro_rules! name {
//     ($name: expr) => { println!("Hey {}", $name)}
// }

// below is a macro that takes a list of expressions
macro_rules! name {
    ($($name: expr),*) => ( $(println!("Hey {}", $name);)* )
}

// below is a macro that takes an expression
macro_rules! xy {
    (x => $e: expr) => (println!("X is {}", $e));
    (y => $e: expr) => (println!("Y is {}", $e));
}

// below is a macro that builds a function
macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    }
}

fn main() {
    my_macro!(); // macro invocation
    name!("John"); // macro invocation
    name!("Alex", "Mary", "Carol"); // macro invocation
    xy!(x => 5); // macro invocation
    xy!(y => 3 * 9); // macro invocation
    build_fn!(hey); // macro invocation
    hey(); // macro invocation
}

// some other macros are: 
// println!() - prints to the console
// vec![] - creates a vector
// format!() - creates a string
// assert!() - checks if a condition is true
// dbg!() - prints the value of an expression for debugging purposes
// eprintln!() - prints to the console
// panic!() - causes the program to crash
// file!() - returns the file name
// line!() - returns the line number
// column!() - returns the column number
// stringify!() - converts an expression into a string
// env!() - returns the value of an environment variable
// include_str!() - reads a file and returns its contents as a string
// include_bytes!() - reads a file and returns its contents as a byte array
// concat!() - concatenates string literals
// concat_idents!() - concatenates identifiers
// format_args!() - creates an object that implements the fmt::Arguments trait
// std::mem::size_of_val() - returns the size of a value in bytes
// std::mem::size_of() - returns the size of a type in bytes
// std::mem::align_of() - returns the alignment of a type in bytes
// std::mem::align_of_val() - returns the alignment of a value in bytes
// std::mem::forget() - prevents a value from being dropped
// std::mem::replace() - replaces a value with another one
// std::mem::swap() - swaps two values
// std::mem::transmute() - converts a value from one type to another
// std::mem::transmute_copy() - converts a value from one type to another
// std::mem::uninitialized() - returns an uninitialized value
// std::mem::zeroed() - returns a zeroed value
// std::mem::drop() - drops a value
// std::mem::take() - takes a value
// std::mem::replace() - replaces a value with another one
// std::mem::swap() - swaps two values
// std::mem::size_of_val() - returns the size of a value in bytes
// std::mem::size_of() - returns the size of a type in bytes
// std::mem::align_of() - returns the alignment of a type in bytes
// std::mem::align_of_val() - returns the alignment of a value in bytes
// std::mem::forget() - prevents a value from being dropped
// std::mem::replace() - replaces a value with another one
// std::mem::swap() - swaps two values
// std::mem::transmute() - converts a value from one type to another
// std::mem::transmute_copy() - converts a value from one type to another
// std::mem::uninitialized() - returns an uninitialized value
// std::mem::zeroed() - returns a zeroed value
