fn f(s: &mut String) {
    *s = String::from("other");
    println!("{}", s);
}

fn main() {
    let mut s1 = String::from("hello world"); //
    f(&mut s1);
}
