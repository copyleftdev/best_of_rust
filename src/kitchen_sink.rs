use std::sync::mpsc;
use std::thread;

trait Printable {
    fn print(&self);
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle with width: {} and height: {}", self.width, self.height);
    }
}

impl Printable for Circle {
    fn print(&self) {
        println!("Circle with radius: {}", self.radius);
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let my_string = String::from("Hello, Rust!");

    // Concurrency: Spawn a thread to print the string concurrently
    thread::spawn(move || {
        println!("{}", my_string);
    });

    // Trait-based generics: Print shapes using the Printable trait
    let rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let circle = Circle {
        radius: 3.0,
    };

    rectangle.print();
    circle.print();

    // Functional programming: Sum all even numbers in the vector using iterators and closures
    let sum_of_evens: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .sum();

    println!("Sum of even numbers: {}", sum_of_evens);

    // Memory safety and ownership system: Use a message-passing channel to safely communicate between threads
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        sender.send("Message from another thread").unwrap();
    });

    let received_message = receiver.recv().unwrap();
    println!("Received message: {}", received_message);
}
