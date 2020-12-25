use adventofcode::Parameters;
use std::fs;

pub fn run(params: Parameters) {
    let numbers: Vec<i32> = fs::read_to_string(
        params
            .args
            .first()
            .map(|filename| filename.to_string())
            .expect("Something went wrong reading the file"),
    )
    .expect("Something went wrong reading the file")
    .split_ascii_whitespace()
    .map(|line| line.trim().parse().expect("Didn't get a number."))
    .collect();

    // TODO: take in goal(s)
    // TODO: solve

    let sum: i32 = numbers.iter().sum();

    println!("their sum: {}", sum);
}
