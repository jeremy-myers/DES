use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn read_1d_vector(filename: &str) -> Vec<f64> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let vector: Vec<f64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .collect();

    vector
}
