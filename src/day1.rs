use adventofcode::Parameters;
use std::fmt;
use std::fs;

trait Product {
    fn product(&self) -> i32;
}

struct Pair {
    a: i32,
    b: i32,
}

impl Product for Pair {
    fn product(&self) -> i32 {
        self.a * self.b
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

fn find_pair_summing_to(numbers: &Vec<i32>, sum: i32) -> Option<Box<dyn Product>> {
    for a in numbers.iter() {
        for b in numbers.iter() {
            if a + b == sum {
                return Some(Box::new(Pair {
                    a: a.clone(),
                    b: b.clone(),
                }));
            }
        }
    }
    return None;
}

struct Triple {
    a: i32,
    b: i32,
    c: i32,
}

impl Product for Triple {
    fn product(&self) -> i32 {
        self.a * self.b * self.c
    }
}

impl std::fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

fn find_triple_summing_to(numbers: &Vec<i32>, sum: i32) -> Option<Box<dyn Product>> {
    for a in numbers.iter() {
        for b in numbers.iter() {
            for c in numbers.iter() {
                if a + b + c == sum {
                    return Some(Box::new(Triple {
                        a: a.clone(),
                        b: b.clone(),
                        c: c.clone(),
                    }));
                }
            }
        }
    }
    return None;
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

    let coll =
        find_pair_summing_to(numbers.as_ref(), goal).expect("Pair summing to goal does not exist");

    let mult: i32 = coll.product();

    println!("their multiplication: {}", mult);

    let coll = find_triple_summing_to(numbers.as_ref(), goal)
        .expect("Pair summing to goal does not exist");

    let mult: i32 = coll.product();

    println!("their multiplication: {}", mult);
}
