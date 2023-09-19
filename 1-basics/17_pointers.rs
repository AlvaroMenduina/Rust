// Practising REFERENCING & and DE-REFERENCING *

fn func(x : i32) -> i32 {
    x + 1
    // Normal function, with no pointers
}

fn by_ref(x : &i32) -> i32 {
    *x + 1
    // If you simply do: x + 1, it still works, so the DE-REFERENCING is not needed?
}

// fn by_ref_wrong(x : i32) -> i32 {
//     // [ERROR] -> This one does not work: by_ref(&i)
//     *x + 1
    
//     // Reason: it expects an i32 as argument, not a pointer. so if you pass &i - the rust 'borrow' it doesn't work
//     // You need to tell it that it's a reference by doing (x : &i32)
// }

fn modifies(x: &mut f64) {
    // A function that modifies the argument
    // You need to define the argument as a POINTER and de-reference it
    *x = 1.0;

    // This seems to be operating on the HEAP mem not the stack, as we can later access the value x
}

fn main() {

    // Case 1 - Normal function: pass the whole argument
    let i = 10;
    println!("[Case 1] - Result: {}\n", func(i));

    let j = 20;
    let res2 = by_ref(&j);
    println!("[Case 2] - Result: {}\n", res2);

    let res3 = by_ref(&41);
    // Here it doesn't work if you do by_ref(41) directly
    println!("[Case 3] - Result: {}\n", res3);

    let mut some_val = 0.0;      // --> if you remove the "mut" here, then it will say "cannot borrow `some_val` as mutable, as it is not declared as mutable"
    // modifies(&some_val);         --> Somehow this doesn't work, it throws a "types differ in mutability" because it doesn't consider that pointer mutable
    modifies(&mut some_val);
    println!("res is {}", some_val);
}