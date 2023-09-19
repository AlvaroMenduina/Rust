// vectors

fn main(){

    let mut v = Vec::new();      // THis is a new empty vector -> like a Python list
    v.push(10);                     // Rust equivalent of the "list.append()"
    v.push(20);
    v.extend(30..35);               // Extend take an "iterator"

    let first = v[0];           // Interesting, a "slice" of a vector cannot be "mutable"
    let maybe_first = v.get(0);     // Using the get() method makes sure it doesn't break if out of range
    // slices of Vectors do NOT need pointers? Compared to arrays
    // let slice = &arr[0..2]

    println!("Vector {:?}", v);
    println!("First item: {}", first);
    // Here we have 2 options: (A) we use debug-notation or (B) we unwrap the value
    println!("[A] First item: {:?}", maybe_first);          // This returns a Some(value)
    println!("[B] First item: {}", maybe_first.unwrap());

    let arr = [10, 20, 30];
    let slice = &arr[0];
    println!("\nArray {:?}", arr);
    println!("First item: {}", slice);

    // -----------------------------------------
    dump(&arr);

    // -----------------------------------------
    // [Iterators]
    let iterator = [10, 20, 30];
    for i in iterator.iter() {
        println!("{}", i);
    }

    // But interestingly, slices get turned directly into ITERATORS
    // so you don't need the iter method
    for i in &iterator {
        println!("{}", i);
    }

    // Even faster to do
    for i in 0..iterator.len() {
        println!("{}", i);
    }

    // -----------------------------------------
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();
    println!("\n V1 {:?}", v1);
    v1.dedup();         // This is like the np.unique in Python
    println!("\n V1 {:?}", v1);
}

fn dump(arr : &[i32]){
    println!("\nHey! Arr is {:?}", arr);
}