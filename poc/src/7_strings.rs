#[allow(unused_variables)]
#[allow(unused_assignments)]

fn string_test() {
    let cat: &'static str = "Fluffy"; // &str is a string slice
    println!("{}", cat); // Fluffy

    let dog = String::new(); // String::new() is a static method. new instance of String
    let mut dog = String::from("Max"); // String::from() is a static method.
    // that takes a string slice and returns a String
    println!("{}", dog); // Max

    let owner = format!("Hi I'm {} the owner of {}", "Mark", dog); // format! is a macro
    println!("{}", owner); // Hi I'm Mark the owner of Max
    println!("{}", dog.len()); // 3
    println!("{}", dog.is_empty()); // false


    dog.push(' '); // push() takes a single character

    dog.push_str("the dog"); // push_str() takes a string slice and returns nothing

    println!("{}", dog); // Max the dog



    let new_dog = dog.replace("the", "is my"); // replace() takes a string slice and replace it with another string slice
    // if any string repeats then it automatically replaces all the occurrences
    
    println!("{}", new_dog); // Max is my dog

    // multi line string example 

    let multi_line = "This is a \
    multi line string"
    ;

    println!("{}", multi_line);
}


// function that take a string and return a string
fn take_and_give_back(a_string: String) -> String {
    a_string
}

// function that take a string and return a string and length
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// function takes a string slice and returns a string slice and length
fn calculate_length_ref(s: &String) -> (&String, usize) {
    let length = s.len();
    (s, length)
}

// below function takes a string slice and returns a string slice. 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // as_bytes() returns a byte slice
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // returns a string slice
        }
    }
    &s[..]
}


// multi line string 
fn multi_line_string() {
    let multi_line = "This is a \
    multi line string"
    ;

    println!("{}", multi_line);
}

// multi line string with new line r#""#

fn multi_line_string_with_new_line() {
    let multi_line = r#"This is a 
    multi line string"#
    ;

    println!("{}", multi_line);
}