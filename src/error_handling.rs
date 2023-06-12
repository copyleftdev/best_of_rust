use std::fs::File;
use std::io::Read;

fn read_file_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}

// In this example, we have a function read_file_contents that reads the contents of a file given its filename. It returns a Result<String, std::io::Error>, where String represents the file contents if the read operation is successful, and std::io::Error represents an error if the read operation fails.

// Inside the read_file_contents function, we attempt to open the file using File::open(filename). The ? operator is used to propagate any errors that occur during the file opening process. If the file is successfully opened, we create a mutable String named contents and read the file contents into it using file.read_to_string(&mut contents)?.

// In the main function, we call read_file_contents("example.txt") and match on the Result returned. If the Result is Ok, we print the file contents. If the Result is Err, we print the error message using eprintln!.

// This code example demonstrates Rust's error handling mechanism using the Result type. By using Result and the ? operator, we can easily propagate and handle errors, ensuring that potential failures are handled gracefully and explicitly.