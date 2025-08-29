use std::env;

use des::read_1d_vector;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let v = read_1d_vector(filename);

    let mut index: i64 = 0;
    let mut sum: f64 = 0.0;
    let mut mean: f64 = 0.0;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut diff;

    for data in v {
        index += 1;
        diff = data - mean;
        sum += diff * diff * ((index as f64) - 1.0) / (index as f64);
        mean += diff / (index as f64);
        if data > max {
            max = data;
        } else if data < min {
            min = data;
        }
    }

    if index > 0 {
        let stdev = (sum / (index as f64)).sqrt();
        println!("\nfor a sample of size {index}");
        println!("mean ................  = {mean:.6}");
        println!("standard deviation ... = {stdev:.6}");
        println!("minimum .............. = {min:.6}");
        println!("maximum .............. = {max:.6}");
    }
}
