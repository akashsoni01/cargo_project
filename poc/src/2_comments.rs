use std::io;

fn comments(){
// how to document functions ? 
// Single line comments 
/*Mulit line comments */
/// doc comment will be automatically generated when we run command 
/// cargo doc

    let mut input = String::new();
    println!("How to document code ? see below comment.");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }

}