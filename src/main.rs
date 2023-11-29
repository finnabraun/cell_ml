use rand::prelude::*;

fn main() {
    //data
    let mut cells = [0.0; 100];
    //random number
    let mut rng = rand::thread_rng();
    
    for _ in 0..10000 {
        //random number [1,10]
        let x1: f64 = rng.gen();
        let x2: f64 = rng.gen();
        let x1 = x1 * 10.0;
        let x2 = x2 * 10.0;
        //compute training datapoint
        let val = x1 * x2;
        //locate cell
        let index = (x1 as u64) + (x2 as u64) * 10;
    }

}
