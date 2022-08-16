use markov::markov::{MarkovStates, NGram};
use rand::thread_rng;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Something went wrong while trying to open the input file.");

    let rng = thread_rng();

    let output = MarkovStates::<NGram<3>, 3, _>::from_random(input.as_bytes(), rng)
        .into_iter()
        .take(1000)
        .map(|byte| byte as char)
        .collect::<String>();

    println!("{}", output);
}
