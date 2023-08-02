#[allow(unused_variables)]
#[allow(unused_assignments)]

fn vectors_test() {
        // similar to array except we can add and remove elements. 
    // vectors are a collection of values of same type 
    // vectors are dynamic size 
    // vectors are allocated on heap
    // elements value can be modified and deleted
    // indexed :- 0 to size-1
    // vectors are slower than arrays
    // vectors are immutable by default
    // vectors are mutable if declared with mut keyword
    // vectors are declared with square brackets []
    // vectors are declared with comma separated values
    // vectors are declared with type and size
    // vectors are declared with type and size and values
    // vectors are declared with type and size and default values
    // vectors are declared with type and size and default values and mutable

    let vectors = vec![1, 2, 3, 4, 5]; // vector bna i32 type ka size hai uska 5
    let mut vectors = vec![1, 2, 3, 4, 5]; // vector bna i32 type ka size hai uska 5, mutable

    let mut primes: Vec<i32> = vec![2, 3, 5, 7, 11]; // vector bna i32 type ka size hai uska 5, mutable
    let mut primes3: Vec<i32> = Vec::new();  // vector bna i32 type ka size hai uska 5, mutable
    
    
    println!("Print before push {:?}", vectors); // print vector

    vectors.push(6); // vector me 6 add krdo
    println!("Print after push {:?}", vectors); // print vector

    vectors.pop(); // vector me se last element delete krdo
    println!("Print before pop {:?}", vectors); // print vector

    vectors.remove(2); // vector me se 2nd index element delete krdo
    println!("Print after after remove from index 2 {:?}", vectors); // print vector

    vectors[2] = 5; // vector ka 2nd index ko 5 krdo
    println!("{:?}", vectors); // print vector

    for vector in vectors.iter() { // vector ka har element print krdo
        println!("{}", vector);
    }

}