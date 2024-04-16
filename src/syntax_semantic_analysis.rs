mod ll1_table;
mod productions;
mod first_set;
mod follow_set;
mod non_terminals;
mod stack;
pub mod symbol_table;
mod syntax_analysis;
mod semantic_analysis;
pub mod symbol_declaration;
pub mod statement_tree;
mod semantic_actions;

use crate::{
    lexical_analysis::{ParsedToken, Token}, syntax_semantic_analysis::{ll1_table::{generate_ll1_table, LL1Table}, syntax_analysis::SyntaxErrorType}
};

use productions::ProductionType;
use non_terminals::NonTerminal;
use stack::Stack;
use symbol_table::SymbolTable;

use self::{semantic_analysis::SemanticError, syntax_analysis::SyntaxError};
use self::follow_set::FollowSetType;

pub struct SyntaxSemanticErrors {
    pub syntax_errors: Vec<SyntaxError>,
    pub semantic_errors: Vec<SemanticError>,
}

pub fn perform_syntax_semantic_analysis(tokens: Vec<ParsedToken>) -> Result<SymbolTable, SyntaxSemanticErrors> {
    let productions = productions::get_constant_productions();
    let follow_sets = follow_set::get_constant_follow_sets();

    // Generate LL(1) table
    let table: LL1Table = generate_ll1_table(&productions, &follow_sets);
    
    // Parse using a stack
    let mut semantic_info = semantic_analysis::SemanticInfo::new();

    let mut errors: SyntaxSemanticErrors = SyntaxSemanticErrors {
        syntax_errors: Vec::new(),
        semantic_errors: Vec::new(),
    };

    let mut stack = Stack::new();
    stack.push(ProductionType::NonTerminal(NonTerminal::Program));

    let mut token_iter = tokens.iter();
    let mut curr_token: &ParsedToken = &token_iter.next().unwrap();
    let mut prev_terminal: Option<Token> = None;

    while !stack.is_empty() {
        match stack.pop().unwrap() {
            ProductionType::Terminal(token) => {
                if !token.equals_type(&curr_token.token) {
                    // Unexpected terminal found
                    errors.syntax_errors.push(SyntaxError::new(
                        SyntaxErrorType::ExpectedToken(token.clone(), curr_token.token.clone()),
                        curr_token.line
                    ));
                }

                prev_terminal = Some(curr_token.token.clone());

                // Move to next token
                if let Some(next_token) = token_iter.next() {
                    curr_token = next_token;
                } else {
                    // No more tokens
                    if !stack.is_empty() {
                        // Unexpected end of file
                        errors.syntax_errors.push(SyntaxError::new(
                            SyntaxErrorType::UnexpectedEndOfFile,
                            curr_token.line
                        ));
                        return Err(errors);
                    }
                }
            },
            ProductionType::NonTerminal(non_terminal) => {
                let production_index = table[non_terminal.to_index()][curr_token.token.to_index()];
                match production_index {
                    Some(prod_index) => {
                        if prod_index >= productions.len() {
                            // Epsilon production
                            continue;
                        }

                        let production = &productions[prod_index];
                        for prod_elem in production.right.iter().rev() {
                            stack.push(prod_elem.clone());
                        }
                    },
                    None => {
                        // println!("No production found for {:?} and {:?}", non_terminal, curr_token.token);
                        // Current non terminal does not have a production for the current token
                        errors.syntax_errors.push(SyntaxError::new(
                            SyntaxErrorType::UnexpectedToken(curr_token.token.clone()),
                            curr_token.line
                        ));

                        // Find next token in the follow set of the current non terminal
                        let mut token_found = false;
                        while let Some(next_token) = token_iter.next() {
                            for follow_elem in follow_sets[non_terminal.to_index()].follow_set.iter() {
                                if let FollowSetType::Terminal(follow_token) = follow_elem {
                                    if follow_token == &next_token.token {
                                        // Resume from token found in follow set
                                        curr_token = next_token;
                                        token_found = true;
                                        break;
                                    }
                                }
                            }

                            if token_found {
                                break;
                            }
                        }

                        if !token_found {
                            // No more tokens
                            if !stack.is_empty() {
                                return Err(errors);
                            }
                        }
                    },
                }
            },
            ProductionType::Action(action) => {
                let action_result = semantic_info.perform_action(&action, &prev_terminal);
                match action_result {
                    Ok(_) => {},
                    Err(err) => {
                        errors.semantic_errors.push(SemanticError::new(err, curr_token.line));
                    },
                }
            },
        }
    }

    if token_iter.next().is_some() {
        // There are more tokens after parsing
        errors.syntax_errors.push(SyntaxError::new(
            SyntaxErrorType::UnexpectedEndOfFile,
            curr_token.line
        ));
    }
    
    // dbg!(&semantic_info.symbol_table);
    if !errors.syntax_errors.is_empty() || !errors.semantic_errors.is_empty() {
        return Err(errors);
    }

    Ok(semantic_info.symbol_table)
}