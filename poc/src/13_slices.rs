#[allow(unused_variables)]
#[allow(unused_assignments)]

fn slices_test() {
    // slices are a reference to a part of array or vector
    // slices are immutable by default
    // slices are mutable if declared with mut keyword
    // slices are declared with square brackets []
    // slices are declared with type and size
    // slices are declared with type and size and values
    // slices are declared with type and size and default values
    // slices are declared with type and size and default values and mutable

    let primes = [2, 3, 5, 7, 11]; // arrey bna i32 type ka size hai uska 5
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0]; // arrey bna f64 type ka size hai uska 4

    let slices1 = &primes[0..3]; // slice bna arrey ka 0 se 3rd index tak
    println!("{:?}", slices1); // print slice

    let slices2 = &primes[..3]; // slice bna arrey ka 0 se 3rd index tak
    println!("{:?}", slices2); // print slice

    let slices3 = &primes[2..]; // slice bna arrey ka 2 se last index tak
    println!("{:?}", slices3); // print slice

    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);

    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);

}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
    // colors_slice[2] = "brown";
}