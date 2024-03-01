use super::first_set::FirstSetType;
use super::productions;
use super::first_set;
use super::follow_set;
use super::symbols;

pub fn generate_ll1_table() -> [[Option<usize>; symbols::NUM_TERMINALS]; symbols::NUM_NON_TERMINALS] {
    let productions = productions::get_constant_productions();
    let first_sets = first_set::get_constant_first_sets();
    let follow_sets = follow_set::get_constant_follow_sets();

    let mut table: [[Option<usize>; symbols::NUM_TERMINALS]; symbols::NUM_NON_TERMINALS]
        = [[None; symbols::NUM_TERMINALS]; symbols::NUM_NON_TERMINALS];
    
    // Loop thru first sets
    for first_set::FirstSet{ non_terminal, first_set } in first_sets.iter() {
        // Find production for non_terminal that matches first set
        for (i, production) in productions.iter().enumerate() {
            if production.left == *non_terminal {
                for terminal in first_set.iter() {
                    match terminal {
                        FirstSetType::Terminal(token) => {
                            table[non_terminal.to_index()][token.to_index()] = Some(i);
                        }
                        FirstSetType::Epsilon => {
                            
                        },
                    }
                }
            }
        }
    }

    table
}