fn main() {
    #[cfg(target_os = "linux")]
    println!("Hello, Linux!");

    #[cfg(target_os = "windows")]
    println!("Hello, Windows!");

    #[cfg(target_os = "macos")]
    println!("Hello, macOS!");
}
// In this example, we use the #[cfg(...)] attribute to conditionally compile and execute different code blocks based on the target operating system.

// If the target operating system is Linux (target_os = "linux"), the program will print "Hello, Linux!".
// If the target operating system is Windows (target_os = "windows"), the program will print "Hello, Windows!".
// If the target operating system is macOS (target_os = "macos"), the program will print "Hello, macOS!".
// By leveraging the #[cfg(...)] attribute and target-specific configurations, you can write code that adapts to different operating systems, making your Rust programs portable and cross-platform compatible.