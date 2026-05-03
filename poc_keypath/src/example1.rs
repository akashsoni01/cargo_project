use std::sync::{Arc, RwLock};
use key_paths_derive::Kp;

#[derive(Debug, Kp)]
struct Level1 {
    field1: String,
    field2: Option<String>,
    field3: Option<Level12>,
    field4: Arc<std::sync::RwLock<Level12>>,
}

#[derive(Debug, Kp)]
struct Level12 {
    field1: String,
    field2: String
    // field2: Option<Box<String>>
}

impl Level1 {
    fn from() -> Self {
        todo!()
    }

    // fn new() -> Self {
    //     Self { 
    //         field1: String::from("field1"),
    //         field2: Some(String::from("field1")),
    //        field3:  Some(Level12 {
    //                        field1: String::from("field1"),
    //                        field2: Some(String::from("field1")),
    //                         // field2: Box::new(Some(String::from("field1"))),
    //                    }),
    //        field4: Arc::new(
    //         RwLock::new(
    //             Level12 {
    //             field1: String::from("field1"),
    //             field2: Some(String::from("field1")),
    //             // field2: Box::new(Some(String::from("field1"))),
    //         }
    //         )
    //       )
    //      }
    // }
}

// How we are going to acces the field
fn access_field_imp() {
    let instance = Level1::from();
    let l2f2 = instance.field3.unwrap().field2;             
    println!("instance.field2 = {:?}", l2f2);

    // if let Some(l2f2) = instance.field3.and_then(|f| f.field2) {
    //         println!("instance.field2 = {:?}", l2f2);
    // }


}

// What we are going to access the field
fn acess_field_dec() {
    let kp = Level1::field1();
    let instance = Level1::from();

    // Level1::field4() >> Level12::field2()
    if let Some(result) = Level1::field3()
    .then(Level12::field2())
    .get(&instance) {
        if result.len() > 0 {
            println!("result valid string = {:?}", result);
        }
    }

    // if let Some(result) = Level1::field4_lock()
    // .then(Level12::field2())
    // .get(&instance) {
    //     if result.len() > 0 {
    //         println!("result valid string = {:?}", result);
    //     }
    // }

}


fn main() {
        access_field_imp();
        acess_field_dec()
}
