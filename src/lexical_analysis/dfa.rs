use std::vec;

use super::constants::{ALPHA, DIGIT, KEYWORDS};
use super::token::Token;

pub struct Dfa {
    pub dfa: Vec<Vec<Option<usize>>>,
    pub accepting: Vec<bool>,
    pub alphabet: Vec<char>,
    pub token_map: Option<Vec<(Token, Vec<usize>)>>,
}

fn get_keyword_identifier_dfa() -> Dfa {
    // Create alphabet and first state
    let mut key_id_alphabet: Vec<char> = vec!['_', ALPHA, DIGIT, 'E'];
    let mut key_id_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(1), None, Some(1)],
    ];
    let init_count = key_id_alphabet.len();

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

    for _ in (init_count - 1)..key_id_alphabet.len() {
        key_id_states.push(vec![Some(1); key_id_alphabet.len()]);
    }

    // Create accepting states
    let key_id_accepting: Vec<bool> = vec![true; key_id_states.len() - 1];

    // Create token map
    let mut key_id_token_map: Vec<(Token, Vec<usize>)> = vec![];
    for i in 1..key_id_states.len() {
        key_id_token_map.push((Token::Identifier(String::from("")), vec![0, i]));
    }

    // Create states for each keyword
    let alph_state_diff = init_count - 2;
    for keyword in KEYWORDS {
        let mut chars = keyword.0.chars();
        let curr_char = chars.nth(0).unwrap();
        let mut prev_index = key_id_alphabet.iter().position(|x| *x == curr_char).unwrap() - alph_state_diff;
        let mut state_transitions: Vec<usize> = vec![];

        for c in chars {
            let char_index = key_id_alphabet.iter().position(|x| *x == c).unwrap();

            // Set next state of previous state to the state for the current character            
            key_id_states[prev_index][char_index] = Some(char_index - alph_state_diff);
            // Add previous state to state transitions
            state_transitions.push(prev_index);

            prev_index = char_index - alph_state_diff;
        }

        // Add last state to state transitions
        state_transitions.push(prev_index);
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

pub fn get_constant_dfas() -> Vec<Dfa> {
    // Keyword and Identifier DFA
    let key_id_dfa = get_keyword_identifier_dfa();

    // Number DFA
    let number_alphabet: Vec<char> = vec![DIGIT, '-', '+', '.', 'E', 'e'];
    let number_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), None, None, Some(2), None, None],
        vec![Some(1), None, None, Some(3), None, None],
        vec![Some(3), None, None, None, None, None],
        vec![Some(3), None, None, None, Some(4), Some(4)],
        vec![Some(5), Some(5), Some(5), None, None, None],
        vec![Some(5), None, None, None, None, None],
    ];
    let number_dfa_accepting: Vec<bool> = vec![true, true, true, false, true];
    let number_token_map: Vec<(Token, Vec<usize>)> = vec![
        (Token::Tint(0), vec![1]),
        (Token::Speriod, vec![2]),
        (Token::Tdouble(0.0), vec![0, 3]),
        (Token::Tdouble(0.0), vec![0, 5]),
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
    let operator_alphabet: Vec<char> = vec!['+', '-', '*', '/', '%'];
    let operator_dfa_states: Vec<Vec<Option<usize>>> = vec![
        vec![Some(1), Some(2), Some(3), Some(4), Some(5)],
        vec![None],
        vec![None],
        vec![None],
        vec![None],
        vec![None],
    ];
    let operator_dfa_accepting: Vec<bool> = vec![true, true, true, true, true];
    let operator_token_map: Vec<(Token, Vec<usize>)> = vec![
        (Token::Oplus, vec![1]),
        (Token::Ominus, vec![2]),
        (Token::Omultiply, vec![3]),
        (Token::Odivide, vec![4]),
        (Token::Omod, vec![5]),
    ];
    let operator_dfa: Dfa = Dfa {
        dfa: operator_dfa_states,
        accepting: operator_dfa_accepting,
        alphabet: operator_alphabet,
        token_map: Some(operator_token_map),
    };

    vec![key_id_dfa, number_dfa, comparator_dfa, separator_dfa, operator_dfa]
}