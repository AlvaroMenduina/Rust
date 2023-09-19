// test about Stack and Heap memory

// [STACK] - fast memory allocation; LIFO: last in, first out; use and clear local variables' memory allocation 
// Issue: values cannot be shared across functions, like when you declare a "global" variable

// [HEAP]
// let x = Box::new(5);
// So if the stack is faster and easier to manage, why do we need the heap? 
// A big reason is that Stack-allocation alone means you only have 'Last In First Out (LIFO)' semantics for reclaiming storage. 
// Heap-allocation is strictly more general, allowing storage to be taken from and returned to the pool in arbitrary order, but at a complexity cost

fn foo(i: &i32) {
    let z = 42;
}

fn main(){
    let mut p = 42;
    p += 257 as i32;
    println!("Variable is {}", p);

    //
    let x = 5;
    let y = &x;

    foo(y);
}