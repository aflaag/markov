use markov::markov::{MarkovStates, NGram};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Something went wrong while trying to open the input file.");

    let output = MarkovStates::<NGram>::from(input)
        .iter()
        .unwrap()
        .take(10)
        .collect::<String>();

    println!("{}", output);
}
