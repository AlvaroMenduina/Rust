// Arrays

fn main(){
    let arr = [10, 20, 30, 40, 50, 60];     // This is you declare arrays, like a Python 'list'
    let first = arr[0];

    println!("First item: {}", first);

    for i in 0..3 {
        println!("Arr[{}]: {}", i, arr[i]);
    }

    println!("\nLength of Arr: {}", arr.len());

    // Passing SLICES to a function
    let result = do_sum(&arr);          // We need to pass the pointer to the array
    println!("\nResult of summing the Arr {:?}: {}", arr, result);

    // Slices MUST borrow from the array - there cannot be 2 owners
    let slice1 = &arr[0..2];
    println!("\nThe slice: {:?}", slice1);

    // Get method for slices
    let the_first = slice1.get(0);
    let the_last = slice1.get(2);           // Get method here is accessing (incorrectly) the 3rd item on the slice, this should normally give error

    println!("first {:?}", the_first);
    println!("last {:?}", the_last);                        // this shouldn't work. this is an "Option" type, which has the .unwrap() method
    // println!("Show me the last: {}", the_last.unwrap())     // [BUG] this throws a run-time error

    let my_last = if the_last.is_some() {*the_last.unwrap()} else {-1};         // It seems like the_last.unwrap() is a pointer so we need to de-reference it
    println!("Show me the last: {}", my_last);

    // a quicker option is to use the metho
    let my_other_last = *slice1.get(2).unwrap_or(&-1);
    println!("Show me the last: {}", my_other_last) ;
}

fn do_sum(arr : &[i32]) -> i32 {
    // If you do this: fn do_sum(arr : [i32]) -> i32 { -- it will compain that the size of the array cannot be known at compilation time

    // By doing (arr : &[i32]), the compiler will know the size of the pointer at compilation time, which is fixed, it's an address!
    // But the actual array can vary in length
    let mut sum = 0;

    for i in 0..arr.len() {
        sum += arr[i];
    }
    sum
}