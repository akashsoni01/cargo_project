use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    character::complete::one_of
};

// Recognizes one of the provided characters.
fn parse_vowel(input: &str) -> IResult<&str, char> {
    one_of("अआइईउऊऋएऐओऔ")(input)
}

/*
Hindi words that contain vowels:

आलसी (Aalsi) - Lazy
इंसान (Insaan) - Human
उदाहरण (Udaharan) - Example
ईमानदार (Imaandaar) - Honest
उपहार (Uphaar) - Gift
आशिक (Aashiq) - Lover
आदमी (Aadmi) - Man (as in human)
ऊंचाई (Oonchaai) - Height
अध्ययन (Adhyayan) - Study
एकता (Ekta) - Unity

*/

/// |input=आकाश सोनी|Recognize=अआइईउऊऋएऐओऔ|Parsed=आ|Remaining=काश सोनी|
/// |input=अमीर|Recognize=अआइईउऊऋएऐओऔ|Parsed=अ|Remaining=मीर|
/// |input=उपहार|Recognize=अआइईउऊऋएऐओऔ|Parsed=उ|Remaining=पहार|
/// |input=एकता|Recognize=अआइईउऊऋएऐओऔ|Parsed=ए|Remaining=कता|
  
  fn main() {
    let input = "एकता";
    let recognize_string:&str = "अआइईउऊऋएऐओऔ"; // 
    let recognize_char: char = ' ';
    match parse_vowel(input) { 
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
    assert_eq!(one_of::<_, _, (&str, ErrorKind)>("abc")("b"), Ok(("", 'b')));
    assert_eq!(one_of::<_, _, (&str, ErrorKind)>("a")("bc"), Err(Err::Error(("bc", ErrorKind::OneOf))));
    assert_eq!(one_of::<_, _, (&str, ErrorKind)>("a")(""), Err(Err::Error(("", ErrorKind::OneOf))));
}
