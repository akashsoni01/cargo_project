use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    bytes::complete::take_while,
    // bytes::complete::take_while1,
    // bytes::complete::take_while_m_n,
    character::is_alphabetic, 
    character::is_digit
};

/// take_while Returns the longest list of bytes for which the provided function returns true. 
/// take_while1 does the same, but must return at least one character, 
/// while take_while_m_n must return between m and n
fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_alphabetic)(s)
  }
/// outputs - alpha
  /// |input=[49, 50, 51, 97, 98, 99, 52, 53, 54, 100, 101, 102]|Recognize=is_alphabetic|Parsed=[]|Remaining=[49, 50, 51, 97, 98, 99, 52, 53, 54, 100, 101, 102]|
  /// |input=[49, 97]|Recognize=is_alphabetic|Parsed=[]|Remaining=[49, 97]|
  /// |input=[49, 46, 97]|Recognize=is_alphabetic|Parsed=[]|Remaining=[49, 46, 97]|
  /// |input=[49, 46, 97, 126]|Recognize=is_alphabetic|Parsed=[]|Remaining=[49, 46, 97, 126]|
  /// |input=[]|Recognize=is_alphabetic|Parsed=[]|Remaining=[]|
  /// |input=[108, 97, 116, 105, 110]|Recognize=is_alphabetic|Parsed=[108, 97, 116, 105, 110]|Remaining=[]|
  /// 

fn parse_lowercase_letters(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c: u8| c.is_ascii_lowercase())(input)
}
/// outputs - parse_lowercase_letters
/// |input=[97, 65]|Recognize=is_ascii_lowercase|Parsed=[97]|Remaining=[65]|
/// |input=[104, 101, 108, 108, 111, 87, 111, 114, 108, 100, 49, 50, 51]|Recognize=is_ascii_lowercase|Parsed=[104, 101, 108, 108, 111]|Remaining=[87, 111, 114, 108, 100, 49, 50, 51]|
/// 

fn parse_uppercase_word(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|c| c.is_ascii_uppercase())(input)
}
/// outputs - parse_uppercase_word
/// 


/*
take_while_m_n: This combinator allows you to specify 
a minimum (m) and maximum (n) number of bytes to take that satisfy the predicate. 
Here's an example of using take_while_m_n to parse a sequence of exactly 3 lowercase letters:
*/
fn parse_lowercase_triplet(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while_m_n(3, 3, |c| c.is_ascii_lowercase())(input)
}
/// outputs - parse_lowercase_triplet
/// In this example, parse_lowercase_triplet uses take_while_m_n 
/// to parse exactly 3 consecutive lowercase letters. 
/// The parsed lowercase triplet is "abc",
///  and the remaining input is "XYZ".





  fn main() {
    let input = b"helloWorld123";
    let recognize_string:&str = "HELLO World"; // 
    let recognize_char: char = ' ';
    match parse_uppercase_word(input) { 
        Ok((remaining, parsed)) => {
            println!();
            if !recognize_char.is_whitespace() {
                print!("|input={:?}|Recognize Char={}|Parsed={:?}|Remaining={:?}|", input, recognize_char, parsed,remaining);
            } else if recognize_string.is_empty() {
                print!("|input={:?}|Parsed={:?}|Remaining={:?}|", input, parsed,remaining);
            } else {
                print!("|input={:?}|Recognize={}|Parsed={:?}|Remaining={:?}|", input, recognize_string, parsed,remaining);
            }
            println!();
            println!();
        }
        Err(_) => {
            println!();
            if !recognize_char.is_whitespace() {
                print!("|input={:?}|Recognize Char={}|Failed to parse|", input, recognize_char);
            } else if recognize_string.is_empty() {
                print!("|input={:?}|Failed to parse|", input);
            } else {
                print!("|input={:?}|Recognize={}|Failed to parse|", input, recognize_string);
            }

            println!();
            println!();
        }
    };
}


#[test]
fn test_parser() {
    assert_eq!(alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
    assert_eq!(alpha(b"12345"), Ok((&b"12345"[..], &b""[..])));
    assert_eq!(alpha(b"latin"), Ok((&b""[..], &b"latin"[..])));
    assert_eq!(alpha(b""), Ok((&b""[..], &b""[..])));
        
}

/*
Parse lowercase ASCII letters:
use nom::{
    bytes::complete::take_while,
    IResult,
};

fn parse_lowercase_letters(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c.is_ascii_lowercase())(input)
}




Parse uppercase ASCII letters:
use nom::{
    bytes::complete::take_while,
    IResult,
};

fn parse_uppercase_letters(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c.is_ascii_uppercase())(input)
}




Parse alphanumeric characters (letters and digits):
use nom::{
    bytes::complete::take_while,
    IResult,
};

fn parse_alphanumeric(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c.is_ascii_alphanumeric())(input)
}




Parse hexadecimal digits:
use nom::{
    bytes::complete::take_while,
    IResult,
};

fn parse_hex_digits(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c.is_ascii_hexdigit())(input)
}



Parse whitespace characters:
use nom::{
    bytes::complete::take_while,
    IResult,
};

fn parse_whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c.is_ascii_whitespace())(input)
}



Parse custom conditions:
You can also define your custom closure to parse bytes based on specific conditions.
 For example, you can parse a sequence of bytes that match a particular pattern or follow certain rules.


The key is to provide the appropriate closure that returns true for bytes 
you want to include in the result and false for bytes you want to skip. 
The take_while combinator will continue taking bytes as long as the closure returns true.






*/

/*
We import the necessary items from nom, including bytes::complete::take_while.
We define a parse_digits function that uses take_while with a closure. 
The closure checks whether each byte in the input slice is an ASCII digit.
In the main function, we provide an input byte slice b"123abc456def".
We use the parse_digits function to parse a sequence of bytes that represent digits.
We pattern match on the result:
If parsing is successful (Ok variant), we print the parsed byte slice (the digits) 
and the remaining input byte slice.
If there's a parsing error (Err variant), we print the error.
In this example, take_while(|c: u8| c.is_ascii_digit()) 
parses all consecutive ASCII digit characters from the input. The parsed digits are b"123", 
and the remaining input is b"abc456def". 
You can customize the closure to match your specific parsing needs.
*/