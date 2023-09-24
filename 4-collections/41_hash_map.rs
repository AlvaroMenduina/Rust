// Practising with collections

use std::collections::HashMap;
use std::iter::zip;

fn main(){

    // HashMaps can be created with the .insert method
    let mut flowers = HashMap::new();
    flowers.insert(String::from("Rose"), 10);
    flowers.insert(String::from("Tulip"), 30);

    // But also with the .collect method
    let teams = vec![String::from("Blue"), String::from("Yellow")];         // Create a vector
    let initial_scores = vec![10, 50];
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let mut scores: HashMap<_, _> = zip(teams, initial_scores).collect();

    // Check if there is an entry, if not, insert
    scores.entry(String::from("Blue")).or_insert(40);
    scores.entry(String::from("Green")).or_insert(40);

    println!("HashMap: {:?}", scores["Blue"]);
    println!("HashMap: {:?}", scores["Green"]);

    // Update a value based on the old one
    let text = "hello world wonderful world";
    let mut text_count = HashMap::new();

    for word in text.split_whitespace(){
        println!("\n{}", word);
        println!("Before: {:?}", text_count);
        let count = text_count.entry(word).or_insert(0);
        // This will add the word to the entry if it doesn't exist.
        *count +=1;         // This is referencing the "key"
        println!("After: {:?}", text_count);
    }

    println!("{:?}", text_count);
}