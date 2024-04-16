mod hello;
mod strings;
mod arrays;

use hello::Solution;


fn main() {
    // println!("Hello, world!");
    hello::test_hello();
    hello::Solution::test();
    hello::Solution::from(Solution{}).test_non_static();
    let solution = Solution{};
    solution.test_non_static();
    println!("Reversed string = {:?}", strings::reverse("testing"));

}
