

async  fn say_hello() {
    println!("hello")
}

#[tokio::main]
async fn main() {
    // say_hello won't run until await;
    let op = say_hello(); // op: impl Future<Output=()>+Sized

    println!("hello world");

    op.await;
}