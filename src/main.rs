use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    print!("Running: {} with args: {}", config.problem, config.args);

    let contents = fs::read_to_string(config.args).expect("Something went wrong reading the file");

    println!("{}", contents);
}

struct Parameters {
    problem: String,
    args: String,
}

fn parse_config(args: &[String]) -> Parameters {
    let problem = args[1].clone();
    let args = args[2].clone();

    Parameters { problem, args }
}
