fn main() {
    
    // Using the 'move' statement to access value
    let value = 42;
    let closure = move || {
        println!("Captured value: {}", value);      // Notice it's taking ownership of "value"
    };
    closure();
    
    // Almost equivalent to declaring the lambda function
    let add_one = |x| x + 1;
    println!("Result: {}", add_one(5));
    
}
