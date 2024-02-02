// create a custom CustomHashMap using type T and V without using std 
use std::collections::HashMap;

fn testHashMap() {
    let mut hashmap= HashMap::new();
    hashmap.insert("a","string");
    hashmap.insert("b","string");
    hashmap.insert("c","string");
    println!("{:?}", hashmap);      

    for (key, value) in &hashmap {
        println!("{}: {}", key, value);
    }

    let mut hashmap2= HashMap::new();
    hashmap2.insert("a","string");
    hashmap2.insert("b","string");
    hashmap2.insert("c","string");

    // `&std::collections::hash_map::Iter<'_, &str, &str>` is not an iterator
    // for (key, value) in &hashmap2.iter() {
    //     println!("{}: {}", key, value);
    // }

    for (key, value) in hashmap2.iter() {
        println!("{}: {}", key, value);
    }

}