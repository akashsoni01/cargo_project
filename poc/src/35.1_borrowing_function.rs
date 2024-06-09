
fn main() {
    let mut s1 = String::from("hello world"); //
    f(&mut s1);

    println!("{}", s1); // the output will be "other" but 
    // if we would call f3 we can't use s1 anymore.
    // because the ownership of the variable s1 has been moved to the function f3. 
    // and after f3 is called, the variable s1 is dropped.

    /*
    f3(s1);
    println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
    */ 
}

// the function f borrows the variable s1 mutably. 
fn f(s: &mut String) {
    *s = String::from("other"); // by dereferencing the variable s, we can change the value of the variable s1
    println!("{}", s);
}


// the function f1 borrows the variable s1 immutably.
// we can borrow the variable s1 immutably multiple times.
fn f2(s: &String) {
    println!("{}", s);
}

fn f3(s: String) {
    println!("{}", s);
}