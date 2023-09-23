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
use nom::{
    IResult,
    sequence::tuple,
    character::complete::char,
    multi::fold_many0,
};

fn main() {
    let input = "Hello, World!";
    let tag_to_parse = "hello";

    match parse_tag_no_case(input, tag_to_parse) {
        Ok((rest, result)) => {
            println!("Successfully parsed: {:?}", result);
            println!("Remaining input: {:?}", rest);
        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
        }
    }
}

fn parse_tag_no_case(input: &str, tag_to_parse: &str) -> IResult<&str, &str> {
    let tag_len = tag_to_parse.len();
    let mut parser = tuple((
        fold_many0(char(tag_to_parse.chars().next().unwrap()), 0, |count, c| count + 1),
        fold_many0(char(tag_to_parse.chars().next().unwrap().to_ascii_uppercase()), 0, |count, c| count + 1),
        char(tag_to_parse.chars().next().unwrap()),
        char(tag_to_parse.chars().next().unwrap().to_ascii_uppercase()),
        tag(&tag_to_parse[1..])
    ));

    let (input, (_, _, first_char, first_char_upper, rest)) = parser(input)?;

    if first_char == first_char_upper {
        Ok((input, &input[..tag_len]))
    } else {
        Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)))
    }
}


In this example:

We define a parse_tag_no_case function that takes an input string and a tag to parse case-insensitively.
The function constructs a parser by defining a tuple of parsers that consider both lowercase and uppercase variants of the first character of the tag.
We use fold_many0 to allow multiple occurrences of the same character, ensuring it's case-insensitive.
If the first character of the parsed tag (after case folding) matches the first character of the tag to parse, we return a successful result. Otherwise, we return a parsing error.
With this custom parse_tag_no_case function, you can parse tags without being sensitive to case differences.


*/




