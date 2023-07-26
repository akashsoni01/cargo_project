use std::io;

fn main() {
    //! # Main Heading - Main
    
    //! fn user_input
    //! the function will take one string from input stream and print it with you said ...
    //! else it is going to show error
    //! 

    //! ```
    //! match io::stdin().read_line(&mut input) {
    //!     Ok(_) => {
    //!         println!("You said {}", input);
    //!     },
    //!     Err(e) => {
    //!         println!("Something went wrong {}", e);
    //!     }
    //! }
    //! ```
    //! 
    //! this is a comment function example that you can use

    let mut input = String::new();
    println!("Say someting");
    print!("yo yo honey sing");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }
}
