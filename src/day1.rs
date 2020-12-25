use adventofcode::Parameters;
use std::fmt;
use std::fs;

struct Pair {
    a: i32,
    b: i32,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

fn find_pair_summing_to(numbers: &Vec<i32>, sum: i32) -> Option<Pair> {
    let mut pair: Option<Pair> = None;
    for a in numbers.iter() {
        for b in numbers.iter() {
            if a + b == sum {
                pair = Some(Pair {
                    a: a.clone(),
                    b: b.clone(),
                });
            }
        }
    }
    return pair;
}

pub fn run(params: Parameters) {
    let numbers: Vec<i32> = fs::read_to_string(
        params
            .args
            .first()
            .map(|filename| filename.to_string())
            .expect("Something went wrong reading the file"),
    )
    .expect("Something went wrong reading the file")
    .split_ascii_whitespace()
    .map(|line| line.trim().parse().expect("Didn't get a number."))
    .collect();

    // TODO: take in goal(s)
    // TODO: solve
    let goal: i32 = params
        .args
        .get(1)
        .expect("No sum goal given.")
        .parse()
        .expect("Non-numeric goal given.");

    let pair =
        find_pair_summing_to(numbers.as_ref(), goal).expect("Pair summing to goal does not exist");
    println!("the pair: {}", pair);

    let mult = pair.a * pair.b;

    println!("their multiplication: {}", mult);
}
