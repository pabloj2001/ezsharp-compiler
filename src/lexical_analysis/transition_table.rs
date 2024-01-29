use super::constants::{ALPHA, DIGIT};
use super::token::Token;
use super::dfa::{Dfa, get_constant_dfas};

#[derive(Debug, PartialEq, Clone)]
struct TokenStates {
    token: Token,
    states: Box<[usize]>,
}

#[derive(Debug)]
pub struct TransitionTable {
    table: Box<[Box<[Option<usize>]>]>,
    accepting: Box<[bool]>,
    alphabet: Box<[char]>,
    alphabet_indexes: Box<[usize]>,
    alphabet_start: usize,
    token_map: Box<[Option<Box<[Option<TokenStates>]>>]>,
}

impl TransitionTable {
    fn init(dfas: &Vec<Dfa>) -> TransitionTable {
        // Get number of states
        let mut state_count = 1;
        for dfa in dfas.iter() {
            state_count += dfa.dfa.len() - 1;
        }

        // Get alphabet
        let mut alpha_index = 0;
        let mut digit_index = 0;
        let mut temp_alphabet = vec![];
        for dfa in dfas.iter() {
            for c in dfa.alphabet.iter() {
                if !temp_alphabet.contains(c) {
                    if *c == ALPHA {
                        alpha_index = temp_alphabet.len();
                    } else if *c == DIGIT {
                        digit_index = temp_alphabet.len();
                    }
                    temp_alphabet.push(*c);
                }
            }
        }

        // Find smallest and largest chars in alphabet
        let mut smallest_char = '0';
        let mut largest_char = 'z';
        for c in temp_alphabet.iter() {
            if *c == ALPHA || *c == DIGIT {
                continue;
            }

            if *c < smallest_char {
                smallest_char = *c;
            } else if *c > largest_char {
                largest_char = *c;
            }
        }

        // Create alphabet indexes
        let alphabet_start = smallest_char as usize;
        let mut alphabet_indexes = vec![0; (largest_char as usize) - alphabet_start + 1].into_boxed_slice();
        for i in 0..temp_alphabet.len() {
            if temp_alphabet[i] == ALPHA || temp_alphabet[i] == DIGIT {
                continue;
            }
            alphabet_indexes[(temp_alphabet[i] as usize) - alphabet_start] = i + 1;
        }

        // Set unset alphabet and digit indexes
        for i in 0..alphabet_indexes.len() {
            if alphabet_indexes[i] == 0 {
                let char_index = i + alphabet_start;
                if char_index >= 0x41 && char_index <= 0x5A {
                    alphabet_indexes[i] = alpha_index + 1;
                } else if char_index >= 0x61 && char_index <= 0x7A {
                    alphabet_indexes[i] = alpha_index + 1;
                } else if char_index >= 0x30 && char_index <= 0x39 {
                    alphabet_indexes[i] = digit_index + 1;
                }
            }
        }

        // Create table
        let mut table = TransitionTable {
            table: vec![vec![None; temp_alphabet.len()].into_boxed_slice(); state_count].into_boxed_slice(),
            accepting: vec![false; state_count].into_boxed_slice(),
            alphabet: temp_alphabet.into_boxed_slice(),
            alphabet_indexes,
            alphabet_start,
            token_map: vec![None; state_count].into_boxed_slice(),
        };

        // Add DFAs to table
        let mut start_state = 1;
        for dfa in dfas.iter() {
            start_state = table.add_dfa(dfa, start_state);
        }

        table
    }

