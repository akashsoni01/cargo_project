#[allow(unused_variables)]
#[allow(unused_assignments)]

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of"
    }
}

/*
0. I have no oranges
1. I have one or two oranges
2. I have one or two oranges
3. I have a few oranges
4. I have a few oranges
5. I have a few oranges
6. I have a few oranges
7. I have a few oranges
8. I have an even amount of oranges
9. I have lots of oranges
10. I have an even amount of oranges
11. I have lots of oranges
12. I have an even amount of oranges
13. I have lots of oranges
14. I have an even amount of oranges
(2, 5)
*/

fn test_pattern_match() {
    for i in 0..15 {
        println!("{}. I have {} oranges", i, get_oranges(i));
    }

    // let point = (0, 0);
    // let point = (6, 0);
    let point = (0, 5);
    // let point = (2, 5);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("({}, {})", x, y),
    }
}

