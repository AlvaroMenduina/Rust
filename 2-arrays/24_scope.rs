// Scope of variables
// We can only 'borrow' variables that remain in scope
// The variable 'tmp' is dropped after the inner block
// So Rust does not let us keep a stale point 'rf' to 'tmp'

fn main(){

    let s1 = "Hey!".to_string();        // We create the variable S1
    let mut rf = &s1;                   // We 'borrow' S1 with a pointer "S1"
    println!("REF: {}", rf);

    {
        let tmp = "hello world".to_string();    // We create a 'tmp' variable 
        // let i = 32;                          // We could not re-assign RF to an integer, the inferred type is string
        rf = &tmp;                              // We 'borrow' tmp into S1
        println!("REF: {}", rf);
    }
    println!("REF: {}", rf);        // This gives a "borrowed value does not live long enough"
}