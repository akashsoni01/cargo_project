use nom::{
    IResult,
    bytes::complete::is_not, error::{Error, ErrorKind},
    Err,
};

// is_not -The parser will return the longest slice till one of the characters of the combinator’s argument are met.
fn not_space(s: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(s)
  }
  
 ///   Failed to parse // empty string case 
 ///   Failed to parse // one space case 
 /// hello world    Parsed=hello      Remaining= world
 /// Hello, World    Parsed=Hello,      Remaining= World
 /// |Nospace|Parsed=Nospace|Remaining=|
 /// |input=Sometimes       |Parsed=Sometimes|Remaining=       |

  fn main() {
    let input = "Sometimes       ";
    match not_space(input) { 
        Ok((remaining, parsed)) => {
            println!();
            print!("|input={}|Parsed={}|Remaining={}|", input, parsed,remaining);
            println!();
            println!();
        }
        Err(_) => {
            println!("{} Failed to parse", input);
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


// todo - lifetime specifire add 
// fn parser2(s: &str, input_string: &str) -> IResult<&str, &str> {
//     your_parser(input_string)(s)
// }



/*

nom::character::complete::is_not is a combinator used to parse 
and return a slice of characters that do not match any of the characters
in a given string. Here's an example of how to use is_not:




use nom::{
    character::complete::is_not,
    IResult,
};

fn main() {
    let input = "Hello, World!";

    match parse_not_punctuation(input) {
        Ok((rest, result)) => {
            println!("Successfully parsed: '{}'", result);
            println!("Remaining input: '{}'", rest);
        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
        }
    }
}

fn parse_not_punctuation(input: &str) -> IResult<&str, &str> {
    is_not(".,!?")(input)
}


In this example:

We import the necessary items from nom, including is_not.
We define a parse_not_punctuation function that uses is_not(".,!?") to parse a slice of characters that do not include any of the characters in the provided string ".,!?".
In the main function, we provide an input string "Hello, World!" and use the parse_not_punctuation function to parse characters that are not punctuation marks.
We pattern match on the result:
If parsing is successful (Ok variant), we print the parsed slice of characters and the remaining input.
If there's a parsing error (Err variant), we print the error.
In this example, is_not(".,!?") parses the characters "Hello" and " World" (excluding the punctuation marks), and the remaining input is "!". You can use is_not to parse slices of characters that do not contain specific characters or character sets.

*/