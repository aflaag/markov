use markov::markov::{MarkovStates, NGram};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Something went wrong while trying to open the input file.");

    let output = MarkovStates::<NGram<2>, 2>::from(input.as_bytes())
        .into_iter()
        .take(100)
        .map(|byte| byte as char)
        .collect::<String>();

    println!("{}", output);
}
