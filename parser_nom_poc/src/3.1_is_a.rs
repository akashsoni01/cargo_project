use nom::{
    IResult,
    error::{Error, ErrorKind},
    Err,
    bytes::complete::is_a,
};

fn hex(s: &str) -> IResult<&str, &str> {
    is_a("1234567890अआइईउऊऋएऐओऔ")(s)
}
/// |input=1234567890अआइईउऊऋएऐओऔ|Recognize=1234567890अआइईउऊऋएऐओऔ|Parsed=1234567890अआइईउऊऋएऐओऔ|Remaining=|
/// |input=12345|Recognize=1234567890अआइईउऊऋएऐओऔ|Parsed=12345|Remaining=|
/// |input=अआइईउऊऋ|Recognize=1234567890अआइईउऊऋएऐओऔ|Parsed=अआइईउऊऋ|Remaining=|
/// |input=अआइ ईउऊऋ|Recognize=1234567890अआइईउऊऋएऐओऔ|Parsed=अआइ|Remaining= ईउऊऋ|
/// |input=1234 ईउऊऋ|Recognize=1234567890अआइईउऊऋएऐओऔ|Parsed=1234|Remaining= ईउऊऋ|
/// |input=5661234 ईउऊऋ|Recognize=1234567890अआइईउऊऋएऐओऔ|Parsed=5661234|Remaining= ईउऊऋ|
/// |input=ग 5661234 ईउऊऋ|Failed to parse||
/// 
  fn main() {
    let input = "ग 5661234 ईउऊऋ";
    let recognize_string:&str = "1234567890अआइईउऊऋएऐओऔ"; // 
    let recognize_char: char = ' ';
    match hex(input) { 
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
    assert_eq!(hex("123 and voila"), Ok((" and voila", "123")));
    assert_eq!(hex("DEADBEEF and others"), Ok((" and others", "DEADBEEF")));
    assert_eq!(hex("BADBABEsomething"), Ok(("something", "BADBABE")));
    assert_eq!(hex("D15EA5E"), Ok(("", "D15EA5E")));
    assert_eq!(hex(""), Err(Err::Error(Error::new("", ErrorKind::IsA))));
        
}
