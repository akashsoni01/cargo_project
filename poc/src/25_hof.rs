#[allow(unused_variables)]
#[allow(unused_assignments)]

/// higher order functions 
/// 
fn main() {
    // let square = |a: i32| a * a;
    let sum = |a:i32| { a + a};
    let square = |a: i32| a * a;
    apply(square, 6);

    // Calculate the sum of all the squares less than 500
    // only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit { break; }
        else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("The sum is {}", sum);

    //With HOFs

    // some other higher order functions are: any(), all(), find(), position(), max(), min(), sum(), product(), collect(), etc.
    /* example of each are 
    // any() returns true if any of the elements in the iterator satisfies the predicate
    let a = [1, 2, 3];
    assert!(a.iter().any(|&x| x != 2));
    assert!(!a.iter().any(|&x| x == 4));

    // all() returns true if all of the elements in the iterator satisfies the predicate
    let a = [1, 2, 3];
    assert!(a.iter().all(|&x| x > 0));
    assert!(!a.iter().all(|&x| x > 2));

    // find() returns the first element that satisfies the predicate
    let a = [1, 2, 3];
    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));

    // position() returns the index of the first element that satisfies the predicate
    let a = [1, 2, 3];
    assert_eq!(a.iter().position(|&x| x == 2), Some(1));

    // max() returns the maximum element in the iterator
    let a = [1, 2, 3];
    assert_eq!(a.iter().max(), Some(&3));

    // min() returns the minimum element in the iterator
    let a = [1, 2, 3];
    assert_eq!(a.iter().min(), Some(&1));

    // sum() returns the sum of all the elements in the iterator
    let a = [1, 2, 3];
    assert_eq!(a.iter().sum::<i32>(), 6);

    // product() returns the product of all the elements in the iterator
    let a = [1, 2, 3];
    assert_eq!(a.iter().product::<i32>(), 6);

    // collect() collects the elements in the iterator into a collection
    let a = [1, 2, 3];
    let b: Vec<_> = a.iter().cloned().collect();
    assert_eq!(b, vec![1, 2, 3]);

    // collect() can also be used to convert one collection into another
    let a = [1, 2, 3];
    let b: Vec<_> = a.iter().map(|&x| 2 * x).collect();
    assert_eq!(b, vec![2, 4, 6]);

    
    */

    // 0.. is an iterator from 0 to infinity
    // map() is a method that takes a closure with one parameter and returning square for each value, similar to map() in JS and swift
    // take_while() is a method that takes a closure and checking the limit, similar to prefix() in swift
    // filter() is a method that takes a closure and filtering the even numbers, similar to filter() in JS and swift
    // fold() is a method that takes a closure and summing the values, similar to reduce() in JS and swift

    let sum2 =
        (0..).map(|x| x * x)
            .take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum, x| sum + x);
    println!("The sum using HOFs is {}", sum2);
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}