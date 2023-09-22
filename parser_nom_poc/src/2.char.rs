use nom::{
    IResult,
    character::complete::char,
};

fn parser(i: &str) -> IResult<&str, char> {
    char('a')(i)
}

fn parser2(i: &str, input_char: char) -> IResult<&str, char> {
    char(input_char)(i)
}

fn main() {
    let input = "abcde";
    match parser2(input, 'e') { // 'a' error 'b', '', 'd', 'e'
        Ok((remaining, parsed)) => {
            println!("parsed string was = {}", parsed);
            println!("remaining string is = {}", remaining);
        }
        Err(_) => {
            println!("Failed to parse {}", input);
        }
    };

// other test case 
assert_eq!(parser("abc"), Ok(("bc", 'a')));
assert_eq!(parser(" abc"), Err(Err::Error(Error::new(" abc", ErrorKind::Char))));
assert_eq!(parser("bc"), Err(Err::Error(Error::new("bc", ErrorKind::Char))));
assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Char))));

}
