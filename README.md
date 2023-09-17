## Rust - Learning notes

# Installation

Here is the ``rustup`` notes (https://rust-lang.github.io/rustup/)

You can check the current version of rust by running this in the terminal
```
rustc --version
```

## Running code

Code is compiled with
```
rustc hello.rs
```

and runs with

```rust
./hello
```

## Macros

Some cool stuff about macros here (https://www.programiz.com/rust/macro)

## Vs Python

Some things are slightly different in Rust than in Python

### Print format

In Rust, we use it like this: 

```rust
fn main(){
    let answer = 42;
    println!("Hello {}", answer);
    // The format is relatively similar to Python where you'd do:
    //      
}
```

Compared to the Python equivalent

```python
print(f"Hello {answer}") or print("Hellow %s" % (answer))

```

### One-line declarations
In Python we use one-line declarations like this x = *value* if *condition* else *other_value*

```python
x = 0 if i % 2 == 0 else 1
```

whereas in Rust, the format is x = if *condition* *value* else *other_value*

```rust
let even_or_odd = if i % 2 == 0 {"even"} else {"odd"};
```

### Cannot know the type
Apparently you cannot print the *type* of a variable in Rust at runtime, (somehow it's not available). A quick explanation here: (https://www.hackertouch.com/how-to-print-type-of-variable-in-rust.html)

### No 'return' at the end of Functions
In Rust you can get away with not adding a *return* statement at the end of the function, by default, it will return the value of the last expression

### Taking 'string' as arguments to Functions
Cool guide (http://xion.io/post/code/rust-string-args.html)