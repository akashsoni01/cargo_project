use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    bytes::complete::take
};

fn take6(s: &str) -> IResult<&str, &str> {
    take(6usize)(s)
  }
    
    /// |input=hello, world!|Parsed=hello,|Remaining= world!|
    /// |input=0123456|Parsed=012345|Remaining=6|
    /// |input=1223|Failed to parse|
    /// |input=abcdefg|Parsed=abcdef|Remaining=g|
    /// |input=ABCDEFG|Parsed=ABCDEF|Remaining=G|
    /// |input=abc|Failed to parse|

  fn main() {
    let input = "abc";
    let recognize_string:&str = ""; // 
    let recognize_char: char = ' ';
    match take6(input) { 
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
    assert_eq!(take6("1234567"), Ok(("7", "123456")));
    assert_eq!(take6("things"), Ok(("", "things")));
    assert_eq!(take6("short"), Err(Err::Error(Error::new("short", ErrorKind::Eof))));
    assert_eq!(take6(""), Err(Err::Error(Error::new("", ErrorKind::Eof))));
    
}


/*
The nom::bytes::complete::take function in Rust's nom library is used to parse and return a specified number 
of bytes from the input data. Here's an example of how to use take to parse a specific number of bytes 
from a byte array:

use nom::{
    bytes::complete::take,
    IResult,
};

fn main() {
    let input = b"Hello, World!";
    let num_bytes_to_take = 5; // Change this to the number of bytes you want to take.

    match parse_take(input, num_bytes_to_take) {
        Ok((rest, result)) => {
            println!("Successfully parsed: {:?}", result);
            println!("Remaining input: {:?}", rest);
        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
        }
    }
}

fn parse_take(input: &[u8], num_bytes: usize) -> IResult<&[u8], &[u8]> {
    take(num_bytes)(input)
}


In this example:

We import the necessary items from nom, including bytes::complete::take.
We define a parse_take function that uses take(num_bytes) to parse the specified number of bytes (num_bytes) from the input byte slice (input).
In the main function, we provide an input byte slice b"Hello, World!" and specify the number of bytes we want to take (num_bytes_to_take).
We use the parse_take function to parse the specified number of bytes from the input.
We pattern match on the result:
If parsing is successful (Ok variant), we print the parsed byte slice and the remaining input byte slice.
If there's a parsing error (Err variant), we print the error.
In this example, take(5) parses the first 5 bytes from the input b"Hello, World!", and the remaining input is b", World!". You can use take to parse a specific number of bytes from binary data.

*/



