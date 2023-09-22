use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
   character::complete::one_of
};

// Recognizes one of the provided characters.
fn parse_vowel(input: &str) -> IResult<&str, char> {
    one_of("aeiou")(input)
}
  
 ///   Failed to parse // empty string case 
 ///   Failed to parse // one space case 
 ///  |input=apple|Recognize=aeiou|Parsed=a|Remaining=pple|
 /// |input=jk|Failed to parse||
 /// |input=sky|Failed to parse||
 /// |input=akash akash|Recognize=aeiou|Parsed=a|Remaining=kash akash|
 /// |input=aeiou|Recognize=aeiou|Parsed=a|Remaining=eiou|

  fn main() {
    let input = "aeiou";
    let recognize_string:&str = "aeiou";
    match parse_vowel(input) { 
        Ok((remaining, parsed)) => {
            println!();
            if recognize_string.is_empty() {
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
    assert_eq!(one_of::<_, _, (&str, ErrorKind)>("abc")("b"), Ok(("", 'b')));
    assert_eq!(one_of::<_, _, (&str, ErrorKind)>("a")("bc"), Err(Err::Error(("bc", ErrorKind::OneOf))));
    assert_eq!(one_of::<_, _, (&str, ErrorKind)>("a")(""), Err(Err::Error(("", ErrorKind::OneOf))));
    
}


// todo - lifetime specifire add 
// fn parser2(s: &str, input_string: &str) -> IResult<&str, &str> {
//     your_parser(input_string)(s)
// }
