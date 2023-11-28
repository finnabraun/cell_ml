//Global Model
// 8x8 grid
let mut cells[f64; 64] = [0; 64];

fn main() {

}

fn evaluate(coords: (f64, f64)) -> f64 {
    let x_cell = coords.0 as u32;
    let y_cell = coords.1.as u32;
    let index: u32 = x_cell + 
