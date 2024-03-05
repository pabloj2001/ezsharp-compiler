mod ll1_table;
mod productions;
mod first_set;
mod follow_set;
mod non_terminals;
mod stack;
mod symbol_table;

use crate::{
    lexical_analysis::{ParsedToken, Token},
    logger::Loggable,
    syntax_analysis::{ll1_table::{generate_ll1_table, LL1Table}, symbol_table::SymbolDecl},
};

use productions::ProductionType;
use non_terminals::NonTerminal;
use stack::Stack;
use symbol_table::{
    SymbolTable,
    SymbolScope,
    BasicType,
    SymbolTypeHint,
    get_new_scope,
    is_scope_end,
    should_reset_type_name
};

use self::follow_set::FollowSetType;

#[derive(Debug)]
enum SyntaxErrorType {
    ExpectedToken(Token, Token),
    UnexpectedToken(Token),
    UnexpectedEndOfFile,
}

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

impl Loggable for Box<[SyntaxError]> {
    fn to_log_message(&self) -> String {
        let mut msg = String::new();
        for error in self.iter() {
            msg.push_str(&error.to_log_message());
            msg.push('\n');
        }
        msg
    }
}

pub fn perform_syntax_analysis(tokens: Vec<ParsedToken>) -> Result<SymbolTable, Box<[SyntaxError]>> {
    let productions = productions::get_constant_productions();
    let follow_sets = follow_set::get_constant_follow_sets();

    // Generate LL(1) table
    let table: LL1Table = generate_ll1_table(&productions, &follow_sets);
    
    // Parse using a stack
    let mut symbol_table: SymbolTable = SymbolTable::new();
    let mut curr_scope: &mut SymbolScope = symbol_table.get_global();
    let mut symbol_type_hint: SymbolTypeHint = SymbolTypeHint::Variable;
    let mut curr_type: Option<BasicType> = None;
    let mut curr_name: Option<String> = None;

    let mut errors: Vec<SyntaxError> = Vec::new();

    let mut stack = Stack::new();
    stack.push(ProductionType::NonTerminal(NonTerminal::Program));

    let mut token_iter = tokens.iter();
    let mut curr_token: &ParsedToken = &token_iter.next().unwrap();

    while !stack.is_empty() {
        match stack.pop().unwrap() {
            ProductionType::Terminal(token) => {
                if !token.equals_type(&curr_token.token) {
                    // Unexpected terminal found
                    errors.push(SyntaxError {
                        error_type: SyntaxErrorType::ExpectedToken(token.clone(), curr_token.token.clone()),
                        line: curr_token.line
                    });
                }

                if is_scope_end(&token, curr_scope) {
                    curr_scope = symbol_table.get_global();
                    // Reset type and name
                    curr_type = None;
                    curr_name = None;
                } else if should_reset_type_name(&token) {
                    // Reset type and name
                    curr_type = None;
                    curr_name = None;
                }

                if let Token::Identifier(name) = &curr_token.token {
                    curr_name = Some(name.clone());
                } else if let Token::Kint = &curr_token.token {
                    curr_type = Some(BasicType::Int);
                } else if let Token::Kdouble = &curr_token.token {
                    curr_type = Some(BasicType::Double);
                }

                if let Some(name) = &curr_name {
                    if let Some(symbol_type) = &curr_type {
                        // Add declaration to current scope
                        let decl = SymbolDecl::from_state(symbol_type, name, &symbol_type_hint);
                        curr_scope.add_declaration(decl);
                        // Reset name
                        curr_name = None;
                    }
                }

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
                if let Some(scope_type) = get_new_scope(&non_terminal, curr_scope) {
                    // Go into new scope
                    let new_scope = SymbolScope::new(scope_type);
                    curr_scope = symbol_table.get_global().add_scope(new_scope);
                    // Reset type and name
                    curr_type = None;
                    curr_name = None;
                }

                if let Some(hint) = symbol_table::get_symbol_type_hint(&non_terminal) {
                    symbol_type_hint = hint;
                }
                
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
    Ok(symbol_table)
}