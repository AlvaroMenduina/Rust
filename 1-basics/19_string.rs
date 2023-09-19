//
fn main(){
    let s1 = String::from("Hello");
    let _s2 = s1;
    println!("{}", s1);     // This doesn't work because s1 has moved the "Hello" to s2, and memory has been cleared
    // so it complains: "borrow of moved value: `s1`"

    // Although s1 and s2 are both "pointers" in the stack to a value on the heap "Hello", in rust values cannot have two owners
    // and because copying in the heap can be expensive, when we assign s2, instead of copying, it just drops s1, so that s2 now owns "Hello"
}
