fn main() {
    let test_var = TestingStruct{ x: 1, y: 2 };

    test_var.test_x();
    // println!("{}", Employee::static_fn_detail()); // call the static method
    TestingStruct::test_static_function();
}

struct TestingStruct {
     x: u8,
     y: u8,
}

impl TestingStruct {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn test_x(&self)  {
        println!("{}", self.x); // call the method
    }

    pub fn test_static_function()  {
        println!("hello world"); // call the method
    }

}