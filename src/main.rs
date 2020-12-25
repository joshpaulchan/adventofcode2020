use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args).expect("Parameters should be given.");

    print!("Running: {} with args: {:?}", config.problem, config.args);

    // let contents = fs::read_to_string(config.args.get(0).as_ref())
    //     .expect("Something went wrong reading the file");

    // println!("{}", contents);
}

struct Parameters {
    problem: String,
    args: Vec<String>,
}

fn parse_config(args: &[String]) -> Option<Parameters> {
    return Some(Parameters {
        problem: args[1].clone(),
        args: args[2..].to_vec(),
    });
}
