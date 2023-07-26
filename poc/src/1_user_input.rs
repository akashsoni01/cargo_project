use std::io;

fn user_input() {
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
