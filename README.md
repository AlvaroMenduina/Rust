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

Whereas in Rust, this is incorrect. To access the elements of a tuple, we have to use ``tup.0`` as if it was a *property* of the tuple

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
