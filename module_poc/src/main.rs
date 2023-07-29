mod player;

fn main() {
    println!("Hello, world!");

    player::play_video( "some video");
    player::pause_video( "some video");
    some_module::some_function();
    some_module::sub_module::sub_function();
}


// we can create module in same file as well 

pub mod  some_module {
    pub fn some_function() {
        println!("some function");
    }


    // you can create sub module as well in a file 

    pub mod sub_module {
        pub fn sub_function() {
            println!("sub function");
        }
    }
}