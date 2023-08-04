#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // the for loop is used to iterate over a range, will print the square of numbers from 1 to 10
    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["cat", "dog", "chihuahua", "bear", "hamster"];
    // the for loop can be used with continue and break statements.
    // The below code will print. and won't print hamster because of break statement 
    
    /*
    I love my cat
    I love my dog
    chihuahua barks too much
    bear is not a pet
    */

    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("{} barks too much", pet);
            continue
        }
        if pet == &"bear" {
            println!("{} is not a pet", pet);
            break
        }
        println!("I love my {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1} ", nb, square);
    }
}