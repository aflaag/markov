use std::collections::HashMap;
use std::hash::Hash;
use rand::Rng;

pub trait State: Hash + Clone {}

#[derive(Clone, Hash)]
pub struct NGram {
    ngram: String,
}

impl State for NGram {}

impl ToString for NGram {
    fn to_string(&self) -> String {
        self.ngram.clone()
    }
}

impl FromIterator<NGram> for String {
    fn from_iter<T: IntoIterator<Item = NGram>>(iter: T) -> Self {
        iter.into_iter().collect::<String>()
    }
}

pub struct MarkovIter<S: State> {
    prev_state: S,
}

impl<S: State> MarkovIter<S> {
    pub fn new(first_state: S) -> Self {
        Self { prev_state: first_state }
    }
}

impl<S: State> Iterator for MarkovIter<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone)]
pub struct MarkovStates<S: State>  {
    states: HashMap<S, Vec<S>>,
}

impl<S: State> MarkovStates<S> {
    pub fn iter(self) -> Option<MarkovIter<S>> {
        self.states.into_keys().take(1).next().map(MarkovIter::new)
    }
}

impl<S: State> Default for MarkovStates<S> {
    fn default() -> Self {
        Self { states: HashMap::new() }
    }
}

impl<S: State> From<String> for MarkovStates<S> {
    fn from(string: String) -> Self {
        todo!()
    }
}

// #[derive(Debug))]
// pub enum MarkovError {
//     MissingStates,
// }
//
// impl std::fmt::Display for MarkovError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match *self {
//             Self::MissingStates => writeln!(f, "There are no saved states to ")
//         }
//     }
// }
