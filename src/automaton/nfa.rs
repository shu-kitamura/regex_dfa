use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::parser::Node;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct NfaState(pub u32);

pub struct NfaContext {
    state_count: u32,
}

impl NfaContext {
    fn new() -> Self {
        NfaContext { state_count: 0 }
    }

    pub fn new_state(&mut self) -> NfaState {
        let id = self.state_count;
        self.state_count += 1;
        NfaState(id)
    }
}

pub struct Nfa {
    pub start: NfaState,
    pub accepts: HashSet<NfaState>,
    transitions: HashMap<NfaState, HashMap<Option<char>, HashSet<NfaState>>>,
}

impl Nfa {
    pub fn new(start: NfaState, accepts: HashSet<NfaState>) -> Self {
        Nfa {
            start,
            accepts,
            transitions: HashMap::new(),
        }
    }

    pub fn from_node(node: Node) -> Self {
        node.assemble(&mut NfaContext::new())
    }

    pub fn next_chars(&self, state: NfaState) -> HashSet<Option<char>> {
        self.transitions
            .get(&state)
            .map(|table| table.keys().cloned().collect())
            .unwrap_or(HashSet::new())
    }

    pub fn next_states(&self, state: NfaState, char: Option<char>) -> HashSet<NfaState> {
        self.transitions
            .get(&state)
            .and_then(|table| table.get(&char))
            .cloned()
            .unwrap_or(HashSet::new())
    }

    pub fn add_transition(mut self, from: NfaState, char: char, to: NfaState) -> Self {
        self._insert_transition(from, to, Some(char));
        self
    }

    pub fn add_empty_transition(mut self, from: NfaState, to: NfaState) -> Self {
        self._insert_transition(from, to, None);
        self
    }

    pub fn merge_transition(mut self, other: &Self) -> Self {
        for (from_state, trans) in &other.transitions {
            for (char, to_states) in trans {
                self.transitions
                    .entry(*from_state)
                    .or_insert(HashMap::new())
                    .entry(*char)
                    .or_insert(HashSet::new())
                    .extend(to_states);
            }
        }
        self
    }

    fn _insert_transition(&mut self, from: NfaState, to: NfaState, char: Option<char>) {
        let to_states = self
            .transitions
            .entry(from)
            .or_insert(HashMap::new())
            .entry(char)
            .or_insert(HashSet::new());
        to_states.insert(to);
    }
}