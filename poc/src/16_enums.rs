use crate::Colors::Red;
use crate::Person::Name;

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]

fn test_nums() {
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    let my_color = Red;
    println!("{:?}", my_color);

    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32)
}

impl Person {
    // associated function
    fn new(name: String, surname: String, age: u32) -> Person {
        Person::Name(name)
    }

    // method
    fn get_name(&self) -> String {
        match self {
            Person::Name(name) => name.to_string(),
            _ => String::from("No name")
        }
    }

    // method
    fn get_surname(&self) -> String {
        match self {
            Person::Surname(surname) => surname.to_string(),
            _ => String::from("No surname")
        }
    }

    // static method
    fn static_fn_detail() -> String {
        String::from("This is a static method")
    }

    // curry method
    fn curry_fn_detail(&self) -> String {
        String::from("This is a curry method")
    }

    // other curry for variadic arguments
    fn curry_fn_detail2(&self) -> String {
        String::from("This is a curry method")
    }

}


// create a enum language with 3 variants, and associated value as string

#[derive(Debug)]
enum Language {
    Rust(String),
    Python(String),
    Java(String)
}

// create a method for Language enum
impl Language {
    fn get_language(&self) -> String {
        match self {
            Language::Rust(lang) => lang.to_string(),
            Language::Python(lang) => lang.to_string(),
            Language::Java(lang) => lang.to_string()
        }
    }
}

// create a enum with 3 variants, and raw value as string 
#[derive(Debug)]
enum Language2 {
    Rust = "Rust".len() as isize,
    Python = "Python".len() as isize,
    Java = "Java".len() as isize
}

// create a method for Language2 enum
impl Language2 {
    fn get_language(&self) -> String {
        match self {
            Language2::Rust => String::from("Rust"),
            Language2::Python => String::from("Python"),
            Language2::Java => String::from("Java")
        }
    }
}


// create a enum with 3 variants, and raw value as integer
#[derive(Debug)]
enum Language3 {
    Rust = 1,
    Python = 2,
    Java = 3
}


// Language4
#[derive(Debug)]
enum Language4 {
    Rust,
    Python,
    Java
}

impl ToString for Language4 {
    fn to_string(&self) -> String {
        match self {
            Language4::Rust => String::from("Rust 1.48"),
            Language4::Python => String::from("Python 3.8"),
            Language4::Java => String::from("Java 11")
        }
    }
}

// use language4 enum
fn test_language4() {
    let lang = Language4::Rust;
    println!("{}", lang.to_string());
}

