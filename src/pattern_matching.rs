enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Stormy,
}

fn main() {
    let today_weather = Weather::Sunny;

    match today_weather {
        Weather::Sunny => println!("It's a sunny day!"),
        Weather::Cloudy => println!("It's a cloudy day."),
        Weather::Rainy => println!("It's raining."),
        Weather::Stormy => println!("There's a storm approaching!"),
    }
}
// In this example, we define an enum called Weather that represents different weather conditions. It has four variants: Sunny, Cloudy, Rainy, and Stormy.

// In the main function, we create a variable today_weather and assign it the value Weather::Sunny.

// We then use a match expression to pattern match on the value of today_weather. Each variant of the Weather enum is matched against and the corresponding code block is executed based on the matched variant.

// Depending on the value of today_weather, the corresponding message will be printed to the console. In this case, since today_weather is Weather::Sunny, the program will print "It's a sunny day!".

// Pattern matching in Rust allows for concise and expressive control flow based on the structure and content of values. It enables handling different cases in a comprehensive and readable manner, making it easier to write code that deals with various scenarios.