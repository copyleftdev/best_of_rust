fn main() {
    let mut my_string = String::from("Hello"); // Create a mutable String

    // Pass ownership of my_string to the modify_string function
    modify_string(&mut my_string);

    println!("{}", my_string); // This will print "Hello, World!"
}

fn modify_string(input: &mut String) {
    input.push_str(", World!"); // Modify the string by appending ", World!"
}
// In this example, we start by creating a mutable String named my_string with the value "Hello". We then pass ownership of my_string to the modify_string function using a mutable reference (&mut). The function appends ", World!" to the string using the push_str method.

// After the function call, we attempt to print my_string in the main function. Since we passed a mutable reference to modify_string, the function was able to modify the original string. Thus, when we print my_string, it now contains "Hello, World!".

// This example demonstrates Rust's ownership system, where ownership of resources is transferred and controlled to ensure memory safety. The use of mutable references (&mut) allows modifying data while maintaining strict rules to prevent data races and undefined behavior.
