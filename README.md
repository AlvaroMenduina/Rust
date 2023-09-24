# Rust - Learning notes

This is a collection of notes about the ``rust`` language, especially focusing on a comparison with Python and notable differences.
For details on how to install and run ``rust`` see [here](guide.md)

# Rust vs Python

Apart from the obvious differences between Rust and Python, there are some interesting variations on syntax or the approach to 
Object Oriented Programming (OOP). Here we explore those differences with some examples:

## [1] Basics

### [1.1] Print format

In Rust, we use print like this, using a *macro* ``println!("<format_string>", print_arg)``:

```rust
fn main(){
    let answer = 42;
    println!("Hello {}", answer);    
}
```

Compared to the Python equivalents

```python
print(f"Hello {answer}") or print("Hellow %s" % (answer))
```

### [1.2] One-line declarations
In Python we use one-line declarations like this x = *value* if *condition* else *other_value*

```python
x = 0 if i % 2 == 0 else 1
```

whereas in Rust, the format is x = if *condition* *value* else *other_value*

```rust
let even_or_odd = if i % 2 == 0 {"even"} else {"odd"};
```

### [1.3] Cannot know the type
Apparently you cannot print the *type* of a variable in Rust at runtime, (somehow it's not available). A quick explanation here: (https://www.hackertouch.com/how-to-print-type-of-variable-in-rust.html)

### [1.4] No 'return' at the end of Functions
In Rust you can get away with not adding a *return* statement at the end of the function, by default, it will return the value of the last expression

```rust
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y           // This is the return
}

fn main(){
    let x = 1;
    let y = 2;
    
    let result = add_numbers(x, y);
    println!("x({}) + y({}) = {}", x, y, result);
}

```

### Taking 'string' as arguments to Functions
Cool guide (http://xion.io/post/code/rust-string-args.html)

### Slicing does not make 'copies'
Slicing in Rust uses *pointers* to the array like

```rust
let arr = [10, 20, 30, 40, 50, 60]; 
let slice1 = &arr[0..1];
```

but slices do **not** make a copy of the data in the array, they all *borrow* the data from the declared arrays.
The size of the arrays will be known at *compile* time, while the size of the slice will be known at *runtime*.

### [1.X] Using ``zip`` with iterators
Zipping is a common operation with iterators. In Python, ``zip`` is a built-in function that takes as arguments as many iterators that need 
zipping, an returns the pairwise collection.

```python
a = [0, 1, 2]
b = [4, 5, 6]

for i, j in zip(a, b):
    print(i, j)
```

However in Rust the syntax can be different. You can either use ``use std::iter::zip`` to have the same format, or you can turn them into *iterables* and then *concatenate* the method ``.zip``. In other words: ``<some_vec>.iter().zip(<other_vec>.iter())``.
```rust
let a = vec![0, 1, 2];
let b = vec![4, 5, 6];
for (i, j) in a.iter().zip(b.iter()){
    println!("{}, {}", i, j)
}
```

## [2] Arrays, Vectors, Lists, Tuples

### [2.1] Tuples and their indices

Unlike arrays or ``HashMaps`` (aka dictionaries), tuples in Rust can contain different types. So in that sense, tuples are similar to Python. However, they have a different syntax for accessing their elements. In Python, the syntax is the same for *lists* and *tuples*:

```python
a = [0, 1, 2]
print("a[0] = ", a[0])      # Accessing a list

b = (3, 4, 5)
print("b[0] = ", b[0])      # Accessing a tuple
```

Whereas in Rust, this is incorrect. To access the elements of a tuple, we have to use ``tup.0`` as if it was a *property* of the tuple. By using different syntax for tuples and arrays, Rust reinforces the distinction between these types and helps prevent potential confusion or misuse. It aligns with Rust's principle of explicitness and safety.

```rust
fn add_numbers(x : f64, y : f64) -> (f64, f64){
    // This function takes a 'tuple' (x, y) and returns a 'tuple
    (x + 1, x + 2)
}

fn main(){
    let tup = add_mul(2.0, 10.0);
    println!("t = {:?}", t);

    // This is incorrect! tup[0] does not work
    println!("t.0 = {}", tup[0]);
}

error[E0608]: cannot index into a value of type `(f64, f64)`
  --> .\25_tuples.rs:13:26
   |
13 |     println!("t.0 = {}", t[0]);
   |                          ^^^^ help: to access tuple elements, use: `t.0`

error: aborting due to previous error

```

### [1.X] ``HashMap`` and ``dict``

The ``HashMap`` in Rust is the equivalent of a Python dictionary ``dict``, a 'list' of things that are indexed via a *key* rather than an index. However, Rust has some specific behaviours that Python lacks:

**Dictionaries must contain the same type**. This is unusual; Python can hold *anything* in a dictionary like this, with no regard to the 
type each key is holding.

```python
my_dict = {}
my_dict["name"] = "John"
my_dict["age"] = 30
my_dict["pets"] = ["cat", "dog"]
```

This is not allowed in Rust. All the items must have the same type, to the point that you cannot even mix integers and floats!

## [2] Object-Oriented Programming in Rust

First of all, Rust does not use *inheritance* to define classes; it uses something called *composition* through the definition
of **traits** and its **implementation**. 

Other behaviours include

### [2.1] Initialising a class
In Python, you don't need to tell which args correspond to what property, it will infer it based on the order in which you declared
the arguments for the ``__init__`` method. In the example below, it "knows" that 30 is the *age* and 180 is the *height*

```python
class Person(object):
    def __init__(self, age, height):
        self.age = age
        self.height = height

p = Person(30, 180)
```

In Rust, you need to *explicitly* tell it the properties when initialising the class, like this ```let p = Person{age:30, height:180};```

```rust
struct Person<T> {
    age: T,
    height: T,
}

fn main(){

    let p = Person{30, 180};        // This doesn't work
    println!("Age: {}", p.age)
}
```

### [2.2] Static Methods in Rust vs Python

First of all, in Rust you do not need to **declare** the method as "static" with the decorator ``@staticmethod`` like Python.
Whether a method is static is *inferred* by its relationship with ``self``. But perhaps more importantly, Rust is more strict on
the ways static methods can be used. In Rust, you **cannot** call a static method from an *instance* of a class, you need to call it from
the generic class like this:

```rust
new_person.say_hi();        // This would work in Python but not in Rust
Person::say_hi();           // In Rust, STATIC method cannot be called from an INSTANCE!
```

Python is less strict with this. An instance of a class retains the static method and call be called without concerns:

```python
class Calculator:
    @staticmethod
    def add_numbers(x, y):
        return x, y

calc = Calculator()
res1 = calc.add_numbers(1, 2)        # This is acceptable in Python but not Rust
res2 = Calculator.add_numbers(1, 2)     # This is the Rust equivalent, calling the static method from the Class without instance
```


### Generic types

You might think that you could get away with this "{:?}" to avoid having to add ```impl<T: std::fmt::Display>``` but it will complain about not implementing debug

```rust
impl<T> Person<T> {
    fn print_stats(&self){
        println!("Age: {:?} | Height: {:?}", self.age, self.height)
    }
}
```

# What are Lifetimes?

In Rust, a *lifetime* is a construct used to track how long references are valid. It helps the compiler ensure that references do not outlive the data they point to, preventing dangling references.

```rust
fn a_function<'a>(x: &'a i32) -> &'a i32 {
    // function body
}
```

and for structs

```rust
struct SomeStruct<'a> {
    data: &'a i32,
}

impl<'a> SomeStruct<'a> {
    fn new(data: &'a i32) -> Self {
        SomeStruct { data }
    }
}
```

**When are explicit lifetimes needed?**. For this case

```rust
// Find the longest string, by reference
fn find_longest(s1 : &str, s2 : &str) -> &str{
    if s1.len() > s2.len(){
        s1
    } else {
        s2
    }
}

fn main(){

    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    let result;
    {
        let s1_ref = &s1;       // We borrow a reference to the string
        let s2_ref = &s2;
        result = find_longest(s1_ref, s2_ref);
    }

}
// The error we get!
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:42
  |
2 | fn find_longest(s1 : &str, s2 : &str) -> &str{
  |                      ----       ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`
help: consider introducing a named lifetime parameter
  |
2 | fn find_longest<'a>(s1 : &'a str, s2 : &'a str) -> &'a str{
  |                ++++       ++            ++          ++

```

The error here is indicating that the compiler cannot determine the relationship between the lifetimes of the input references ``s1`` and ``s2`` and the lifetime of the output reference ``&str``. This is because we cannot know the result of the comparison at compile time, so the compiler has no way of implicitly linking the lifetime of the result to that of the inputs.

By adding explicit lifetime annotations as you did (fn find_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str), you are specifying that the output reference will have the same lifetime as the input references. This clarifies the relationship and helps the compiler ensure that the returned reference is valid and doesn't outlive the references it's based on (s1 and s2).

**Lifetimes are only a concern with "borrowed" values**. If you pass actual values as arguments, ownership and lifetime concerns are handled differently because the ownership of the value is transferred to the function. 

**Another example**. The code below has a similar problem. The *struct* is trying to store a reference to a string ``&str``. However, Rust doesn't allow storing references in structs without specifying **lifetimes**.

```rust
struct MyClass {
    s: &str
}

fn main() {
    let result = MyClass {s: "Hello world!"};

    println!("{:?}", result);
}
```

Here we have several options to fix this. First of all, you can specify a generic lifetime ``'a``:

```rust
struct MyClass<'a> {
    s: &'a str
}
```

This means that the reference ``s`` in the struct has a lifetime tied to some specific scope or context, as determined by where an instance of ``MyClass`` is used.

Secondly, we can use ``'static`` to specify that the reference ``s`` is valid for the entire duration of the program, and it's not tied to any specific scope or function. This is commonly used for string literals (e.g., "hello") or other data that exists for the entire program's lifetime

```rust
struct MyClass {
    s: &'static str
}
```

One could think that using ``'static`` risks causing the safety issues that Rust tries so hard to avoid, by referencing something whose lifetimes is not actually as long as the entire program. And you'd be correct. Here is a case where things break down

```rust
#[derive(Debug)]        // Btw, this allows us to debug-print the whole Struct with its contents
struct MyClass {
    s: &'static str,
}

fn main() {
    let s = String::from("hello");
    let a = MyClass { s: &s };  // Error: `s` does not have a `'static` lifetime

    println!("{:?}", a);
}

// --- The error message
error[E0597]: `s` does not live long enough
  --> src/main.rs:8:20
   |
7  |     let s = String::from("hello");
   |         - binding `s` declared here
8  |     let a = MyClass { s: &s };  // Error: `s` does not have a `'static` lifetime
   |                    ^^
   |                    |
   |                    borrowed value does not live long enough
   |                    this usage requires that `s` is borrowed for `'static`
...
11 | }
   | - `s` dropped here while still borrowed
```

The only difference here lies in the use of **string literals** ``&str``, which have a static lifetime and are always available. On the other hand, **String instances** ``&String`` have a lifetime tied to their scope and ownership. When you create a String, it is tied to a specific scope or block, and its lifetime is limited to that scope.

When you try to create a struct ``MyClass`` with a reference to a String ``&String``, you're essentially trying to give ``'static`` lifetime semantics to a reference that is not ``'static``. The Rust compiler will prevent this because it would lead to unsafe and incorrect assumptions about the reference's lifetime.

**Generic vs explicit lifetimes**. One attempted fix is to specify a generic lifetime in the struct definition. However, this still has en error in the ``makes_a`` function. When we say that it should return a struct ``A``, we need to *explicitly* define its lifetime.

```rust
#[derive(Debug)]
struct A<'a> {
    s: &'a str,
}

