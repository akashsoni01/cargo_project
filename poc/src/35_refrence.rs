
/*
https://dhghomon.github.io/easy_rust/Chapter_16.html
*/

fn main() {
    println!("{}", return_string());
}

fn create_any_number_of_borrowed_refrence() {
    let country = String::from("Austria");
    let ref_one = &country;
    let ref_two = &country;

    println!("{}", ref_one);
}

/*
fn return_str() -> &'str {
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref // ⚠️
}

error[E0106]: missing lifetime specifier
  --> src/main.rs:26:20
   |
26 | fn return_str() -> &str {
   |                    ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
   |
26 | fn return_str() -> &'static str {
   |                     +++++++
help: instead, you are more likely to want to return an owned value
   |
26 | fn return_str() -> String {
   |                    ~~~~~~

*/

















/*
fn return_str() -> &'static str {
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref // ⚠️
}

error[E0515]: cannot return value referencing local variable `country`
  --> src/main.rs:53:5
   |
52 |     let country_ref = &country;
   |                       -------- `country` is borrowed here
53 |     country_ref // ⚠️
   |     ^^^^^^^^^^^ returns a value referencing data owned by the current function



*/

// solution
// insstead of returning owned reference return value
fn return_string() -> String {
    let country = String::from("Austria");
    return country;
}