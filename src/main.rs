use rand::prelude::*;
use std::time::Instant;

const TRAIN_DURR: u64 = 100000;
const TEST_DURR: u64 = 100000;
const LEARN_RATE: f64 = 0.01;
const RESOLUTION: usize = 100;

fn main() {
    //data
    let mut cell = [0.0; RESOLUTION.pow(2)];
    //random number
    let mut rng = rand::thread_rng();
    
    //training
    let train = Instant::now();
    for _ in 0..TRAIN_DURR {
        //random number [1,10]
        let x1: f64 = rng.gen();
        let x2: f64 = rng.gen();
        let val = x1 * x2;
        //locate cell
        let x1_cell = (x1 * RESOLUTION as f64) as usize;
        let x2_cell = (x2 * RESOLUTION as f64) as usize;
        let index = x1_cell + x2_cell * RESOLUTION;
        //update cell
        cell[index] += (val - cell[index]) * LEARN_RATE;
    }
    println!("training: {:?}", train.elapsed());
    
    //testing
    let test = Instant::now();
    let mut total_err = 0.0;
    let mut max_err = 0.0;
    for _ in 0..TEST_DURR {
        //random number [1,10]
        let x1: f64 = rng.gen();
        let x2: f64 = rng.gen();
        let val = x1 * x2;
        //locate cell
        let x1_cell = (x1 * RESOLUTION as f64) as usize;
        let x2_cell = (x2 * RESOLUTION as f64) as usize;
        let index = x1_cell + x2_cell * RESOLUTION;
        //compute error
        let error = val - cell[index];
        total_err += (error).abs();
        if error > max_err {
            max_err = error;
        }

    }
    println!("testing: {:?}", test.elapsed());
    let avg_err = total_err / (TEST_DURR as f64);
    println!("error: {avg_err}");
    println!("max error: {max_err}");
}
