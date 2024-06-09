use std::iter::Rev;
use std::str::Chars;


/*
Problem 1
*/
pub fn reverse(string
               : &str) -> String {
    return string.chars().rev().collect::<String>();
}

pub fn reverse_string(s: &mut String) -> String {
    s.chars().rev().collect()
}
