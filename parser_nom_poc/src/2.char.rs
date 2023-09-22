use nom::{
    IResult,
    character::complete::char,
};

fn parser(i: &str, input_char: char) -> IResult<&str, char> {
    char(input_char)(i)
}

fn main() {
    let input = "abcde";
    match parser(input, 'e') { // 'a' error 'b', '', 'd', 'e'
        Ok((remaining, parsed)) => {
            println!("parsed string was = {}", parsed);
            println!("remaining string is = {}", remaining);
        }
        Err(_) => {
            println!("Failed to parse {}", input);
        }
    };
}
