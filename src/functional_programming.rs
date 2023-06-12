fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Using closures and iterators to perform operations on the numbers
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    let sum: i32 = numbers.iter().sum();
    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

    println!("Squared: {:?}", squared);  // Prints "[1, 4, 9, 16, 25]"
    println!("Sum: {}", sum);  // Prints "15"
    println!("Even numbers: {:?}", even_numbers);  // Prints "[2, 4]"
}
// In this example, we start with a Vec named numbers containing a sequence of integers.

// We then use closures, iterators, and higher-order functions to perform various operations on the numbers.

// The map() function applies a closure to each element of the numbers vector, squaring each number. The result is collected into a new Vec named squared.

// The sum() function calculates the sum of all the elements in the numbers vector.

// The filter() function uses a closure to filter out only the even numbers from the numbers vector. The cloned() method is used to create a new vector containing the cloned values.

// Finally, we use println!() to display the squared numbers, the sum, and the even numbers on the console.

// This code example demonstrates Rust's functional programming features, such as closures, iterators, and higher-order functions. It allows for concise and expressive code by applying transformations and performing operations on collections in a functional style.