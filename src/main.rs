use rand::prelude::*;
use std::time::Instant;

const TRAIN_DURR: u64 = 100000;
const TEST_DURR: u64 = 100000;
const LEARN_RATE: f64 = 0.01;

fn main() {
    //data
    let mut cell = [0.0; 100];
    //random number
    let mut rng = rand::thread_rng();
    
    //training
    let train = Instant::now();
    for _ in 0..TRAIN_DURR {
        //random number [1,10]
        let x1: f64 = rng.gen();
        let x2: f64 = rng.gen();
        let x1 = x1 * 10.0;
        let x2 = x2 * 10.0;
        //compute training datapoint
        let val = x1 * x2;
        //locate cell
        let index = (x1 as usize) + (x2 as usize) * 10;
        //update cell
        cell[index] += (val - cell[index]) * LEARN_RATE;
    }
    println!("training: {:?}", train.elapsed());
    
    //testing
    let test = Instant::now();
    let mut total_err = 0.0;
    for _ in 0..TEST_DURR {
        //random number [1,10]
        let x1: f64 = rng.gen();
        let x2: f64 = rng.gen();
        let x1 = x1 * 10.0;
        let x2 = x2 * 10.0;
        //compute training datapoint
        let val = x1 * x2;
        //locate cell
        let index = (x1 as usize) + (x2 as usize) * 10;
        //compute error
        total_err += (val - cell[index]).abs();
    }
    println!("testing: {:?}", test.elapsed());
    let avg_err = total_err / (TEST_DURR as f64);
    println!("error: {avg_err}");
}
