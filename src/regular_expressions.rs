use std::{vec, collections::HashMap};

use crate::lexical::Token;

#[derive(Debug)]
pub struct StateTable {
    table: Vec<Vec<Option<usize>>>,
    accepting: Vec<bool>,
    alphabet: Vec<char>,
    token_map: HashMap<String, Token>,
    alpha_index: usize,
    digit_index: usize,
}

impl StateTable {
    fn new() -> StateTable {
        StateTable {
            table: vec![],
            accepting: vec![false],
            alphabet: vec![],
            token_map: HashMap::new(),
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
                self.alphabet.push(*c);
                if *c == ALPHA {
                    self.alpha_index = self.alphabet.len() - 1;
                } else if *c == DIGIT {
                    self.digit_index = self.alphabet.len() - 1;
                }
                col_indices.push(self.alphabet.len() - 1);
            }
        }
        col_indices.sort();
    
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

            let mut i: usize = 0;
            if first_row {
                // Add first state to first state of table
                for j in 0..self.alphabet.len() {
                    if j == col_indices[i] {
                        if let Some(state) = row[i] {
                            if self.table[0][j].is_some() {
                                panic!(
                                    "Duplicate entry in starting state! table[0][{}] is already set to {} but trying to set it to {}",
                                    j,
                                    self.table[0][j].unwrap(),
                                    state + state_count
                                );
                            }
                            self.table[0][j] = Some(state + state_count);
                        }
                        i += 1;
                    }
                }
                first_row = false;
                continue;
            }

            let mut new_row: Vec<Option<usize>> = vec![];
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
        if let Some(index) = self.alphabet.iter().position(|x| *x == input) {
            return Some(index);
        }

        let int_input = input as u8;
        if (int_input >= 0x41 && int_input <= 0x5A) || (int_input >= 0x61 && int_input <= 0x7A) {
            return Some(self.alpha_index);
        } else if int_input >= 0x30 && int_input <= 0x39 {
            return Some(self.digit_index);
        }
        
        return None
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

struct Dfa {
    dfa: Vec<Vec<Option<usize>>>,
    accepting: Vec<bool>,
    alphabet: Vec<char>,
    token_map: Option<Vec<(Token, Vec<usize>)>>,
}

const ALPHA: char = 0x01 as char;
const DIGIT: char = 0x02 as char;
const KEYWORDS: [(&str, Token); 16] = [
    ("if", Token::Kif),
    ("then", Token::Kthen),
    ("fi", Token::Kfi),
    ("else", Token::Kelse),
    ("while", Token::Kwhile),
    ("do", Token::Kdo),
    ("od", Token::Kod),
    ("def", Token::Kdef),
    ("fed", Token::Kfed),
    ("return", Token::Kreturn),
    ("and", Token::Kand),
    ("or", Token::Kor),
    ("not", Token::Knot),
    ("int", Token::Kint),
    ("double", Token::Kdouble),
    ("print", Token::Kprint),
];

fn get_keyword_identifier_dfa() -> Dfa {
    // Create alphabet and first state
    let mut key_id_alphabet: Vec<char> = vec!['_', ALPHA, DIGIT];
    let mut key_id_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(1), None],
    ];

    let mut curr_state = 2;
    for keyword in KEYWORDS {
        for c in keyword.0.chars() {
            let index = key_id_alphabet.iter().position(|x| *x == c);
            if index.is_none() {
                key_id_alphabet.push(c);
                key_id_states[0].push(Some(curr_state));
                curr_state += 1;
            }
        }
    }

    for _ in 2..key_id_alphabet.len() {
        key_id_states.push(vec![Some(1); key_id_alphabet.len()]);
    }

    // Create accepting states
    let key_id_accepting: Vec<bool> = vec![true; key_id_alphabet.len() - 2];

    // Create token map
    let mut key_id_token_map: Vec<(Token, Vec<usize>)> = vec![];
    for i in 0..key_id_alphabet.len() - 2 {
        key_id_token_map.push((Token::Identifier(String::from("")), vec![0, i + 1]));
    }

    // Create states for each keyword
    for keyword in KEYWORDS {
        let mut chars = keyword.0.chars();
        let curr_char = chars.nth(0).unwrap();
        let mut prev_index = key_id_alphabet.iter().position(|x| *x == curr_char).unwrap();
        let mut state_transitions: Vec<usize> = vec![];

        for c in chars {
            let state_index = key_id_alphabet.iter().position(|x| *x == c).unwrap();

            // Set next state of previous state to the state for the current character            
            key_id_states[prev_index - 1][state_index] = Some(state_index - 1);
            // Add previous state to state transitions
            state_transitions.push(prev_index - 1);

            prev_index = state_index;
        }

        // Add last state to state transitions
        state_transitions.push(prev_index - 1);
        // Add token map for keyword
        key_id_token_map.push((keyword.1.clone(), state_transitions));
    }

    Dfa {
        dfa: key_id_states,
        accepting: key_id_accepting,
        alphabet: key_id_alphabet,
        token_map: Some(key_id_token_map),
    }
}

