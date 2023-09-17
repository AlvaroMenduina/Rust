// Some macro stuff

// You define Macros like this with "macro_rules!"
macro_rules! print_something {
    // The syntax is: () => {}
    () => {
        // This macro does not take arguments and does
        println!("Something!");
    }
}

// Another macro
macro_rules! print_message {
    ($message:expr) => {
        println!("{}", {$message});     // This macro takes a message and prints it
    }
}

fn main(){
    // Call the macros
    print_something!();

    print_message!("Hello world again!");
}