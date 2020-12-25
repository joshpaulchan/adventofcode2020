use std::fs;

pub struct Parameters {
    pub problem: String,
    pub args: Vec<String>,
}

pub fn parse_config(args: &[String]) -> Option<Parameters> {
    Some(Parameters {
        problem: args[1].clone(),
        args: args[2..].to_vec(),
    })
}

pub fn dump_file(params: Parameters) {
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
