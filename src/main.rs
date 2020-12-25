use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = adventofcode::parse_config(&args).expect("Parameters should be given.");
    print!("Running: {} with args: {:?}", config.problem, config.args);

    match config.problem.as_ref() {
        "1" => println!("Day 1."),
        _ => adventofcode::dump_file(config),
    }
}
