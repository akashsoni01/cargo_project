use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    character::complete::none_of,
};

// opposit of one of.
fn parse_not_punctuation(input: &str) -> IResult<&str, char> {
    none_of(".,!?")(input)
}

/// |input=Hello, world!|Recognize=.,!?||Parsed=H|Remaining=ello, world!|
/// |input=,Hello, world!|Failed to parse||
/// 

  fn main() {
    let input = ",Hello, world!";
    let recognize_string:&str = ".,!?|"; // 
    let recognize_char: char = ' ';
    match parse_not_punctuation(input) { 
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
    assert_eq!(none_of::<_, _, (&str, ErrorKind)>("abc")("z"), Ok(("", 'z')));
    assert_eq!(none_of::<_, _, (&str, ErrorKind)>("ab")("a"), Err(Err::Error(("a", ErrorKind::NoneOf))));
    assert_eq!(none_of::<_, _, (&str, ErrorKind)>("a")(""), Err(Err::Error(("", ErrorKind::NoneOf))));
    }