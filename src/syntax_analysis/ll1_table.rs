use super::first_set;
use super::first_set::FirstSetType;
use super::follow_set::FollowSetType;
use super::productions;
use super::productions::ProductionType;
use super::follow_set;
use super::symbols;

pub type LL1Table = [[Option<usize>; symbols::NUM_TERMINALS + 1]; symbols::NUM_NON_TERMINALS];

fn set_table_entry(
    table: &mut LL1Table,
    non_terminal: &symbols::NonTerminal,
    token_index: usize,
    production_index: usize
) {
    assert!(
        table[non_terminal.to_index()][token_index].is_none(),
        "Conflict in LL(1) table at non terminal {:?} and token {:?}",
        non_terminal,
        token_index
    );
    table[non_terminal.to_index()][token_index] = Some(production_index);
}

pub fn generate_ll1_table() -> LL1Table {
    let productions = productions::get_constant_productions();
    let first_sets = first_set::get_constant_first_sets();
    let follow_sets = follow_set::get_constant_follow_sets();

    let mut table: LL1Table = [[None; (symbols::NUM_TERMINALS + 1)]; symbols::NUM_NON_TERMINALS];
    
    // Loop thru first sets
    for first_set::FirstSet{ non_terminal, first_set } in first_sets.iter() {
        for fs_type in first_set.iter() {
            if let FirstSetType::Terminal(token) = fs_type {
                // If first set contains terminal, add to table
                for (i, production) in productions.iter().enumerate() {
                    // Find non_terminal production that matches first set
                    if *non_terminal == production.left {
                        // Go through production to ensure first set terminal is part of it
                        let mut prod_elem_contains_epsilon = false;
                        for prod_elem in production.right.iter() {
                            if let ProductionType::NonTerminal(prod_non_terminal) = prod_elem {
                                // Check if in first set of production non terminal
                                for prod_first_elem in first_sets[prod_non_terminal.to_index()].first_set.iter() {
                                    if let FirstSetType::Terminal(prod_token) = prod_first_elem {
                                        if token == prod_token {
                                            set_table_entry(&mut table, non_terminal, token.to_index(), i);
                                            break;
                                        }
                                    } else {
                                        // If non terminal first set contains epsilon, might need to check next element in production
                                        prod_elem_contains_epsilon = true;
                                    }
                                }
                            } else if let ProductionType::Terminal(prod_token) = prod_elem {
                                // Check if terminal in first set matches first terminal in production
                                if token == prod_token {
                                    set_table_entry(&mut table, non_terminal, token.to_index(), i);
                                    break;
                                }
                            }

                            if !prod_elem_contains_epsilon {
                                break;
                            }
                        }

                        if table[non_terminal.to_index()][token.to_index()].is_some() {
                            // Production for token in this first set has been found
                            break;
                        }
                    }
                }                
                assert!(table[non_terminal.to_index()][token.to_index()].is_some(), "No entry added to table for {:?} -> {:?}", non_terminal, token);
            } else if let FirstSetType::Epsilon = fs_type {
                // If first set contains Epsilon, add follow set to table
                for follow_elem in follow_sets[non_terminal.to_index()].follow_set.iter() {
                    if let FollowSetType::Terminal(token) = follow_elem {
                        set_table_entry(&mut table, non_terminal, token.to_index(), productions.len());
                    } else {
                        // End of input
                        set_table_entry(&mut table, non_terminal, symbols::NUM_TERMINALS, productions.len());
                    }
                }
            }
        }
    }

    table
}