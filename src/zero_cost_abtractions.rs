fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum: i32 = numbers.iter().sum();
    let average: f64 = numbers.iter().sum::<i32>() as f64 / numbers.len() as f64;

    println!("Sum: {}", sum);
    println!("Average: {:.2}", average);
}

// In this example, we start by creating a Vec named numbers containing a sequence of integers.

// We then use two higher-order functions, iter() and sum(), to calculate the sum and average of the numbers.

// The iter() function returns an iterator over the elements of numbers, and the sum() function is called on that iterator to calculate the sum of the elements.

// The result of sum() is assigned to the variable sum, which has the type i32.

// To calculate the average, we use similar code but cast the sum to f64 and divide it by the length of numbers cast to f64.

// The result is assigned to the variable average, which has the type f64.

// Finally, we use println!() to display the sum and average on the console. The "{:.2}" format specifier is used to display the average with two decimal places.

// This example demonstrates Rust's zero-cost abstractions by allowing us to perform calculations on collections using higher-order functions without incurring additional runtime overhead. The code is concise, expressive, and performs efficiently without sacrificing performance.