
fn main() {
    multi_line_string_with_new_line();
}
fn multi_line_string_with_new_line() {
    let multi_line = r#"This is a 
    multi line string"#;

    println!("{}", multi_line);
}