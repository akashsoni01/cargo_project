fn print_test() {
    println!("Hello, world!");
    println!("My name is {} and I'm {} years old", "Alex", 29);
    println!("a + b = {}", 3 + 9);
    println!("{0} has a {2} and {0} has a {1}", "Alex", "cat", "dog");
    println!("{name} {surname}", surname="Smith", name="Alex");
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50); // b 
    println!("array: {:?}", [1, 2, 3]); // ? denotes to debug description string version of object 

}