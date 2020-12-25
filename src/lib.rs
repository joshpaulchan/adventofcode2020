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
