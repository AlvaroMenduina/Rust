// Generic types

struct Person<T> {
    age: T,
    height: T
}

struct Number<T> (T);

// This is a custom implementation of the TRAIT ViewStat
trait ViewStat {
    fn print_stats(&self){
        // Funnily enough, it needs Self, but it does not belong to a class ViewStat??
        println!("Not implemented")
    }
}

// This is saying: "the implementation of the trait ViewStat for the struct Person, only for types T that implement the 'Display' method"
impl<T: std::fmt::Display> ViewStat for Person<T> {
    // This is saying, when the Type T HAS an implementation of std::fmt::Display,
    // the traits ViewStat is implemented as follows: by allowing print_stats
    fn print_stats(&self){
        println!("Age: {} | Height: {}", self.age, self.height)
    }
}

impl<T> ViewStat for Person<Number<T>> {
    // the IMPLEMENTATION of a trait for this type (when person uses a Number struct)
    // this is just saying "Implement the trait ViewStat in this case"
}

// impl<T: std::fmt::Display> Person<T> {
//     // Surprisingly, we need to add impl<T: std::fmt::Display>`
//     // [NOTE]: I don't really understand what this does... 
//     // Something about constraining...


    // fn print_stats(&self){
    //     println!("Age: {} | Height: {}", self.age, self.height)
    // }
// }

fn main(){

    let p = Person{age:30, height:180};
    println!("Age: {} | Height: {}", p.age, p.height);

    p.print_stats();

    // --------------
    let pf = Person{age:29.0, height:175.8};
    pf.print_stats();

    let pp = Person{age:Number(25), height:Number(99)};
    pp.print_stats();
}