fn makes_a() -> A {     // This is wrong! Missing the lifetime
    let string = "I'm a little string".to_string();
    A { s: &string }
}

fn main() {
    let a = makes_a();
    println!("{:?}", a);
}

```

In Rust, when you define a struct with a reference as a field, you need to specify a lifetime parameter to indicate the relationship between the struct and the reference. However, when you have a function like ``makes_a`` and you want to return an instance of the struct ``A``, you need to specify the lifetime explicitly in the function signature. Like this

```rust
fn makes_a() -> A<'static> {     // This is now fixed
    let string = "I'm a little string".to_string();
    A { s: &string }
}
```

This now indicates that the function should return a struct with ``static`` lifetime. If we wanted to have any other lifetime, we could tie it to the lifetime of the function itself:

```rust
fn makes_a<'a>() -> A<'a> {//...//}
```

But we should note that now the ``A`` struct has a lifetime limited to the *scope* of the function. So as soon as ``makes_a`` goes out of scope, the struct will get dropped!

```rust
fn main() {
    let a;
    {
        let a_ref = makes_a();  // Lifetime of A is tied to this inner scope
        a = a_ref;
        println!("{:?}", a_ref);
    }  // a_ref and A go out of scope here
    // println!("{:?}", a);  // This would cause an error because a's lifetime ended in the previous scope
}
```
