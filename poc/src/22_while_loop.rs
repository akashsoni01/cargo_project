#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    get_squares(3478);
    get_cubes(4938);
}

fn get_squares(limit: i32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0} * {0} = {1}", x, x * x);
        x += 1;
    }
}

fn get_cubes(limit: i32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break
        }
    }
}