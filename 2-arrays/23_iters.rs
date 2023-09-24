use std::iter::zip;
fn main(){
    let a = vec![0, 1, 2];
    let b = vec![4, 5, 6];
    // for (i, j) in a.iter().zip(b.iter()){
    //     println!("{}, {}", i, j)
    // }

    for (i, j) in zip(a, b){
        println!("{}, {}", i, j);
    }
}