use nom::{
    IResult,
    bytes::complete::is_a,
};

// is_a - 
fn hex(s: &str) -> IResult<&str, &str> {
    is_a("1234567890ABCDEF")(s)
}

// todo - lifetime specifire add 
// fn parser2(s: &str, input_string: &str) -> IResult<&str, &str> {
//     your_parser(input_string)(s)
// }

 ///  AA XX // parsed = AA remaining =  XX
 /// AXXA XAA // Parsed = A      Remaining = XXA XAA
 /// XXA XAA Failed to parse
 /// DAXXA XAA    Parsed = DA      Remaining = XXA XAA
 /// xx aa xxa aax Failed to parse

  fn main() {
    let input = "xx aa xxa aax";
    match hex(input) { 
        Ok((remaining, parsed)) => {
            print!("{}    Parsed = {}      ", input, parsed);
            print!("Remaining = {}", remaining);
            println!();
        }
        Err(_) => {
            println!("{} Failed to parse", input);
        }
    };

assert_eq!(hex("123 and voila"), Ok((" and voila", "123")));
assert_eq!(hex("DEADBEEF and others"), Ok((" and others", "DEADBEEF")));
assert_eq!(hex("BADBABEsomething"), Ok(("something", "BADBABE")));
assert_eq!(hex("D15EA5E"), Ok(("", "D15EA5E")));
// assert_eq!(hex(""), Err(Err::Error(Error::new("", ErrorKind::IsA))));

}
