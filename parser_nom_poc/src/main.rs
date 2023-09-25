use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    bytes::complete::{take_until, take_until1}
};


fn until_eof(s: &str) -> IResult<&str, &str> {
    take_until("eof")(s) // matching a tag while till match a condition
  }
  
/// |input="hello, world"|Recognize=eof|Failed to parse|
/// 
fn until_eof1(s: &str) -> IResult<&str, &str> {
    take_until("eof")(s)
  }

/// |input="eofHello, world:some string"|Recognize=eof|Parsed=""|Remaining="eofHello, world:some string"|
/// |input="hello, eofworld"|Recognize=eof|Parsed="hello, "|Remaining="eofworld"|
  fn main() {
    let input = "hello, eofworld";
    let recognize_string:&str = "eof"; // 
    let recognize_char: char = ' ';
    match until_eof1(input) { 
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
    assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
    assert_eq!(until_eof("hello, world"), Err(Err::Error(Error::new("hello, world", ErrorKind::TakeUntil))));
    assert_eq!(until_eof(""), Err(Err::Error(Error::new("", ErrorKind::TakeUntil))));
    assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
                
}



/*
In the context of the `nom` library in Rust, both `take_until` and `take_till` are used to parse a sequence of bytes until a specified condition is met. However, there is a subtle difference between them:

1. **`take_until`**: This combinator parses bytes until it encounters a specific byte sequence (a tag). It stops parsing when it finds the specified tag and includes the tag in the result.

   For example, if you use `take_until("abc")`, it will parse until it finds the first occurrence of "abc" and include "abc" in the result.

   ```rust
   use nom::{
       bytes::complete::take_until,
       IResult,
   };

   fn main() {
       let input = b"Hello, abc World!";
       
       match parse_take_until(input) {
           Ok((rest, result)) => {
               println!("Successfully parsed: {:?}", result);
               println!("Remaining input: {:?}", rest);
           }
           Err(err) => {
               println!("Parsing error: {:?}", err);
           }
       }
   }

   fn parse_take_until(input: &[u8]) -> IResult<&[u8], &[u8]> {
       take_until("abc")(input)
   }
   ```

   In this example, `parse_take_until` uses `take_until("abc")` to parse until it finds "abc," and "abc" is included in the result.

2. **`take_till`**: This combinator parses bytes until a specified condition is met, based on a provided closure. It stops parsing when the closure's condition is `true` and does not include the byte that satisfies the condition in the result.

   For example, if you use `take_till(|c| c == b',')`, it will parse until it finds a comma (`,`), but the comma won't be included in the result.

   ```rust
   use nom::{
       bytes::complete::take_till,
       IResult,
   };

   fn main() {
       let input = b"Hello, World!";
       
       match parse_take_till(input) {
           Ok((rest, result)) => {
               println!("Successfully parsed: {:?}", result);
               println!("Remaining input: {:?}", rest);
           }
           Err(err) => {
               println!("Parsing error: {:?}", err);
           }
       }
   }

   fn parse_take_till(input: &[u8]) -> IResult<&[u8], &[u8]> {
       take_till(|c| c == b',')(input)
   }
   ```

   In this example, `parse_take_till` uses `take_till(|c| c == b',')` to parse until it finds a comma (`,`), but the comma is not included in the result.

So, the main difference is in what they consider as the stopping point: `take_until` stops at a specific tag and includes it, while `take_till` stops based on a condition you define and does not include the character that satisfies the condition.
*/