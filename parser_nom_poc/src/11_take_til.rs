use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    bytes::complete::{take_till, take_till1}
};


/// empty allowed wihle. atleast on take is mandatory in take_till1
/// |input=":Hello, world:some string"|Recognize=,|Parsed=""|Remaining=":Hello, world:some string"|
/// |input="Hello, world:some string"|Recognize=,|Parsed="Hello, world"|Remaining=":some string"|
fn till_colon(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == ':')(s)
  }

  /// |input=",Hello world:some string"|Recognize=,|Failed to parse|
  /// |input="Hello, world:some string"|Recognize=,|Parsed="Hello"|Remaining=", world:some string"|
  /// |input="Hello world:some string"|Recognize=,|Parsed="Hello world:some string"|Remaining=""|
  /// 
  fn till_comma(s: &str) -> IResult<&str, &str> {
    take_till1(|c| c == ',')(s)
  }



  fn main() {
    let input = "Hello, world:some string";
    let recognize_string:&str = ","; // 
    let recognize_char: char = ' ';
    match till_colon(input) { 
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
    assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
    assert_eq!(till_colon(":empty matched"), Ok((":empty matched", ""))); //allowed
    assert_eq!(till_colon("12345"), Ok(("", "12345")));
    assert_eq!(till_colon(""), Ok(("", "")));
            
}


/*

Sure! Imagine you have a big box of colorful building blocks, 
and you want to pick some of them.
But you have a special rule: you can only pick blocks until you find a block of a certain color. 
Let's say you're looking for a blue block.

take_till: With "take_till," you start picking blocks one by one. 
You keep picking blocks until you find a blue block. 
When you find the blue block, you stop picking. 
If there are no blue blocks, you keep picking until you've looked at all the blocks.

take_till1: "take_till1" is similar, but it has a rule that says you must pick at least one block before you stop. 

So, if you see a blue block right away, you pick it and stop. But if you don't see a blue block, 
you keep picking until you find one.

take_till_m_n: "take_till_m_n" is a bit different. It's like having two rules. 
One rule says you have to pick at least a certain number of blocks, like 2 blocks. 
The other rule says you stop when you find a blue block or when you've picked a certain number of blocks, like 5 blocks. 
So, you might pick 2, 3, 4, or 5 blocks, 
but you'll stop when you find a blue block or when you've picked 5 blocks.

These helpers help you pick blocks in a special way, 
following your rules about finding blue blocks. 
It's like playing a game with your blocks and looking for the blue ones!







take_till1: Similar to take_till, but it requires at least one byte to satisfy the condition.

use nom::{
    bytes::complete::take_till1,
    IResult,
};

fn main() {
    let input = b"Hello, World!";
    
    match parse_take_till1(input) {
        Ok((rest, result)) => {
            println!("Successfully parsed: {:?}", result);
            println!("Remaining input: {:?}", rest);
        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
        }
    }
}

fn parse_take_till1(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till1(|c| c == b',')(input)
}









take_till_m_n: This combinator allows you to specify a minimum (m) and maximum (n)
 number of bytes to take that satisfy the condition.
use nom::{
    bytes::complete::take_till_m_n,
    IResult,
};

fn main() {
    let input = b"Hello, World!";
    
    match parse_take_till_m_n(input) {
        Ok((rest, result)) => {
            println!("Successfully parsed: {:?}", result);
            println!("Remaining input: {:?}", rest);
        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
        }
    }
}

// not available 
fn parse_take_till_m_n(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till_m_n(2, 5, |c| c == b',')(input)
}

In this example, parse_take_till_m_n uses take_till_m_n to parse between 2 to 5 bytes until a comma (,). 
The parsed bytes can be "Hello" or "Hello,".
These are some related parsing functions that you can use with take_till in nom. 
They allow you to customize the parsing behavior based on your specific needs.




*/