#[allow(unused_variables)]
#[allow(unused_assignments)]

fn test_struct() {
    //! # Main Heading - Struct 
    //! here we are initializing a struct with name, company and age
    let emp = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35
    };

    println!("{:?}", emp); // print the struct
    println!("{}", emp.name); // print the name
    println!("{}", emp.fn_details()); // call the method
    println!("{}", Employee::static_fn_detail()); // call the static method
}

// derive directive is used to add some basic implementations to our struct
// Debug is one of the traits that we get for free
// We can now print the struct using {:?} or {:#?} for pretty print
// We can also use the struct in println!() without using the .name
#[derive(Debug)]
// structs are similar to classes in other languages
// structs are value types
// In rust we can have multiple structs with same name in different modules
// structs are used to create custom data types
// structs can have methods
// structs can have associated functions
// structs can have tuple structs
// structs can have unit structs
// structs can have generic structs
// structs can have generic methods
// structs can have generic impl blocks
// structs can have generic enum
// structs can have generic enum methods
// structs can have generic enum impl blocks
// structs can have generic trait
// structs can have generic trait methods
// structs can have generic trait impl blocks
// structs can have generic trait enum
// structs can have generic trait enum methods
// structs can have generic trait enum impl blocks

// Employ is a struct with 3 fields name, company and age
// name is of type String
// company is of type String
// age is of type u32
struct Employee {
    name: String,
    company: String,
    age: u32
}

// impl block is used to add methods to a struct
// in below code we are adding 2 methods to Employee struct
impl Employee {
    
    // new is an associated function that takes 3 parameters and returns an Employee
    fn new(name: String, company: String, age: u32) -> Employee {
        Employee {
            name,
            company,
            age
        }
    }

    // fn_details is a method that takes self as a parameter and returns a String
    fn fn_details(&self) -> String {
        format!("name: {}, age: {}, company: {} ", &self.name, &self.age, &self.company)
    }

    // static_fn_detail is an associated function that takes no parameter and returns a String
    fn static_fn_detail() -> String {
        String::from("Details of a person") // return String 
    }
}
