use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main] // the code provide async runtime to main the same code will look like below if not using macro
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listner.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}