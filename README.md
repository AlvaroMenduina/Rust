# Rust - Learning notes

This is a collection of notes about the ``rust`` language, especially focusing on a comparison with Python and notable differences.
For details on how to install and run ``rust`` see [here](guide.md)

# Rust vs Python

Apart from the obvious differences between Rust and Python, there are some interesting variations on syntax or the approach to 
Object Oriented Programming (OOP). Here we explore those differences with some examples:

### Print format

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

### Slicing does not make 'copies'
Slicing in Rust uses *pointers* to the array like

```rust
let arr = [10, 20, 30, 40, 50, 60]; 
let slice1 = &arr[0..1];
```

but slices do **not** make a copy of the data in the array, they all *borrow* the data from the declared arrays.
The size of the arrays will be known at *compile* time, while the size of the slice will be known at *runtime*.

### [1.X] ``HashMap`` and ``dict``

The ``HashMap`` in Rust is the equivalent of a Python dictionary ``dict``, a 'list' of things that are indexed via a *key* rather than an index. However, Rust has some specific behaviours that Python lacks:

**Dictionaries must contain the same type**. This is unusual; Python can hold *anything* in a dictionary like this, with no regard to the 
type each key is holding.

```python
my_dict = {}
my_dict["name"] = "Alvaro"
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
