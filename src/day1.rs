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
    println!("{}", contents);
}
