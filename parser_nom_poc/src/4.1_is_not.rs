use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    bytes::complete::is_not,
};

// is_not -The parser will return the longest slice till one of the characters of the combinator’s argument are met.
fn not_space(s: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(s)
  }
/// |input=नमस्ते संसार ||Recognize=\t\r\n|Parsed=नमस्ते|Remaining= संसार ||
/// |input=सा नमस्ते संसार ||Recognize=\t\r\n|Parsed=सा|Remaining= नमस्ते संसार ||
/// 
/// 
/// 
  
  fn main() {
    let input = "नमस्ते संसार |";
    let recognize_string:&str = "\\t\\r\\n"; // 
    let recognize_char: char = ' ';
    match not_space(input) { 
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


#[test]
fn test_parser() {
    assert_eq!(not_space("Hello, World!"), Ok((" World!", "Hello,")));
    assert_eq!(not_space("Sometimes\t"), Ok(("\t", "Sometimes")));
    assert_eq!(not_space("Nospace"), Ok(("", "Nospace")));
    assert_eq!(not_space(""), Err(Err::Error(Error::new("", ErrorKind::IsNot))));
}
