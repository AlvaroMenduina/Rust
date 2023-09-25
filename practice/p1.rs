#![allow(dead_code, unused_variables)]
// Some practice scripts in Rust
// Trying to do the Zernike calculations

struct Zernike {
    n_zern : i32,
    n_lim: i32
}

impl Zernike {
    fn new() -> Self{
        Zernike {n_zern : 0, n_lim : 0}        // Initialize to a default value 0
    }

    fn get_n_zern(&mut self, x: &Vec<f64>) {
        // This method takes a reference to 'mutable' self, and a reference to a vector of f64
        self.n_zern = x.len() as i32;
    }

    fn get_limit_index(&mut self){
        let y = (1 + 8*self.n_zern) as f32;
        let sqrt_x = y.sqrt();
        let ceil_x : f32 = 0.5 * (sqrt_x - 3.0);
        self.n_lim = ceil_x.ceil() as i32;
        println!("n_lim={}", self.n_lim);
    }

    fn z_nm(&self, n: i32, m: i32, rho: &[f64], theta: &[f64], mode: &str)-> Vec<f64>{

        let r: Vec<f64> = match mode {
            "Standard" => self.r_nm(n, m, rho),
            // "Jacobi" => self.r_nm_jacobi(),
            _ => {
                println!("Unknown mode: {}", mode);
                zeros_like(rho)
            }
            
        };
        r
    }

    fn r_nm(&self, n: i32, m: i32, rho: &[f64])-> Vec<f64>{

        let n_abs = n.abs();
        let m_abs = m.abs();
        let mut r = zeros_like(rho);

        if (n_abs - m_abs) % 2 != 0 {
            r
        } else{
            // for j in range(int((n - m) / 2) + 1)
            let max_idx = (n_abs - m_abs) / 2 + 1;
            for j in 0..max_idx{
                let exp = n_abs - 2 * j;
                let poly: &[f64] = rho.iter().map(|&x| x.powei(exp)).collect();
                r += poly;
            }
            r
        }
        
    }

    // fn r_nm_jacobi(&self)-> f64{
    //     let r = 0.0;
    //     r
    // }
}

fn zeros_like(arr: &[f64])-> Vec<f64>{
    // A function that mimics Python np.zeros_like(x)
    vec![0.0; arr.len()]
}

fn main(){

    let mut zern = Zernike::new();

    // some array of coefficients of a given shape
    let coef = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    zern.get_n_zern(&coef);
    zern.get_limit_index();

    println!("Len: {}", zern.n_zern);
    println!("N: {}", zern.n_lim);

    let zeros = zeros_like(&coef);
    println!("Zeros: {:?}", zeros);

}