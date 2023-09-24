// Tuples can contain multiple types, unlike arrays, vectors, and hashmaps

fn add_mul(x : f64, y : f64) -> (f64, f64){
    (x + y, x * y)
}


fn main(){
    let t = add_mul(2.0, 10.0);
    println!("t = {:?}", t);

    // Tuples have indices like this
    println!("t.0 = {}", t[0]);
}