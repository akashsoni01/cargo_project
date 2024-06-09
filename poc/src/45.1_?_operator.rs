use std::fmt::Error;

fn main() -> Result<(), Error> {
    let optional = convert_to_option(12);
    let optional_unwraped = convert_to_option(12)?;
    let optional_unwraped_2 = convert_to_option(12).unwrap();
    let optional_unwraped_3 = convert_to_option(12).unwrap_or(0);


    let result = convert_to_result(12);
    let result_unwrapped = convert_to_result(12)?;
    let result_unwrapped_2 = convert_to_result(12).unwrap();
    let result_unwrapped = convert_to_result(12).unwrap_or(0);


    return Ok(())
}

fn convert_to_option(input: i32) -> Option<i32> {
    return Some(input)
}
fn convert_to_result(input: i32) -> Result<i32, Error> {
    return Ok(input)
}

