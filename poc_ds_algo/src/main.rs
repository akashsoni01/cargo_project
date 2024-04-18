// mod hello;
// mod strings;
// mod arrays;
//
// use hello::Solution;


mod alien;

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

    alien::test();
}


/*
fn test<'a>(a: u8, b: String) {

}

fn test() -> 'static &str {

}
*/