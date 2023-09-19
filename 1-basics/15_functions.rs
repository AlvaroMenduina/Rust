// Rust functions

// Functions follow this syntax: fn some_func (x: its_type) -> output_type {}
fn sqr(x : f64) -> f64 {
    return x * x;
}

fn func(x : &str) -> f64 {
    // Here we have added a 'pointer'-like argument to a string
    
    println!("Reading some string '{}' for fun", x);
    let y = 2.0;
    y
    // [NOTE] This last line is crazy: it doesn't have an explicit 'return' statement and it ouputs the last 'expression' without a semicolon
}

fn abs(x : f64) -> f64 {
    // A function to calculate the Absolute value of a Float
    if x >= 0.0 {
        x
    }
    else {
        -x
    }
}

fn other_abs(x : f64) -> f64 {
    let res = if x >= 0.0 {x} else {-x};
    res
}

fn factorial(n : u64) -> () {
    // A function without return value is marked by (n : type) -> () and making sure there are no "expressions" without ;
    println!("Reading the input '{}' for factorial", n);
    // return or you can just add a 'return' like in Python
}

fn main(){

    let x = 2.0;
    let y = sqr(x);
    println!("\nThe square of {:.1} is {}", x, y);

    // In Rust you can get away with not adding a *return* statement at the end of the function, by default, it will return the value of the last expression
    let z = func("Hey");
    println!("\nThe result of 'func' is: {}", z);

    let val = -3.0;
    let abs_x = abs(val);
    // let abs_x = abs(val);
    println!("\nAbsolute Value of {} is {}", val, abs_x);

    // This is an interesting trick to avoid having to declare a specific variable to contain the result of the other function
    // Just use a 'pointer' to the result &other_abs(val)
    println!("\nAbsolute Value of {} is {}", val, other_abs(val));

    // Factorial
    factorial(34);
}