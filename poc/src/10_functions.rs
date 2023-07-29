#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn function_test() {
    // for i in 1..6 {
    //     say_hi();
    // }

    let mut name = "John";
    let greeting = say_hello(&mut name);
    println!("{}", greeting);
}

// fn say_hi() {
//     println!("Hello there");
// }
fn say_hello2(some_string: &str) {
    println!("{}", some_string);
}


fn say_hello(name: &mut &str) -> String { // pass by reference 
    let greeting = format!("Hello {}", name);
    greeting
}