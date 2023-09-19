// Interesting property

fn take_ownership(some_str : String){
    // This function takes ownership of whatever String is passed as argument
    // it does the same as '19_string.rs' it "drops" the variable 's' in the main scope,
    // such that only 'some_str' is in scope and points to the value "Hello"
    println!("Here: {}\n", some_str);
}

// But we can fix this by relying on POINTERS!
fn retain_ownership(some_ptr : &String){
    // By telling the function it will take a Pointer to a string

    println!("Look we call the string: {}", *some_ptr);     // and just de-referencing this value

    // it means the actual string argument will remain in-scope
    // and can be called again
}

fn main(){

    let s = String::from("Hello");

    take_ownership(s);

    //try to print s now!   --> this will cause an error "value borrowed here after move"
    // println!("Again: {}", s);

    // ---------------------------------------------
    let new_s = String::from("Hello");

    retain_ownership(&new_s);       // pass the pointer
    println!("Again: {}", new_s);


    std::mem::size_of_val(&x)


    fn main() {
        // Because of the annotation, the compiler knows that `elem` has type u8.
        let elem = 5u8;
    
        // Create an empty vector (a growable array).
        let mut vec = Vec::new();At th
        // is point the compiler doesn't know the exact type of `vec`, it
        // just knows that it's a vector of something (`Vec<_>`).
    
        // Insert `elem` in the vector.
        vec.push(elem);
        // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
        // TODO ^ Try commenting out the `vec.push(elem)` line
    
        println!("{:?}", vec);
    }

    //6.1 From trait implementation
    //6.3 Parsing strings

}