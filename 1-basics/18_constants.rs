// loading some constants

use std::f64::consts;

fn main() {
    let x = 2.0 * consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();     

    assert!(abs_difference < 1e-10);

    // Somehow x has a built-in method to get cosine??
    // So do f32

    let thetat = 3.14 as f32;       // if you do 'let thetat = 3.14' it will throw "can't call method `cos` on ambiguous numeric type `{float}`"
    let cos_x = thetat.cos();
    println!("\nResult {}", cos_x)
}