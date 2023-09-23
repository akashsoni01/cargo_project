use nom::{
    IResult,
    character::complete::char,
};

fn parser(i: &str) -> IResult<&str, char> {
    char('अ')(i)
}

fn parser2(i: &str, input_char: char) -> IResult<&str, char> {
    char(input_char)(i)
}

fn main() {
    let input = "अ,आ,इ,ई,उ,ऊ,ऋ,ए,ऐ,ओ,औ";
    let recognize_string:&str = "इ";
    let recognize_char: char = 'अ';
    match parser2(input, recognize_char) { 
        Ok((remaining, parsed)) => {
            println!();
            if !recognize_char.is_whitespace() {
                print!("|input={}|Recognize Char={}|Parsed={}|Remaining={}|", input, recognize_char, parsed,remaining);
            } else if recognize_string.is_empty() {
                print!("|input={}|Parsed={}|Remaining={}|", input, parsed,remaining);
            } else {
                print!("|input={}|Recognize={}|Parsed={}|Remaining={}|", input, recognize_string, parsed,remaining);
            }
            println!();
            println!();
        }
        Err(_) => {
            println!();
            print!("|input={}|Failed to parse||", input);
            println!();
            println!();
        }
    };
}
