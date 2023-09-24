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

## Another example

Another interesting example is shown below for a simple struct. Because the ``Book`` struct holds *references* as string literal ``&str``, we need to define the lifetime.

```rust
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

fn main() {
    let title = String::from("Rust Book");
    let author = String::from("Rustacean");
    let book = Book {
        title: &title,
        author: &author,
    };
}

```

```rust
trait Printer {
    fn print(&self);
}

struct Document<'a> {
    content: &'a str,
}

impl<'a> Printer for Document<'a> {
    fn print(&self) {
        println!("{}", self.content);
    }
}

```

Here, the Document struct holds a reference to a string slice, and the Printer trait defines a method that operates on this reference



#### Some other things to learn

```rust
fn do_something() -> Result<MyResultType, Error> {
    let result = some_operation()?;
    Ok(result)
}
```