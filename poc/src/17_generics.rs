use std::fmt::Display;
use crate::Colors::Red;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn test_generics() {
    let p1: Point<i32> = Point {X: 6, Y: 8}; // create a Point struct with i32 type
    let p2: Point<f64> = Point {X: 3.25, Y: 8.63}; // create a Point struct with f64 type
    println!("{:?}", p1); // print the struct
    println!("{:?}", p2); // print the struct

    let c1 = Red("#f00"); // create a Colors enum with String type
    let c2 = Red(255); // create a Colors enum with i32 type
    println!("{:?}", c1); // print the enum
    println!("{:?}", c2); // print the enum

    let p3: Point2<i32, f64> = Point2 {x: 34, y: 8.5}; // create a Point2 struct with i32 and f64 type
    println!("{:?}", p3); // print the struct

     // array of generic type T
    // let mut array: [T; size] = [value; size];
    // let mut array: [T; size] = [default_value; size];
}

#[derive(Debug)]
// In Rust, generics refer to the parameterization of datatypes and traits.

struct Point<T> { // generic struct
    X: T,
    Y: T
}

#[derive(Debug)]
enum Colors<T> { // generic enum, below enum have three variants with same type
    // enum can have multiple variants
    // each variant can have different types
    // each variant can have same type
    // each variant can have generic type
    Red(T),
    Blue(T),
    Green(T)
}

#[derive(Debug)]
struct Point2<T, V> { // generic struct with multiple types, below struct have two fields with different types
    x: T,
    y: V
}

// impl block is used to add methods to a struct
// in below code we are adding 2 methods to Point struct
impl<T> Point<T> { // generic impl block
    // new is an associated function that takes 2 parameters and returns a Point
    fn new(x: T, y: T) -> Point<T> {
        Point {X: x, Y: y}
    }

    // fn_y is a method that takes a reference to a Point and returns a reference to a T
    fn fn_y(&self) -> &T {
        &self.Y
    }

    // fn_x_mut is a method that takes a mutable reference to a Point and returns a mutable reference to a T
    fn fn_x_mut(&mut self) -> &mut T {
        &mut self.X
    }
        
    // Define a function that takes two arguments and returns their sum
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Currying implementation for the add function
    fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }

    // currying implementaiton for the add function with generic type
    fn curry_add_generic<T: std::ops::Add<Output = T> + Copy>(x: T) -> impl Fn(T) -> T {
        move |y| x + y
    }

    // how to use or call curry_add_generic in main function
    // let add_five = curry_add_generic(5); // Partially apply add function with the first argument as 5
    // println!("{}", add_five(3)); // Output: 8
    // println!("{}", add_five(10)); // Output: 15


    // fn main() {
    //     let add_five = curry_add(5); // Partially apply add function with the first argument as 5
    //     println!("{}", add_five(3)); // Output: 8
    //    println!("{}", add_five(10)); // Output: 15
    // }

    // currying implementation for the some function with two generic type T and U
    fn curry_some<T, U>(x: T) -> impl Fn(U) -> (T, U) {
        move |y| (x, y)
    }

    // example of currying implementation for the some function with two generic type T and U
    // let some = curry_some(5); // Partially apply some function with the first argument as 5
    // println!("{:?}", some(3)); // Output: (5, 3)
    // println!("{:?}", some(10)); // Output: (5, 10)
    

    // fn_x is a method that takes a reference to a Point and returns a reference to a T
    fn fn_x(&self) -> &T {
        &self.X
    }

    // curry is a method that takes a reference to a Point and returns a reference to a function
    fn curry(&self) -> &dyn Fn(T) -> Point<T> {
        &|y| Point {X: self.X, Y: y}
    }

    // how to use or call it in main function
    // let p = Point::new(3, 4);
    // let p2 = p.curry()(5); 
    // println!("{:?}", p); // output Point { X: 3, Y: 4 }
    // println!("{:?}", p2); // output Point { X: 3, Y: 5 }

    // curry for two arguments
    fn curry2(&self) -> &dyn Fn(T, T) -> Point<T> {
        &|x, y| Point {X: x, Y: y}
    }

    // how to use or call it in main function 
    // let p = Point::new(3, 4);
    // let p2 = p.curry()(5);
    // let p3 = p.curry2()(5, 6);
    // println!("{:?}", p); // output Point { X: 3, Y: 4 }
    // println!("{:?}", p2); // output Point { X: 3, Y: 5 }
    // println!("{:?}", p3); // output Point { X: 5, Y: 6 }

    // curry for three arguments
    fn curry3(&self) -> &dyn Fn(T, T, T) -> Point<T> {
        &|x, y, z| Point {X: x, Y: y}
    }

    // how to use or call it in main function
    // let p = Point::new(3, 4);
    // let p2 = p.curry()(5);
    // let p3 = p.curry2()(5, 6);
    // let p4 = p.curry3()(5, 6, 7);
    // println!("{:?}", p); // output Point { X: 3, Y: 4 }
    // println!("{:?}", p2); // output Point { X: 3, Y: 5 }
    // println!("{:?}", p3); // output Point { X: 5, Y: 6 }
    // println!("{:?}", p4); // output Point { X: 5, Y: 6 }


    // currey method for variadic arguments
    fn curry2(&self) -> &dyn Fn(T, T) -> Point<T> {
        &|x, y| Point {X: x, Y: y}
    }
    // how to use or call it in main function
    // let p = Point::new(3, 4);
    // let p2 = p.curry()(5);
    // let p3 = p.curry2()(5, 6);
    // println!("{:?}", p); // output 
    // println!("{:?}", p2);
    // println!("{:?}", p3);

}

trait YourGenericTrait {
    fn print_it<T: Display>(&self, first: T);
    fn print_it<T: Display>(&self);
    fn print_it<T: Display>(&self) -> T;
}