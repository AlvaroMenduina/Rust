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
        // self.n_lim = self.n_zern + 1;
        let y = (1 + 8*self.n_zern) as f32;
        // println!("y={}", y);
        let sqrt_x = y.sqrt();
        // println!("sq_y={}", sqrt_x);
        let ceil_x : f32 = 0.5 * (sqrt_x - 3.0);
        // println!("ceil={}", ceil_x);
        self.n_lim = ceil_x.ceil() as i32;
        println!("n_lim={}", self.n_lim);
        // n = int(np.ceil(0.5 * (np.sqrt(1 + 8*N) - 3)))
    }

    fn z_nm(&self, mode: &str)-> f64{

        let r: f64 = match mode {
            "Standard" => self.r_nm(),
            "Jacobi" => self.r_nm_jacobi(),
            _ => {
                println!("Unknown mode: {}", mode);
                0.0
            }
            
        };
        r
    }

    fn r_nm(&self)-> f64{
        let r = 0.0;
        r
    }

    fn r_nm_jacobi(&self)-> f64{
        let r = 0.0;
        r
    }
}

fn main(){

    let mut zern = Zernike::new();

    // some array of coefficients of a given shape
    let coef = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    zern.get_n_zern(&coef);
    zern.get_limit_index();

    println!("Len: {}", zern.n_zern);
    println!("N: {}", zern.n_lim);

}