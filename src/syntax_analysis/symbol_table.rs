use super::non_terminals::NonTerminal;
use crate::{lexical_analysis::Token, logger::Loggable};

#[derive(Clone, Debug)]
pub enum BasicType {
    Int,
    Double,
}

pub enum SymbolType {
    Int,
    Double,
    Function(BasicType),
}

pub enum SymbolTypeHint {
    Variable,
    Function,
}

pub fn get_symbol_type_hint(non_terminal: &NonTerminal) -> Option<SymbolTypeHint> {
    match non_terminal {
        NonTerminal::Fdec => Some(SymbolTypeHint::Function),
        NonTerminal::Decl => Some(SymbolTypeHint::Variable),
        NonTerminal::Params => Some(SymbolTypeHint::Variable),
        _ => None,
    }
}

pub struct SymbolDecl {
    pub name: String,
    symbol_type: SymbolType,
}

impl SymbolDecl {
    pub fn from_state(basic_type: &BasicType, name: &String, hint: &SymbolTypeHint) -> SymbolDecl {
        let symbol_type = match basic_type {
            BasicType::Int => SymbolType::Int,
            BasicType::Double => SymbolType::Double,
        };
        
        match hint {
            SymbolTypeHint::Variable => SymbolDecl {
                name: name.clone(),
                symbol_type,
            },
            SymbolTypeHint::Function => {
                SymbolDecl {
                    name: name.clone(),
                    symbol_type: SymbolType::Function(basic_type.clone()),
                }
            },
        }
    }
}

impl Loggable for SymbolDecl {
    fn to_log_message(&self) -> String {
        let mut msg = match &self.symbol_type {
            SymbolType::Int => String::from("int "),
            SymbolType::Double => String::from("double "),
            SymbolType::Function(basic_type) => {
                match basic_type {
                    BasicType::Int => String::from("Func int "),
                    BasicType::Double => String::from("Func double "),
                }
            }
        };
        msg.push_str(format!("{};\n", &self.name).as_str());
        msg
    }
}

pub enum ScopeType {
    Global,
    Local,
    Parameters,
}

impl Loggable for ScopeType {
    fn to_log_message(&self) -> String {
        match self {
            ScopeType::Local => String::from("Local"),
            ScopeType::Parameters => String::from("Parameters"),
            ScopeType::Global => String::from("Global"),
        }
    }
}

pub enum SymbolEntry {
    Scope(SymbolScope),
    Decl(SymbolDecl),
}

pub struct SymbolScope {
    children: Vec<SymbolEntry>,
    scope_type: ScopeType,
}

impl SymbolScope {
    pub fn new(scope_type: ScopeType) -> SymbolScope {
        SymbolScope {
            children: Vec::new(),
            scope_type,
        }
    }
    
    pub fn add_scope(&mut self, scope: SymbolScope) -> &mut SymbolScope {
        self.children.push(SymbolEntry::Scope(scope));
        match self.children.last_mut().unwrap() {
            SymbolEntry::Scope(scope) => scope,
            _ => panic!("Expected scope"),
        }
    }

    pub fn add_declaration(&mut self, symbol: SymbolDecl) {
        self.children.push(SymbolEntry::Decl(symbol));
    }
}

impl Loggable for SymbolScope {
    fn to_log_message(&self) -> String {
        let tabs = match &self.scope_type {
            ScopeType::Global => 1,
            ScopeType::Local => 2,
            ScopeType::Parameters => 2,
        };

        let mut message = format!("{} {{\n", &self.scope_type.to_log_message());
        for child in &self.children {
            message.push_str(&"\t".repeat(tabs));
            match child {
                SymbolEntry::Scope(scope) => message.push_str(&scope.to_log_message()),
                SymbolEntry::Decl(decl) => message.push_str(&decl.to_log_message()),
            }
        }
        message.push_str(format!("{}}}\n", &"\t".repeat(tabs - 1)).as_str());
        message
    }
}

pub struct SymbolTable {
    global: SymbolScope,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            global: SymbolScope::new(ScopeType::Global),
        }
    }

    pub fn get_global(&mut self) -> &mut SymbolScope {
        &mut self.global
    }
}

impl Loggable for SymbolTable {
    fn to_log_message(&self) -> String {
        self.global.to_log_message()
    }
}

pub fn get_new_scope(non_terminal: &NonTerminal, curr_scope: &SymbolScope) -> Option<ScopeType> {
    match non_terminal {
        NonTerminal::DeclarationsSeq => {
            match curr_scope.scope_type {
                ScopeType::Parameters => Some(ScopeType::Local),
                _ => None,
            }
        },
        NonTerminal::Params => {
            match curr_scope.scope_type {
                // Global scope
                ScopeType::Global => Some(ScopeType::Parameters),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn is_scope_end(terminal: &Token, curr_scope: &SymbolScope) -> bool {
    match terminal {
        Token::Kfed => {
            match curr_scope.scope_type {
                ScopeType::Local => true,
                _ => false,
            }
        },
        _ => false,
    }
}

pub fn should_reset_type_name(token: &Token) -> bool {
    token.equals_type(&Token::Ssemicolon)
}