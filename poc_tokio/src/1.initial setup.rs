// https://tokio.rs/tokio/tutorial/setup

// use tokio::runtime::Handle;
//
// // #[tokio::main
// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
// async fn main() {
//     // let mut runtime = tokio::runtime::
//     println!("Current runtime id: {}", Handle::current().id());
// }


/*use tokio::runtime::Handle;

#[tokio::main(flavor = "current_thread")]
async fn main() {
        println!("Current runtime id: {}", Handle::current().id());
}

*/
/*
fn main() {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    /*
    if you don't want to use async fn then you can also use async block inside non async functions.
    Runs a future to completion on the Tokio runtime. This is the runtime's entry point.
    This runs the given future on the current thread, blocking until it is complete,
    and yielding its resolved result. Any tasks or timers which the future spawns internally will be executed on the runtime.
    */
    runtime.block_on(async {
        println!("hello world from async block or runtime")
    })
}
*/


use mini_redis::{client};
#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    /*
    concurrency - if you are altering into two task then you are working on both task concurrently
    parallelism - two people working, one dedicated to each task

    pros of tokio - async code allows you to work on many tasks concurrently although tokio can
    */


    // jaha b future return hoga usko await kr do and ? is to unwrap from result
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result = {:?}", result);

    Ok(())
}

/*
sync order - line by line code run, one after another
async order - the operation that can not complete immediately are suspended to the background.
so that thread can not blocked and continue to run other things.
*/

/*
don't forget to resume state once async operation completes.
*/

/*
to run a async funtion use await keyword
*/