    fn add_dfa(&mut self, dfa: &Dfa, start_state: usize) -> usize {
        // Check for errors
        if dfa.accepting.len() != dfa.dfa.len() - 1 {
            panic!("Accepting states length does not match number of states! {} != {}", dfa.accepting.len(), dfa.dfa.len());
        }

        for (row_i, row) in dfa.dfa.iter().enumerate() {
            let table_state = if row_i == 0 { 0 } else { start_state + row_i - 1 };
            for (col_i, col) in row.iter().enumerate() {
                if let Some(state) = col {
                    if *state >= dfa.dfa.len() {
                        panic!("State out of bounds! {} >= {}", state, dfa.dfa.len());
                    }

                    // Get table alphabet index for current column
                    let mut alpha_index = 0;
                    for i in 0..self.alphabet.len() {
                        if self.alphabet[i] == dfa.alphabet[col_i] {
                            alpha_index = i;
                            break;
                        }
                    }

                    if self.table[table_state][alpha_index].is_some() {
                        panic!(
                            "Duplicate entry in table! table[{}][{}] ('{}') is already set to {} but trying to set it to {}",
                            table_state,
                            alpha_index,
                            self.alphabet[alpha_index],
                            self.table[table_state][alpha_index].unwrap(),
                            state + start_state - 1
                        );
                    }

                    self.table[table_state][alpha_index] = Some(state + start_state - 1);
                }
            }
        }
    
        // Add accepting states to table
        for (accept_i, accept) in dfa.accepting.iter().enumerate() {
            self.accepting[start_state + accept_i] = accept.clone();
        }
    
        // Add token map to table
        if let Some(token_map) = &dfa.token_map {
            for (token, states) in token_map.iter() {
                if states.len() == 0 {
                    panic!("Token map states length is 0 for token {:?}!", token);
                }

                let mapped_states = (states.iter().map(|x| *x + start_state - 1).collect::<Vec<usize>>()).into_boxed_slice();
                let last_state = mapped_states.last().unwrap().clone();

                let curr_token_states = Some(TokenStates { token: token.clone(), states: mapped_states });
                if let Some(token_list) = &self.token_map[last_state] {
                    // Make sure token doesn't already exist for states
                    if token_list.contains(&curr_token_states) {
                        panic!("Token {:?} already exists for state {} in token map!", token, last_state);
                    }
                }

                // Add token to token map
                let new_token_states = 
                    self.add_token_state(&self.token_map[last_state], curr_token_states.unwrap());
                self.token_map[last_state] = Some(new_token_states);
            }
        }

        // Return new start state
        start_state + dfa.dfa.len() - 1
    }

    fn add_token_state(&self, curr_token_states: &Option<Box<[Option<TokenStates>]>>, new_token_states: TokenStates) -> Box<[Option<TokenStates>]> {
        if curr_token_states.is_none() {
            return vec![Some(new_token_states)].into_boxed_slice();
        }

        let curr_token_states = curr_token_states.clone().unwrap();

        let mut size = curr_token_states.len() + 1;
        let mut new_states: Box<[Option<TokenStates>]> = vec![None; size].into_boxed_slice();
        
        let mut has_collisions = true;
        while has_collisions {
            has_collisions = false;

            // Add new token states
            let mut hash = self.get_states_hash(&new_token_states.states, size);
            new_states[hash] = Some(new_token_states.clone());
            
            // Add old token states
            for i in 0..curr_token_states.len() {
                if let Some(token_states) = &curr_token_states[i] {
                    hash = self.get_states_hash(&token_states.states, size);
                    if new_states[hash].is_some() {
                        size += 1;
                        new_states = vec![None; size].into_boxed_slice();
                        has_collisions = true;
                        break;
                    }
                    new_states[hash] = Some(token_states.clone());
                }
            }
        }

        new_states
    }

    fn get_states_hash(&self, states: &[usize], state_mod: usize) -> usize {
        let mut index = states[0];
        for (i, state) in states.iter().skip(1).enumerate() {
            index += state * usize::pow(7, (i + 1) as u32);
        }
        index % state_mod
    }

    fn get_alphabet_index(&self, input: char) -> Option<usize> {
        let input_int = input as usize;
        if input_int < self.alphabet_start {
            return None;
        }
        
        let input_int = input_int - self.alphabet_start;
        if input_int >= self.alphabet_indexes.len() {
            return None;
        }
        
        let index = self.alphabet_indexes[input_int];
        if index == 0 {
            return None;
        }
        
        Some(index - 1)
    }

    pub fn get_next_state(&self, state: usize, input: char) -> Option<usize> {
        let col = self.get_alphabet_index(input)?;
        self.table[state][col]
    }

    pub fn is_accepting(&self, state: usize) -> bool {
        self.accepting[state]
    }

    pub fn get_token(&self, states: &[usize]) -> Option<Token> {
        if states.len() == 0 {
            return None;
        }

        if let Some(token_states) = &self.token_map[states.last().unwrap().clone()] {
            if token_states.len() == 1 {
                return Some(token_states[0].clone().unwrap().token.clone());
            } else {
                let state_hash = self.get_states_hash(states, token_states.len());
                if let Some(token) = token_states[state_hash].clone() {
                    return Some(token.token.clone());
                }
            }
        }

        return None;
    }
}

pub fn init_transition_table() -> TransitionTable {
    let dfas = get_constant_dfas();
    let table = TransitionTable::init(&dfas);
    
    // dbg!(&table);
    table
}