// mod hello;
// mod strings;
// mod arrays;
//
// use hello::Solution;


use std::collections::{HashMap, HashSet};
use std::ptr::hash;

mod alien;
mod vector;
mod strings;
mod hello;
mod arrays;

fn f(s: &mut String) {
    *s = String::from("other");
    println!("{}", s);
}

fn f2(s: &String) {
    println!("{}", s);
}

fn f3(s: String) {
    println!("{}", s);
}

fn main() {
    // let mut s1 = String::from("hello world"); //
    // f(&mut s1);
    // println!("reversed string = {}", strings::reverse_string(&mut "hello".to_string()));
    // println!("reversed string = {}", strings::reverse("hello"));

}


/*
fn test<'a>(a: u8, b: String) {

}

fn test() -> 'static &str {

}
*/