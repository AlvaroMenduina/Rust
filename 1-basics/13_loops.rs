
// practising loops.rs

fn main(){

    for i in 0..5 {
        println!("Hello {}", i);
    }

    // More stuff - print if even or odd
    for i in 0..5 {
        if i % 2 == 0 {
            println!("This number {} is even", i);
        }
        else {
            println!("This number {} is odd", i);
        }
    }

    // More interestingly
    for i in 5..10 {
        let even_or_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("Hey - this number {} is {}", i, even_or_odd);
    }

}