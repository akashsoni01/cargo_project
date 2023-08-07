#[allow(unused_variables)]
#[allow(unused_assignments)]

trait Summable<T> {
    fn sum(&self) -> T;
}

// implement summable for array
impl Summable<i32> for [i32; 5] {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

// implement summable for vector
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

// implement summable for slice
impl Summable<i32> for &[i32] {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

// implement summable for different types like float
impl Summable<f64> for Vec<f64> {
    fn sum(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum());
    // let b = vec![1.0, 2.0, 3.0];
    // println!("sum float = {}", b.sum());
}