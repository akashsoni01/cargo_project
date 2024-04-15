mod hello;
mod strings;

fn main() {
    // println!("Hello, world!");
    hello::test_hello();
    println!("Reversed string = {:?}", strings::reverse("testing"));

}
