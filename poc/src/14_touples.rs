#[allow(unused_variables)]
#[allow(unused_assignments)]

fn touples_test() {
    // Tuple
    // A tuple is a collection of values of different types.
    // Tuples are constructed using parentheses (),
    // and each tuple itself is a value with type signature (T1, T2, ...),
    // where T1, T2 are the types of its members.
    // 

    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);
    println!("{:?}", person.0);
    person.0 = "Mike";
    println!("{:?}", person.0);
    let (name, age, employment) = person;
    println!("name: {}, age: {}, employed: {}", name, age, employment);
}

