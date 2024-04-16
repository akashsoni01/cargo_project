pub fn test_hello() {
    println!("hello world");
}

pub struct Solution;
impl Solution {
    pub fn test() {
        println!("hello world");
    }

    pub fn test_non_static(&self) {
        println!("non static function / hello world");
    }
}