// Practice script to sum the powers of an array
// rho = a1 * r^1 + a2 * r^2...

fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| start + (end - start) * (i as f64) / ((n - 1) as f64))
        .collect()
}

fn practice_map(n: usize) -> Vec<usize>{
    // Practice function where we create an 'iterator' of a certain size
    // and return a vector of integers where the 'even' values are doubled
    (0..n)
        .map(|i| if i % 2 == 0 {2*i} else {i}).collect()
}

fn get_scale_coef(n: i32, m: i32) {
    let div : f32 = (n as f32 - m as f32) / 2.0;
    let max_idx = (div + 1.0) as i32;
    println!("Max idx = {}", max_idx)

}

fn sum_along_direction(arr: &Vec<Vec<f64>>) -> Vec<f64>{
    let shape = arr[0].len();
    let mut result = vec![0.0; shape];

    // Sum over the arrays
    for each_vec in arr{
        println!("{:?}", each_vec);
        for (i, &val) in each_vec.iter().enumerate(){
            result[i] += val;
        }
    }
    result
}

// Function to get
// for j in range(int((n - m) / 2) + 1):
// coef = ((-1) ** j * fact(n - j)) / (fact(j) * fact((n + m) / 2 - j) * fact((n - m) / 2 - j))

fn main(){
    let rho : Vec<f64> = linspace(0.0, 1.0, 10);
    println!("{:?}", rho);

    println!("{:?}", practice_map(10));

    get_scale_coef(1, 1);

    let vec2d: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let res = sum_along_direction(&vec2d);
    println!("{:?}", res);

}