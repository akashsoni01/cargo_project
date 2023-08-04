#[allow(unused_variables)]
#[allow(unused_assignments)]

fn arrays_test() {
    // arrays are a collection of values of same type 
    // arrays are fixed size 
    // arrays are allocated on stack
    // elements value can be modified but not deleted
    // indexed :- 0 to size-1
    // arrays are faster than vectors
    // arrays are immutable by default
    // arrays are mutable if declared with mut keyword
    // arrays are declared with square brackets []
    // arrays are declared with comma separated values
    // arrays are declared with type and size
    // arrays are declared with type and size and values
    // arrays are declared with type and size and default values
    // arrays are declared with type and size and default values and mutable


    let primes = [2, 3, 5, 7, 11]; // arrey bna i32 type ka size hai uska 5
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0]; // arrey bna f64 type ka size hai uska 4
    println!("{:?}", primes); // print arrey
    println!("{:?}", doubles); // print arrey

    let numbers = [0;15]; // arrey bna i32 type ka size hai uska 15
    println!("{:?}", numbers); // print arrey

    const DEFAULT: i32 = 3; // constant bna i32 type ka
    let mut numbers = [DEFAULT; 15]; // arrey bna i32 type ka size hai uska 15, mutable
    println!("{:?}", numbers); // print arrey
    println!("{:?}", numbers[3]); // print arrey ka 3rd index

    numbers[3] = 5; // arrey ka 3rd index ko 5 krdo
    println!("{:?}", numbers); // print arrey

    for number in numbers.iter() { // arrey ka har element print krdo
        println!("{}", number);
    }

}