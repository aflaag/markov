use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::Read;
use rand::Rng;

pub trait State: Hash + Eq {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NGram<const N: usize> {
    ngram: [u8; N],
}

impl<const N: usize> State for NGram<N> {}

impl<const N: usize> ToString for NGram<N> {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl<const N: usize> FromIterator<NGram<N>> for String {
    fn from_iter<T: IntoIterator<Item = NGram<N>>>(iter: T) -> Self {
        iter.into_iter().collect::<String>()
    }
}

impl<const N: usize> From<[u8; N]> for NGram<N> {
    fn from(ngram: [u8; N]) -> Self {
        Self { ngram }
    }
}

pub struct MarkovIter<S: State> {
    prev_state: Option<S>,
}

impl<S: State> MarkovIter<S> {
    pub fn new(first_state: Option<S>) -> Self {
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
pub struct MarkovStates<S: State, const N: usize>  {
    states: HashMap<S, HashSet<S>>,
}

impl<S: State, const N: usize> IntoIterator for MarkovStates<S, N> {
    type Item = S;
    type IntoIter = MarkovIter<S>;

    fn into_iter(self) -> MarkovIter<S> {
        MarkovIter::new(self.states.into_keys().take(1).next())
    }
}

impl<S: State, const N: usize> Default for MarkovStates<S, N> {
    fn default() -> Self {
        Self { states: HashMap::new() }
    }
}

impl<S: std::fmt::Debug+ State + From<[u8; N]>, const N: usize> From<&[u8]> for MarkovStates<S, N> {
    fn from(chars: &[u8]) -> Self {
        let mut states: HashMap<S, HashSet<S>> = HashMap::new();

        chars
            .windows(N * 2)
            .for_each(|slice| {
                let mut curr = [0; N];
                let mut next = [0; N];

                let mut bytes = slice.iter();

                curr
                    .iter_mut()
                    .for_each(|byte| *byte = *bytes.next().unwrap());

                next
                    .iter_mut()
                    .for_each(|byte| *byte = *bytes.next().unwrap());

                let curr_state = S::from(curr);
                let next_state = S::from(next);

                if let Some(next_states) = states.get_mut(&curr_state) {
                    next_states.insert(next_state);
                } else {
                    states.insert(curr_state, HashSet::from([next_state]));
                }
            });
        println!("{:?}", states);

        Self { states }
    }
}
