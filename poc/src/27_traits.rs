#[allow(unused_variables)]
#[allow(unused_assignments)]

// Traits are similar to interfaces in Java. 
// They are used to define a set of methods that a type must implement.
// A type can implement a trait by implementing all of its methods.
// A type can implement multiple traits.
// Traits can be used to define default behavior for types.
// Traits can be used to define generic functions.
// Traits can be used to define generic types.

struct RustDev {
    awesome: bool
}


struct JavaDev {
    awesome: bool
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) { println!("Hello world!") }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello world!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello world!\");");
    }
}

fn test_traits() {
    // let r = RustDev { awesome: true};
    let r = RustDev::new(true);
    let j = JavaDev::new(false);
    println!("{}", r.language()); // output Rust
    r.say_hello(); // output println!("Hello world!");
    println!("{}", j.language()); // output Java 1.8
    j.say_hello(); // output System.out.println("Hello world!");
}

// a trait with optional / default methods

/*
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn print_details(&self) {
        println!("Area: {}, Perimeter: {}", self.area(), self.perimeter());
    }
}
*/
