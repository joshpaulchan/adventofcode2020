use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    print!("Running: {} ", config.problem);
    println!("with input: {}", config.input);

    let contents = fs::read_to_string(config.input).expect("Something went wrong reading the file");

    println!("{}", contents);
}

struct Parameters {
    problem: String,
    input: String,
}

fn parse_config(args: &[String]) -> Parameters {
    let problem = args[1].clone();
    let input = args[2].clone();

    Parameters { problem, input }
}
