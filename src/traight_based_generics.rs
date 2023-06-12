trait Shape {
    fn area(&self) -> f64;
    fn display(&self);
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn display(&self) {
        println!("Rectangle with width: {} and height: {}", self.width, self.height);
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn display(&self) {
        println!("Circle with radius: {}", self.radius);
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let circle = Circle {
        radius: 3.0,
    };

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(rectangle),
        Box::new(circle),
    ];

    for shape in shapes {
        shape.display();
        println!("Area: {}", shape.area());
    }
}
// In this example, we define a trait called Shape with two methods: area() and display(). The area() method calculates the area of a shape, and the display() method prints information about the shape.

// We then implement the Shape trait for two types: Rectangle and Circle. Each implementation provides the necessary implementations for the area() and display() methods specific to the shape.

// In the main function, we create instances of Rectangle and Circle and store them in a vector of boxed trait objects (Vec<Box<dyn Shape>>). This allows us to store different shapes in a single collection.

// We iterate over the shapes in the vector and call the display() and area() methods on each shape. The dynamic dispatch ensures that the appropriate implementations of the methods for each shape are called.

// This code demonstrates Rust's trait-based generics, where the Shape trait acts as an interface that allows multiple types (Rectangle and Circle) to be treated uniformly when they implement the required methods.
//  It promotes code reuse, enables polymorphism, and allows for the creation of generic functions or collections that can work with different types that satisfy the specified trait constraints.