use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn read_1d_array(filename: &str) -> Vec<f64> {
    let f = BufReader::new(File::open(filename).unwrap());
    let arr: Vec<f64> = f
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .collect();

    arr
}

pub fn read_2d_array(filename: &str) -> Vec<Vec<f64>> {
    let f = BufReader::new(File::open(filename).unwrap());
    let arr: Vec<Vec<f64>> = f
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .split(char::is_whitespace)
                .map(|number| number.parse::<f64>().unwrap())
                .collect()
        })
        .collect();
    arr
}
