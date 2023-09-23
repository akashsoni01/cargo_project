use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    bytes::complete::tag_no_case
};

fn parser(s: &str) -> IResult<&str, &str> {
    tag_no_case("Hello")(s)
  }
  
  /// |input=hello, world!|Recognize=Hello|Parsed=hello|Remaining=, world!|

  fn main() {
    let input = "hello, world!";
    let recognize_string:&str = "Hello"; // 
    let recognize_char: char = ' ';
    match parser(input) { 
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
            if !recognize_char.is_whitespace() {
                print!("|input={}|Recognize Char={}|Failed to parse|", input, recognize_char);
            } else if recognize_string.is_empty() {
                print!("|input={}|Failed to parse|", input);
            } else {
                print!("|input={}|Recognize={}|Failed to parse|", input, recognize_string);
            }

            println!();
            println!();
        }
    };
}


#[test]
fn test_parser() {
    assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
    assert_eq!(parser("Something"), Err(Err::Error(Error::new("Something", ErrorKind::Tag))));
    assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));

}


/*
  fn parse_tag(input: &[u8], tag_to_parse: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(tag_to_parse)(input)
}

fn main() {
    let input = b"Hello, World!";
    let tag_to_parse = b"Hello";

    match parse_tag(input, tag_to_parse) {
        Ok((rest, result)) => {
            println!("Successfully parsed: {:?}", result);
            println!("Remaining input: {:?}", rest);
        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
        }
    }
}


In this example, tag(b"Hello")
parses the byte slice "Hello" from the input b"Hello, World!", 
and the remaining input is b", World!".
You can use tag to search for and parse specific byte sequences in binary data.




*/



