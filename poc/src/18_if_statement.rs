use rand::Rng;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn test_match() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 11);
    // if num >= 5 {
    //     println!("Number {} is greater than or equal to 5", num);
    // } else {
    //     println!("Number {} is smaller than 5", num);
    // }

    if num > 5 {
        println!("{} > 5", num);
    } else if num == 5 {
        println!("{} == 5", num);
    } else {
        println!("{} < 5", num);
    }

    let res = if num >= 5 { true } else { false };
    println!("{}", res);
}