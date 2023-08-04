use crate::Suit::{Heart, Club, Diamond, Spade};

#[allow(unused_variables)]
#[allow(unused_assignments)]

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn country(code: i32) {
    // below code will return UK if code is 44,
    // Spain if code is 34,
    // unknown if code is between 1 and 999 
    // and invalid if code is anything else
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => { println!("\u{2665}") }
        Spade => { println!("\u{2660}") }
        Club => { println!("\u{2663}") }
        Diamond => { println!("\u{2666}") }
    }
}
// Match statement is similar to when or switch in other language 

fn test_match() {
    print_choice(Heart);
    print_choice(Club);
    print_choice(Diamond);
    print_choice(Spade);

    country(44);
    country(34);
    country(125);
    country(-15);
}


