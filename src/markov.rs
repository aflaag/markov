use std::collections::{HashMap, HashSet};
use std::hash::Hash;
// use rand::Rng;

pub trait State: Hash + Eq + IntoIterator {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NGram<const N: usize> {
    ngram: [u8; N],
}

impl<const N: usize> State for NGram<N> {}

impl<const N: usize> IntoIterator for NGram<N> {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.ngram.into_iter()
    }
}

impl<const N: usize> ToString for NGram<N> {
    fn to_string(&self) -> String {
        String::from_utf8(self.ngram.to_vec()).unwrap()
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

pub struct MarkovIter<S: State, const N: usize> {
    prev_state: Option<S>,
    states: HashMap<S, HashSet<u8>>,
}

impl<S: State, const N: usize> MarkovIter<S, N> {
    pub fn new(first_state: Option<S>, states: HashMap<S, HashSet<u8>>) -> Self {
        Self { prev_state: first_state, states }
    }
}

impl<S, const N: usize> Iterator for MarkovIter<S, N>
where
    S: State + Clone + Copy + From<[u8; N]>,
    <S as IntoIterator>::Item: Into<u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prev) = self.prev_state {
            let next_states = self.states.get(&prev).unwrap();

            let next_char = *next_states.iter().take(1).next().unwrap();
            // println!("{:?} {}", next_states, next_char);

            let mut new_prev_state = [0; N];

            new_prev_state
                .iter_mut()
                .zip(prev.into_iter().skip(1))
                .for_each(|(new_c, old_c)| *new_c = old_c.into());

            new_prev_state[N - 1] = next_char;

            self.prev_state = Some(new_prev_state.into());
            
            Some(next_char)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct MarkovStates<S: State, const N: usize>  {
    states: HashMap<S, HashSet<u8>>,
}

impl<S, const N: usize> IntoIterator for MarkovStates<S, N>
where
    S: State + Clone + Copy + From<[u8; N]>,
    <S as IntoIterator>::Item: Into<u8>,
{
    type Item = u8;
    type IntoIter = MarkovIter<S, N>;

    fn into_iter(self) -> MarkovIter<S, N> {
        MarkovIter::new(self.states.clone().into_keys().take(1).next(), self.states)
    }
}

impl<S: State, const N: usize> Default for MarkovStates<S, N> {
    fn default() -> Self {
        Self { states: HashMap::new() }
    }
}

impl<S: std::fmt::Debug+ State + From<[u8; N]>, const N: usize> From<&[u8]> for MarkovStates<S, N> {
    fn from(chars: &[u8]) -> Self {
        let mut states: HashMap<S, HashSet<u8>> = HashMap::new();

        chars
            .windows(N + 1)
            .for_each(|slice| {
                let mut curr = [0; N];

                let mut bytes = slice.iter();

                curr
                    .iter_mut()
                    .for_each(|byte| *byte = *bytes.next().unwrap());

                let curr_state = S::from(curr);
                let next_char = slice[N];

                if let Some(next_chars) = states.get_mut(&curr_state) {
                    next_chars.insert(next_char);
                } else {
                    states.insert(curr_state, HashSet::from([next_char]));
                }
            });

        Self { states }
    }
}
