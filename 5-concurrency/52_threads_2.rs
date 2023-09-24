// More threads
use std::thread;
use std::iter::FromIterator;

fn main(){
    let num = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = num.len();
        let sum = num.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}