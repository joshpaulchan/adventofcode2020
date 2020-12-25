use std::env;
use std::fs;

use adventofcode2020::Parameters;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = adventofcode2020::parse_config(&args).expect("Parameters should be given.");

    print!("Running: {} with args: {:?}", config.problem, config.args);

    let contents = fs::read_to_string(
        config
            .args
            .first()
            .map(|filename| filename.to_string())
            .expect("There was no filename given."),
    )
    .expect("Something went wrong reading the file");

    println!("{}", contents);
}
