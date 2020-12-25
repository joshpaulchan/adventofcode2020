use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = adventofcode::parse_config(&args).expect("Parameters should be given.");
    println!("Running: {} with args: {:?}", config.problem, config.args);

    match config.problem.as_ref() {
        "1" => crate::day1::run(config),
        _ => adventofcode::dump_file(config),
    }
}
