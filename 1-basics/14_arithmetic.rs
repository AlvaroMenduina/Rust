// Practice arithmetic

// This code fails on 2 points: the type mixing for the Trait "AddAsign" and the "inmutability" of the variable sum

fn main(){
    let mut sum = 0.0;        // Change this between 0 and 0.0

    for i in 0..5 {
        // sum += i;           // This should fail because sum has been declared 'implicitly' as float, and we are adding an integer
        sum += i as f64;        // Change this to "cast" the i integer as a float64
    }
    // Rust wants you to do 'casting' explicitly
    println!("The sum is {}", sum)
}