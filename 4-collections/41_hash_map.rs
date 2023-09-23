// Practising with collections

use std::collections::HashMap;

fn main(){
    let mut flowers = HashMap::new();

    flowers.insert(String::from("Rose"), 10);
    flowers.insert(String::from("Tulip"), 30.0);
}