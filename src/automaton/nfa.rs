use std::collections::HashSet;

use crate::parser::Ast;

pub type NfaState = usize;

pub struct Nfa {
    start: NfaState,
    accept: HashSet<NfaState>,
    transitions: HashSet<(NfaState, Option<char>, NfaState)>,
}

impl Nfa {
    pub fn new(state: NfaState) -> Self {
        Nfa {
            start: state,
            accept: HashSet::new(),
            transitions: HashSet::new(),
        }
    }

    pub fn add_accept(&mut self, state: NfaState) {
        self.accept.insert(state);
    }

    pub fn add_transition(&mut self, from: NfaState, to: NfaState, char: char) {
        self.transitions.insert((from, Some(char), to));
    }

    pub fn add_epsilon_transition(&mut self, from: NfaState, to: NfaState) {
        self.transitions.insert((from, None, to));
    }

    pub fn from_ast(ast: &Ast, state: &mut NfaState) {
        let mut nfa = Nfa::new(*state);

        match ast {
            Ast::Char(c) => {
                let start = new_state(state);
                let accept = new_state(state);
                nfa.add_accept(accept);
                nfa.add_transition(start, accept, *c);
                // Ok(nfa)
            }
            _ => {}
        }
    }
}

fn new_state(states: &mut NfaState) -> NfaState {
    *states += 1;
    *states - 1
}

// --- tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let mut state = 0;
        assert_eq!(new_state(&mut state), 0);
        assert_eq!(state, 1);
        assert_eq!(new_state(&mut state), 1);
        assert_eq!(state, 2);
    }
}
