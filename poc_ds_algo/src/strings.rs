use std::iter::Rev;
use std::str::Chars;

pub fn reverse(string: &str) -> String {
    return string.chars().rev().collect::<String>();
}