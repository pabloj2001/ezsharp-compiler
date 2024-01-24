use std::collections::HashMap;

use super::constants::{ALPHA, DIGIT};
use super::token::Token;
use super::dfa::{Dfa, get_constant_dfas};

#[derive(Debug)]
pub struct TransitionTable {
    table: Vec<Vec<Option<usize>>>,
    accepting: Vec<bool>,
    alphabet: Vec<char>,
    token_map: HashMap<String, Token>,
    alphabet_indexes: Box<[usize]>,
    alphabet_start: usize,
    alpha_index: usize,
    digit_index: usize,
}

impl TransitionTable {
    fn new() -> TransitionTable {
        TransitionTable {
            table: vec![],
            accepting: vec![false],
            alphabet: vec![],
            token_map: HashMap::new(),
            alphabet_indexes: Box::new([]),
            alphabet_start: 0,
            alpha_index: 0,
            digit_index: 0,
        }
    }

    fn add_dfa(&mut self, dfa: &mut Dfa) {
        // Check for errors
        if dfa.accepting.len() != dfa.dfa.len() - 1 {
            panic!("Accepting states length does not match number of states! {} != {}", dfa.accepting.len(), dfa.dfa.len());
        }

        // Add alphabet to table
        let mut col_indices: Vec<usize> = vec![];
        for c in dfa.alphabet.iter() {
            if let Some(index) = self.alphabet.iter().position(|x| *x == *c) {
                col_indices.push(index);
            } else {
                if *c == ALPHA {
                    self.alpha_index = self.alphabet.len();
                } else if *c == DIGIT {
                    self.digit_index = self.alphabet.len();
                }
                col_indices.push(self.alphabet.len());
                self.alphabet.push(*c);
            }
        }
    
        // Update previous states with new alphabet
        for row in self.table.iter_mut() {
            for i in 0..col_indices.len() {
                if col_indices[i] >= row.len() {
                    row.push(None);
                }
            }
        }
    
        let state_count = if self.table.len() == 0 { 0 } else { self.table.len() - 1 };

        // Add states to table
        let mut first_row = true;
        for row in dfa.dfa.iter_mut() {
            if row.len() != self.alphabet.len() {
                // Fill in missing columns with None
                row.resize(self.alphabet.len(), None);
            }

            if self.table.len() == 0 {
                first_row = false;
            }

            if first_row {
                // Add first state to first state of table
                for i in 0..col_indices.len() {
                    if let Some(state) = row[i] {
                        if self.table[0][col_indices[i]].is_some() {
                            panic!(
                                "Duplicate entry in starting state! table[0][{}] ('{}') is already set to {} but trying to set it to {}",
                                col_indices[i],
                                self.alphabet[col_indices[i]],
                                self.table[0][col_indices[i]].unwrap(),
                                state + state_count
                            );
                        }
                        self.table[0][col_indices[i]] = Some(state + state_count);
                    }
                }
                first_row = false;
                continue;
            }

            let mut new_row: Vec<Option<usize>> = vec![None; self.alphabet.len()];
            for i in 0..col_indices.len() {            
                if let Some(state) = row[i] {
                    new_row[col_indices[i]] = Some(state + state_count);
                }
            }
            self.table.push(new_row);
        }
    
        // Add accepting states to table
        for accept in dfa.accepting.iter() {
            self.accepting.push(*accept);
        }
    
        // Add token map to table
        if let Some(token_map) = &dfa.token_map {
            for (token, states) in token_map.iter() {
                let mapped_states: Vec<usize> = states.iter().map(|x| if *x > 0 { *x + state_count } else { 0 }).collect();
                let state_hash = get_states_hash(&mapped_states);
    
                if self.token_map.contains_key(&state_hash) {
                    panic!("Duplicate state hash in token map {}", state_hash);
                }
                self.token_map.insert(state_hash, token.clone());
            }
        }
    }

    fn finish_table(&mut self) {
        // Find smallest and largest characters
        let mut smallest_char = '0';
        let mut largest_char = 'z';
        for c in self.alphabet.iter() {
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
        self.alphabet_start = smallest_char as usize;
        self.alphabet_indexes = vec![0; (largest_char as usize) - self.alphabet_start + 1].into_boxed_slice();
        for i in 0..self.alphabet.len() {
            if self.alphabet[i] == ALPHA || self.alphabet[i] == DIGIT {
                continue;
            }
            self.alphabet_indexes[(self.alphabet[i] as usize) - self.alphabet_start] = i + 1;
        }

        // Set unset alphabet and digit indexes
        for i in 0..self.alphabet_indexes.len() {
            if self.alphabet_indexes[i] == 0 {
                let char_index = i + self.alphabet_start;
                if char_index >= 0x41 && char_index <= 0x5A {
                    self.alphabet_indexes[i] = self.alpha_index + 1;
                } else if char_index >= 0x61 && char_index <= 0x7A {
                    self.alphabet_indexes[i] = self.alpha_index + 1;
                } else if char_index >= 0x30 && char_index <= 0x39 {
                    self.alphabet_indexes[i] = self.digit_index + 1;
                }
            }
        }
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

    pub fn get_token(&self, states: Vec<usize>) -> Option<Token> {
        if states.len() == 0 {
            return None;
        }

        let mut state_hash = get_states_hash(&states);
        if let Some(token) = self.token_map.get(&state_hash) {
            return Some(token.clone());
        }

        // Check for tokens only requiring an ending state
        let ending_states = vec![0, states.last().unwrap().clone()];
        state_hash = get_states_hash(&ending_states);
        if let Some(token) = self.token_map.get(&state_hash) {
            return Some(token.clone());
        }

        return None;
    }
}

pub fn init_transition_table() -> Result<TransitionTable, String> {
    let mut table = TransitionTable::new();

    let mut dfas = get_constant_dfas();
    for dfa in dfas.iter_mut() {
        table.add_dfa(dfa);
    }
    table.finish_table();

    // dbg!(&table);
    Ok(table)
}

pub fn get_states_hash(states: &Vec<usize>) -> String {
    let str_states: Vec<String> = states.iter().map(|x| x.to_string()).collect();
    str_states.join("-")
}