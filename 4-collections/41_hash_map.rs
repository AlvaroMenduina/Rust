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
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let a = vec![0, 1, 2];
    let b = vec![4, 5, 6];
    // for (i, j) in a.iter().zip(b.iter()){
    //     println!("{}, {}", i, j)
    // }

    for (i, j) in zip(a, b){
        println!("{}, {}", i, j);
    }
}