fn get_constant_dfas() -> Vec<Dfa> {
    // Keyword and Identifier DFA
    let key_id_dfa = get_keyword_identifier_dfa();

    // Number DFA
    let number_alphabet: Vec<char> = vec![DIGIT, '-', '.', 'E'];
    let number_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(2), Some(3), None],    // 0
        vec![Some(4), None, Some(5), None],       // 1
        vec![Some(1), None, Some(3), None],       // 2
        vec![Some(6), None, None, None],          // 3
        vec![Some(4), None, Some(7), None],       // 4
        vec![Some(6), None, None, None],          // 5
        vec![Some(6), None, None, Some(8)],       // 6
        vec![Some(9), None, None, None],          // 7
        vec![None, Some(10), None, None],         // 8
        vec![Some(9), None, None, None],          // 9
        vec![Some(9), None, None, None],          // 10
    ];
    let number_dfa_accepting: Vec<bool> = vec![true, true, true, true, true, true, true, false, true, false];
    let number_token_map: Vec<(Token, Vec<usize>)> = vec![
        (Token::Tint(0), vec![1]),
        (Token::Tint(0), vec![1, 4]),
        (Token::Tint(0), vec![2, 1]),
        (Token::Tint(0), vec![2, 1, 4]),
        (Token::Tdouble(0.0), vec![0, 5]),
        (Token::Tdouble(0.0), vec![0, 6]),
        (Token::Tdouble(0.0), vec![0, 7]),
        (Token::Tdouble(0.0), vec![0, 9]),
        (Token::Ominus, vec![2]),
        (Token::Speriod, vec![3]),
    ];
    let number_dfa: Dfa = Dfa {
        dfa: number_dfa_states,
        accepting: number_dfa_accepting,
        alphabet: number_alphabet,
        token_map: Some(number_token_map),
    };

    // Comparator DFA
    let comparator_alphabet: Vec<char> = vec!['=', '<', '>'];
    let comparator_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(3), Some(5)],
        vec![Some(2), None, None],
        vec![None, None, None],
        vec![Some(2), None, Some(4)],
        vec![None, None, None],
        vec![Some(2), None, None],
    ];
    let comparator_dfa_accepting: Vec<bool> = vec![true, true, true, true, true];
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

    // Separator DFA
    let separator_alphabet: Vec<char> = vec![';', ',', '(', ')'];
    let separator_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(2), Some(3), Some(4)],
        vec![None],
        vec![None],
        vec![None],
        vec![None],
    ];
    let separator_dfa_accepting: Vec<bool> = vec![true, true, true, true];
    let separator_token_map: Vec<(Token, Vec<usize>)> = vec![
        (Token::Ssemicolon, vec![1]),
        (Token::Scomma, vec![2]),
        (Token::Soparen, vec![3]),
        (Token::Scparen, vec![4]),
    ];
    let separator_dfa: Dfa = Dfa {
        dfa: separator_dfa_states,
        accepting: separator_dfa_accepting,
        alphabet: separator_alphabet,
        token_map: Some(separator_token_map),
    };

    // Operator DFA
    let operator_alphabet: Vec<char> = vec!['+', '*', '/', '%'];
    let operator_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(2), Some(3), Some(4)],
        vec![None],
        vec![None],
        vec![None],
        vec![None],
    ];
    let operator_dfa_accepting: Vec<bool> = vec![true, true, true, true];
    let operator_token_map: Vec<(Token, Vec<usize>)> = vec![
        (Token::Oplus, vec![1]),
        (Token::Omultiply, vec![2]),
        (Token::Odivide, vec![3]),
        (Token::Omod, vec![4]),
    ];
    let operator_dfa: Dfa = Dfa {
        dfa: operator_dfa_states,
        accepting: operator_dfa_accepting,
        alphabet: operator_alphabet,
        token_map: Some(operator_token_map),
    };

    vec![key_id_dfa, number_dfa, comparator_dfa, separator_dfa, operator_dfa]
}

pub fn init_state_table() -> Result<StateTable, String> {
    let mut table = StateTable::new();

    let mut dfas = get_constant_dfas();
    for dfa in dfas.iter_mut() {
        table.add_dfa(dfa);
    }

    // dbg!(&table);
    Ok(table)
}

pub fn get_states_hash(states: &Vec<usize>) -> String {
    let str_states: Vec<String> = states.iter().map(|x| x.to_string()).collect();
    str_states.join("-")
}