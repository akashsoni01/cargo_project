

async  fn say_hello() {
    println!("hello")
}


#[tokio::main] // the code provide async runtime to main the same code will look like below if not using macro
/*fn main() {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("without tokio::main the async function will work on block");
    })
}*/
async fn main() {
    // say_hello won't run until await;
    let op = say_hello(); // op: impl Future<Output=()>+Sized

    println!("hello world");

    op.await;
}