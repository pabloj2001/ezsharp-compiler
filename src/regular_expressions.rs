use std::{vec, collections::HashMap};

use crate::lexical::Token;

#[derive(Debug)]
pub struct StateTable {
    table: Vec<Vec<Option<usize>>>,
    accepting: Vec<bool>,
    alphabet: Vec<char>,
    token_map: HashMap<String, Token>,
}

impl StateTable {
    fn add_dfa(&mut self, dfa: &Dfa) {
        // Add alphabet to table
        let mut col_indices: Vec<usize> = vec![];
        for c in dfa.alphabet.iter() {
            if let Some(index) = self.alphabet.iter().position(|x| *x == *c) {
                col_indices.push(index);
            } else {
                self.alphabet.push(*c);
                col_indices.push(self.alphabet.len() - 1);
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
    
        let state_count = self.table.len();
    
        // Add states to table
        for row in dfa.dfa.iter() {
            let mut new_row: Vec<Option<usize>> = vec![];
            let mut i: usize = 0;
            for j in 0..self.alphabet.len() {
                if j == col_indices[i] {
                    if let Some(state) = row[i] {
                        new_row.push(Some(state + state_count));
                    } else {
                        new_row.push(None);
                    }
                    i += 1;
                } else {
                    new_row.push(None);
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

    fn get_alphabet_index(&self, input: char) -> Option<usize> {
        let int_input = input as u8;
        if (int_input >= 0x41 && int_input <= 0x5A) || (int_input >= 0x61 && int_input <= 0x7A) {
            return self.alphabet.iter().position(|x| *x == ALPHA);
        } else if int_input >= 0x30 && int_input <= 0x39 {
            return self.alphabet.iter().position(|x| *x == DIGIT);
        }
        self.alphabet.iter().position(|x| *x == input)
    }

    pub fn get_next_state(&self, state: usize, input: char) -> Option<usize> {
        let col = self.get_alphabet_index(input)?;
        self.table[state][col]
    }

    pub fn is_accepting(&self, state: usize) -> bool {
        self.accepting[state]
    }

    pub fn get_token(&self, states: Vec<usize>) -> Option<Token> {
        let state_hash = get_states_hash(&states);
        self.token_map.get(&state_hash).map(|x| x.clone())
    }
}

struct Dfa {
    dfa: Vec<Vec<Option<usize>>>,
    accepting: Vec<bool>,
    alphabet: Vec<char>,
    token_map: Option<Vec<(Token, Vec<usize>)>>,
}

const ALPHA: char = 0x01 as char;
const DIGIT: char = 0x02 as char;

fn get_constant_dfas() -> Vec<Dfa> {
    let id_alphabet: Vec<char> = vec!['_', ALPHA, DIGIT];
    let id_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(1), None],
        vec![Some(1), Some(1), Some(1)],
    ];
    let id_dfa_accepting: Vec<bool> = vec![false, true];
    let id_token_map: Vec<(Token, Vec<usize>)> = vec![(Token::Identifier("".to_string()), vec![1])];
    let id_dfa: Dfa = Dfa {
        dfa: id_dfa_states,
        accepting: id_dfa_accepting,
        alphabet: id_alphabet,
        token_map: Some(id_token_map),
    };

    let number_alphabet: Vec<char> = vec![DIGIT, '+', '-', '.', 'E'];
    let number_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(2), Some(2), Some(3), None],   // 0
        vec![Some(4), None, None, Some(5), None],         // 1
        vec![Some(1), None, None, Some(3), None],         // 2
        vec![Some(6), None, None, None, None],            // 3
        vec![Some(4), None, None, Some(7), None],         // 4
        vec![Some(6), None, None, None, None],            // 5
        vec![Some(6), None, None, None, Some(8)],         // 6
        vec![Some(9), None, None, None, None],            // 7
        vec![None, Some(10), Some(10), None, None],       // 8
        vec![Some(9), None, None, None, None],            // 9
        vec![Some(9), None, None, None, None],            // 10
    ];
    let number_dfa_accepting: Vec<bool> = vec![false, true, false, false, true, true, true, true, false, true, false];
    let number_token_map: Vec<(Token, Vec<usize>)> = vec![
        (Token::Tint(0), vec![1]),
        (Token::Tint(0), vec![1, 4]),
        (Token::Tint(0), vec![2, 1]),
        (Token::Tint(0), vec![2, 1, 4]),
        (Token::Tdouble(0.0), vec![0, 5]),
        (Token::Tdouble(0.0), vec![0, 6]),
        (Token::Tdouble(0.0), vec![0, 7]),
        (Token::Tdouble(0.0), vec![0, 9]),
    ];
    let number_dfa: Dfa = Dfa {
        dfa: number_dfa_states,
        accepting: number_dfa_accepting,
        alphabet: number_alphabet,
        token_map: Some(number_token_map),
    };

    let comparator_alphabet: Vec<char> = vec!['=', '<', '>'];
    let comparator_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(3), Some(5)],
        vec![Some(2), None, None],
        vec![None, None, None],
        vec![Some(2), None, Some(4)],
        vec![None, None, None],
        vec![Some(2), None, None],
    ];
    let comparator_dfa_accepting: Vec<bool> = vec![false, true, true, true, true, true];
    let comparator_token_map: Vec<(Token, Vec<usize>)> = vec![
        (Token::Oassign, vec![1]),
        (Token::Oequal, vec![1, 2]),
        (Token::Olt, vec![3]),
        (Token::Olte, vec![3, 2]),
        (Token::Ogt, vec![5]),
        (Token::Ogte, vec![5, 2]),
        (Token::Onot, vec![3, 4]),
    ];
    let comparator_dfa: Dfa = Dfa {
        dfa: comparator_dfa_states,
        accepting: comparator_dfa_accepting,
        alphabet: comparator_alphabet,
        token_map: Some(comparator_token_map),
    };

    vec![id_dfa, number_dfa, comparator_dfa]
}

pub fn init_state_table() -> Result<StateTable, String> {
    let mut table = StateTable {
        table: vec![],
        accepting: vec![],
        alphabet: vec![],
        token_map: HashMap::new(),
    };

    let dfas = get_constant_dfas();
    for dfa in dfas.iter() {
        table.add_dfa(dfa);
    }

    dbg!(&table);
    Ok(table)
}

pub fn get_states_hash(states: &Vec<usize>) -> String {
    let str_states: Vec<String> = states.iter().map(|x| x.to_string()).collect();
    str_states.join("-")
}