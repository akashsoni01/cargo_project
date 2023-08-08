/*
a way to send data between threads. 
MPSC stands for multiple producer, single consumer.


sender.send(message).unwrap(); // blocking
sender.try_send(message).unwrap(); // non-blocking
receiver.recv().unwrap(); // blocking
receiver.try_recv().unwrap(); // non-blocking

Thread communication using channels:
Channels are useful for sending data between threads in a synchronous way.
A channel has two endpoints: the sender and the receiver.
The sender endpoint can be cloned to send multiple messages.
*/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel(); // Create a channel

    thread::spawn(move || { // Spawn a thread
        let message = "Hello from the thread!";
        sender.send(message).unwrap(); // Send a message
    });

    // The main thread waits for the message to be received
    let received_message = receiver.recv().unwrap(); // Receive a message
    println!("Received message: {}", received_message); // Received message: Hello from the thread!
}
