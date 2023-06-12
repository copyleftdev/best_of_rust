use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel for sending and receiving messages
    let (sender, receiver) = mpsc::channel();

    // Spawn two threads to send messages
    let sender1 = sender.clone();
    thread::spawn(move || {
        sender1.send("Hello from Thread 1").unwrap();
    });

    let sender2 = sender.clone();
    thread::spawn(move || {
        sender2.send("Hello from Thread 2").unwrap();
    });

    // Receive and print messages
    println!("{}", receiver.recv().unwrap());
    println!("{}", receiver.recv().unwrap());
}


// In this example, we use Rust's concurrency features and message-passing channels from the std::sync::mpsc module.

// We start by creating a channel using mpsc::channel(), which returns two ends of the channel: a Sender and a Receiver.

// Next, we clone the Sender so that we have two separate senders to use in different threads.

// We then spawn two threads using thread::spawn() and move the respective senders into the closures of the threads. Each thread sends a message to the receiver end of the channel using send().

// Finally, in the main thread, we use receiver.recv() to receive and print the messages from the channel. We call recv() twice to receive messages from both threads.

// The unwrap() method is used to extract the values from the Result returned by recv() and panics if there is an error (in this case, if the sender has disconnected).

// This example demonstrates Rust's ability to handle concurrency safely by using message-passing channels to communicate between threads. The channels ensure that data races are avoided, and the ownership system enforces proper sharing and synchronization of data across threads.