mod ll1_table;
mod productions;
mod first_set;
mod follow_set;
mod non_terminals;
mod stack;
mod declarations;

use crate::{
    lexical_analysis::{ParsedToken, Token},
    syntax_analysis::{declarations::PartialDeclaration, ll1_table::{generate_ll1_table, LL1Table}},
    logger::Loggable,
};

use productions::ProductionType;
use non_terminals::NonTerminal;
use stack::Stack;
use declarations::{ParsedDeclaration, DeclarationValue};

use self::follow_set::FollowSetType;

#[derive(Debug)]
enum SyntaxErrorType {
    ExpectedToken(Token, Token),
    UnexpectedToken(Token),
    UnexpectedEndOfFile,
}

#[derive(Debug)]
pub struct SyntaxError {
    error_type: SyntaxErrorType,
    line: usize,
}

impl Loggable for SyntaxError {
    fn to_log_message(&self) -> String {
        match &self.error_type {
            SyntaxErrorType::ExpectedToken(expected, found) => format!("Expected token {:?} but found {:?} on line {}", expected, found, self.line),
            SyntaxErrorType::UnexpectedToken(token) => format!("Unexpected token {:?} on line {}", token, self.line),
            SyntaxErrorType::UnexpectedEndOfFile => format!("Unexpected end of file on line {}", self.line),
        }
    }
}

pub fn perform_syntax_analysis(tokens: Vec<ParsedToken>) -> Result<Box<[ParsedDeclaration]>, Box<[SyntaxError]>> {
    println!("Performing syntax analysis");

    let productions = productions::get_constant_productions();
    let follow_sets = follow_set::get_constant_follow_sets();

    // Generate LL(1) table
    let table: LL1Table = generate_ll1_table(&productions, &follow_sets);
    
    // Parse using a stack
    let decls: Vec<ParsedDeclaration> = Vec::new();
    let mut curr_decl: PartialDeclaration = PartialDeclaration::new();
    let mut errors: Vec<SyntaxError> = Vec::new();

    let mut stack = Stack::new();
    stack.push(ProductionType::NonTerminal(NonTerminal::Program));

    let mut token_iter = tokens.iter();
    let mut curr_token: &ParsedToken = &token_iter.next().unwrap();

    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        println!("Current Symbol: {:?}", top);
        match top {
            ProductionType::Terminal(token) => {
                if !token.equals_type(&curr_token.token) {
                    // Unexpected terminal found
                    errors.push(SyntaxError {
                        error_type: SyntaxErrorType::ExpectedToken(token.clone(), curr_token.token.clone()),
                        line: curr_token.line
                    });
                }

                println!("Valid token: {:?}", &curr_token.token);
                // Move to next token
                if let Some(next_token) = token_iter.next() {
                    curr_token = next_token;
                } else {
                    // No more tokens
                    if !stack.is_empty() {
                        // Unexpected end of file
                        errors.push(SyntaxError {
                            error_type: SyntaxErrorType::UnexpectedEndOfFile,
                            line: curr_token.line
                        });
                        return Err(errors.into_boxed_slice());
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
                        // println!("Adding productions: {:?}", &production.right);
                        for prod_elem in production.right.iter().rev() {
                            stack.push(prod_elem.clone());
                        }
                    },
                    None => {
                        println!("Unexpected token: {:?}", &curr_token.token);
                        // Current non terminal does not have a production for the current token
                        errors.push(SyntaxError {
                            error_type: SyntaxErrorType::UnexpectedToken(curr_token.token.clone()),
                            line: curr_token.line
                        });

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
                                return Err(errors.into_boxed_slice());
                            }
                        }
                    },
                }
            },
        }
    }

    if !errors.is_empty() {
        return Err(errors.into_boxed_slice());
    }
    Ok(decls.into_boxed_slice())
}