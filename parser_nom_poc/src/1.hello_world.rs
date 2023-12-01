/*
you can define a parser that recognizes the exact string "Hello, World!"
and returns a success or failure indication.
Here's how you can do it:
*/
use nom::{
    IResult,
    bytes::complete::tag,
};

/// tag :- hello_world_parser is a parser function that uses
/// the tag combinator to match the string "Hello, World!" exactly.
fn hello_world_parser(input: &str) -> IResult<&str, &str> {
    tag("Hello, World!")(input)
}

/// नमस्ते संसार 
fn main() {
    let input = "Hello, World! testing"; // Input string

    match hello_world_parser(input) {
        Ok((remaining, parsed)) => {
            println!("Parsed 'Hello, World!'");
            println!("parsed string was = {}", parsed);
            println!("remaining string is = {}", remaining);
        }
        Err(_) => {
            println!("Failed to parse 'Hello, World!'");
        }
    }
}

/*
tag:

The tag combinator expects an input string to match
exactly with the provided string slice.
If there is a match, it returns a successful result containing 
the matched string slice; otherwise,
it returns an error.


use nom::{IResult, bytes::complete::tag};

fn main() {
    let input = "Hello, World!";

    let result = tag("Hello")(input);
    match result {
        Ok((remaining, matched)) => {
            println!("Matched: {:?}", matched);
            println!("Remaining: {:?}", remaining);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}




Matched: "Hello"
Remaining: ", World!"

*/

/*
tag_no_case

The tag_no_case combinator is similar to tag,
but it performs a case-insensitive comparison. 
It allows you to match the provided string regardless of letter case.
use nom::{IResult, bytes::complete::tag_no_case};

fn main() {
    let input = "Hello, World!";

    let result = tag_no_case("hello")(input);
    match result {
        Ok((remaining, matched)) => {
            println!("Matched: {:?}", matched);
            println!("Remaining: {:?}", remaining);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}



Matched: "Hello"
Remaining: ", World!"


*/


/*
tag_s

The tag_s combinator is similar to tag, 
but it works with byte slices (i.e., [u8]) 
instead of strings. This can be 
useful when working with binary data.

use nom::{IResult, bytes::complete::tag_s};

fn main() {
    let input: &[u8] = b"Hello, World!";

    let result = tag_s(b"Hello")(input);
    match result {
        Ok((remaining, matched)) => {
            println!("Matched: {:?}", matched);
            println!("Remaining: {:?}", remaining);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}


Matched: [72, 101, 108, 108, 111]
Remaining: [44, 32, 87, 111, 114, 108, 100, 33]




In this example, tag_s(b"Hello") successfully matches 
the byte slice [72, 101, 108, 108, 111] 
in the input byte slice 
[72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33].
*/

/*
These tag variants are commonly used in parsers 
to recognize specific sequences of bytes or characters 
in the input data. The choice of which one to use 
depends on the requirements of your parsing 
task and whether you need case sensitivity or 
are working with byte slices.
*/
