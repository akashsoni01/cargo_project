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