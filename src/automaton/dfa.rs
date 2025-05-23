use std::collections::{HashMap, HashSet};

use super::{Nfa, NfaState};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct DfaState(u32);


struct DfaContext {
    state_count: u32,
    state_map: HashMap<Vec<NfaState>, DfaState>,
}

impl DfaContext {
    fn new() -> DfaContext {
        DfaContext {
            state_count: 0,
            state_map: HashMap::new(),
        }
    }

    fn get_state(&mut self, states: &[NfaState]) -> DfaState {
        let mut sorted_states = states.to_vec();
        sorted_states.sort();
        match self.state_map.get(&sorted_states) {
            Some(state) => *state,
            None => {
                let id = self.state_count;
                self.state_count += 1;
                self.state_map.insert(sorted_states, DfaState(id));
                DfaState(id)
            }
        }
    }
}

pub struct Dfa {
    pub start: DfaState,
    pub accepts: HashSet<DfaState>,
    transitions: HashMap<(DfaState, char), DfaState>,
}

impl Dfa {
    pub fn next_state(&self, state: DfaState, char: char) -> Option<DfaState> {
        self.transitions.get(&(state, char)).cloned()
    }
    
    pub fn from_nfa(nfa: Nfa) -> Self {
        let mut context = DfaContext::new();

        // start: DFAの開始状態 (DFAState)
        // start_states: NFAとしての開始状態集合 (Vec<NFAState>)
        let (start, start_states) = {
            let mut ret = vec![nfa.start];
            let mut stack = nfa
                .next_states(nfa.start, None)
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            while let Some(state) = stack.pop() {
                ret.push(state);
                let next = nfa.next_states(state, None);
                stack.extend(next.iter().filter(|s| !ret.contains(s)).cloned());
            }
            (context.get_state(&ret), ret)
        };

        // 遷移テーブル
        let transitions = {
            let mut ret = HashMap::<(DfaState, char), DfaState>::new();
            let mut waiting = vec![start_states];
            let mut visited = HashSet::<DfaState>::new();
            while let Some(look_states) = waiting.pop() {
                visited.insert(context.get_state(&look_states));

                // Collect states that can be transitioned from the current state (look_states).
                // transition_map[char] = The set of states that can be transitioned by `char`.
                let mut transition_map = HashMap::<char, HashSet<NfaState>>::new();
                for look_state in &look_states {
                    for char in nfa
                        .next_chars(*look_state)
                        .iter()
                        .filter_map(|c| c.is_some().then(|| c.unwrap()))
                    {
                        let mut next_states = nfa
                            .next_states(*look_state, Some(char))
                            .into_iter()
                            .chain(nfa.next_states(*look_state, None))
                            .collect::<Vec<_>>();
                        let mut stack = next_states
                            .iter()
                            .filter(|s| !nfa.next_states(**s, None).is_empty())
                            .cloned()
                            .collect::<Vec<_>>();
                        while let Some(state) = stack.pop() {
                            let next = nfa.next_states(state, None);
                            stack.extend(next.iter().filter(|s| !next_states.contains(s)).cloned());
                            next_states.extend(next);
                        }
                        transition_map
                            .entry(char)
                            .or_insert(HashSet::new())
                            .extend(next_states);
                    }
                }

                let form_state = context.get_state(&look_states);
                for (char, next_states) in transition_map {
                    let next_states_vec: Vec<_> = next_states.iter().cloned().collect();
                    let to_state = context.get_state(&next_states_vec);
                    if !visited.contains(&to_state) {
                        waiting.push(next_states.into_iter().collect());
                    }
                    ret.insert((form_state, char), to_state);
                }
            }
            ret
        };

        // 受理状態 (HashSet<DFAState>)
        let accepts = {
            let mut ret = HashSet::<DfaState>::new();
            for (nfa_states, dfa_state) in context.state_map {
                if nfa_states.iter().any(|s| nfa.accepts.contains(s)) {
                    ret.insert(dfa_state);
                }
            }
            ret
        };

        Dfa {
            start,
            accepts,
            transitions,
        }
    }
}