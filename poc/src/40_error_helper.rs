use std::fs::File;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // let f = File::open("main.jpg").unwrap();
    let f:Result<File> = File::open("main.jpg").expect("Unable to open file");
    // let f = File::open("main.jpg");
    // match f {
    //     Ok(f) => {
    //         println!("file found {:?}", f);
    //     },
    //    Err(e) => {
    //         println!("file not found \n{:?}", e);
    //     }
    // }

    // let f = File::open("main.jpg").unwrap();
    // let f = File::open("main.jpg").expect("Unable to open file");
    // println!("Continuing on with the execution");


}