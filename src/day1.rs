use adventofcode::Parameters;
use std::fs;

pub fn run(params: Parameters) {
    let contents = fs::read_to_string(
        params
            .args
            .first()
            .map(|filename| filename.to_string())
            .expect("There was no filename given."),
    )
    .expect("Something went wrong reading the file");
    // println!("{}", contents);

    let numbers: Vec<i32> = contents
        .split_ascii_whitespace()
        .map(|line| line.trim().parse().expect("Didn't get a number."))
        .collect();

    let sum: i32 = numbers.iter().sum();

    println!("their sum: {}", sum);
}
