// OOP

struct Person {
    first_name: String,
    last_name : String
}

// "Composition" through the 'impl' block
impl Person {

    fn new(their_name: &str, their_surname: &str) -> Person {

        // Because the "new" method must return an instance of the Person class
        // we need to make a last expression (the declaration __init__) for the return
        Person {
            first_name: their_name.to_string(),
            last_name: their_surname.to_string()
        }
    }

    // This is a STATIC method of the class person
    fn say_hi() {
        println!("\nHi, I'm a person");
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn make_copy(&self) -> Person {
        Person {
            first_name: self.first_name.to_string(),
            last_name: self.last_name.to_string()
        }
    }
}

fn main(){

    // Create an instance of person directly
    let a_person = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string(),
    };

    println!("\nThis person is: {} {}", a_person.first_name, a_person.last_name);

    // ------------------------------------------
    // Create an instance of person with the "new" method
    let new_person = Person::new("Jack", "Jonhson");
    println!("\nThis person is: {} {}", new_person.first_name, new_person.last_name);

    // ------------------------------------------
    // Using STATIC methods
    // [BUG] 
    // new_person.say_hi();        // This would work in Python but not in Rust
    Person::say_hi();           // In Rust, STATIC method cannot be called from an INSTANCE!

    // A method should have a reference to &self at least, in which case it would not be a static method, but we could call it from an instance
    // fn say_hi(&self) and --- new_person.say_hi()

    // ------------------------------------------
    println!("\nThe fullname {}", new_person.full_name());

    let a_clone = new_person.make_copy();
    println!("\nThis person is: {} {}", a_clone.first_name, a_clone.last_name);

}