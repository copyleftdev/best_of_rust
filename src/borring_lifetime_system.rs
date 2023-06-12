fn main() {
    let message = String::from("Hello, Rust!"); // Create a String

    print_message(&message); // Pass a reference to the print_message function

    // Attempting to use `message` here would result in a compilation error
    // since it was borrowed immutably in the print_message function.
}

fn print_message(message: &String) {
    println!("{}", message); // Print the message
} // The borrowed reference goes out of scope and is dropped

// In this example, we create a String named message containing the text "Hello, Rust!". We then pass a reference to this String to the print_message function using an immutable borrow (&).

// Inside the print_message function, we simply print the message. The reference to message allows us to access its contents without taking ownership of it.

// After the function call, attempting to use message again in the main function would result in a compilation error. This demonstrates Rust's borrowing rules, which prevent multiple mutable references or a combination of mutable and immutable references to the same data. The compiler ensures that references are used correctly and that there are no data races or dangling references.

// The code example showcases how Rust's borrowing and lifetime system enables safe and efficient memory management by enforcing strict rules for referencing and accessing data.
