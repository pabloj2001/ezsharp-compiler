use crate::logger::Loggable;

use super::{semantic_analysis::SemanticErrorType, statement_tree::StatementTree, symbol_declaration::SymbolDecl};

#[derive(Debug)]
pub enum ScopeType {
    Global,
    Function,
    Local,
}

impl Loggable for ScopeType {
    fn to_log_message(&self) -> String {
        match self {
            ScopeType::Global => String::from("Global"),
            ScopeType::Function => String::from("Function"),
            ScopeType::Local => String::from("Local"),
        }
    }
}

#[derive(Debug)]
pub enum SymbolEntry {
    Decl(SymbolDecl),
    Scope(usize),
    StatementTree(StatementTree),
}

#[derive(Debug)]
pub struct SymbolScope {
    symbols: Vec<SymbolEntry>,
    scope_type: ScopeType,
    scope_id: usize,
    parent_scope: usize,
}

impl SymbolScope {
    pub fn new(scope_type: ScopeType, scope_id: usize, parent_scope: usize) -> SymbolScope {
        SymbolScope {
            symbols: Vec::new(),
            scope_type,
            scope_id,
            parent_scope,
        }
    }

    pub fn add_declaration(&mut self, decl: SymbolDecl) {
        self.symbols.push(SymbolEntry::Decl(decl));
    }

    pub fn add_decl_new_scope(&mut self, decl: SymbolDecl, new_scope: usize) {
        self.symbols.push(SymbolEntry::Decl(decl));
        self.symbols.push(SymbolEntry::Scope(new_scope));
    }
}

pub const GLOBAL_SCOPE: usize = 0;

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<SymbolScope>,
    decls: Vec<SymbolDecl>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            scopes: vec![SymbolScope::new(ScopeType::Global, GLOBAL_SCOPE, GLOBAL_SCOPE)],
            decls: Vec::new(),
        }
    }

    pub fn add_scope(&mut self, scope_type: ScopeType, curr_scope: usize) -> usize {
        self.scopes.push(SymbolScope::new(scope_type, self.scopes.len(), curr_scope));
        self.scopes.len() - 1
    }

    pub fn get_parent_scope(&mut self, curr_scope: usize) -> usize {
        self.scopes[curr_scope].parent_scope
    }

    pub fn add_declaration(&mut self, symbol_decl: SymbolDecl) -> Result<(), SemanticErrorType> {
        self.insert_decl(symbol_decl.clone())?;
        self.scopes[symbol_decl.scope].add_declaration(symbol_decl);
        Ok(())
    }

    pub fn add_decl_new_scope(&mut self, symbol_decl: SymbolDecl, new_scope: usize) -> Result<(), SemanticErrorType> {
        self.insert_decl(symbol_decl.clone())?;
        self.scopes[symbol_decl.scope].add_decl_new_scope(symbol_decl, new_scope);
        Ok(())
    }

    pub fn add_type_tree(&mut self, tree: StatementTree, scope: usize) {
        self.scopes[scope].symbols.push(SymbolEntry::StatementTree(tree));
    }

    fn insert_decl(&mut self, decl: SymbolDecl) -> Result<usize, SemanticErrorType> {
        let mut low: usize = 0;
        let mut high: usize = self.decls.len();

        while low < high {
            let index: usize = (high + low) / 2;
            if self.decls[index].name == decl.name {
                if self.decls[index].scope == decl.scope {
                    return Err(SemanticErrorType::DuplicateDeclaration(
                        format!("Symbol {} already declared in scope {}", decl.name, decl.scope)
                    ));
                } else if self.decls[index].scope < decl.scope {
                    low = index + 1;
                } else {
                    high = index;
                }
            } else if self.decls[index].name < decl.name {
                low = index + 1;
            } else {
                high = index;
            }
        }

        self.decls.insert(low, decl);
        Ok(low)
    }

    pub fn get_scope(&self, scope: usize) -> &SymbolScope {
        &self.scopes[scope]
    }

    pub fn find_decl(&self, name: &String, curr_scope: usize) -> Option<&SymbolDecl> {
        let mut low: usize = 0;
        let mut high: usize = self.decls.len();

        let mut greatest_scope: Option<usize> = None;
        let mut greatest_index: Option<usize> = None;
        while low < high {
            let index: usize = (high + low) / 2;
            if self.decls[index].name == *name {
                if self.decls[index].scope == curr_scope {
                    return Some(&self.decls[index]);
                } else if self.decls[index].scope < curr_scope {
                    if let Some(greatest_scope) = &mut greatest_scope {
                        if self.decls[index].scope > *greatest_scope {
                            *greatest_scope = self.decls[index].scope;
                            greatest_index = Some(index);
                        }
                    } else {
                        greatest_scope = Some(self.decls[index].scope);
                        greatest_index = Some(index);
                    }
                    low = index + 1;
                } else {
                    high = index;
                }
            } else if self.decls[index].name < *name {
                low = index + 1;
            } else {
                high = index;
            }
        }
        
        match greatest_scope {
            Some(_) => Some(&self.decls[greatest_index.unwrap()]),
            None => None,
        }
    }
}

impl Loggable for SymbolTable {
    fn to_log_message(&self) -> String {
        let mut msg = String::new();
        let mut scope_stack: Vec<usize> = vec![GLOBAL_SCOPE];
        let mut return_stack: Vec<usize> = Vec::new();
        let mut tabs = 0;

        while !scope_stack.is_empty() {
            let mut curr_symbol = 0;
            if !return_stack.is_empty() && return_stack.len() == scope_stack.len() {
                curr_symbol = return_stack.pop().unwrap();
            } else {
                msg.push_str(format!("{}{{\n", "\t".repeat(tabs)).as_str());
                tabs += 1;
            }

            let curr_scope = scope_stack.pop().unwrap();
            let scope = &self.scopes[curr_scope];

            while curr_symbol < scope.symbols.len() {
                let symbol = &scope.symbols[curr_symbol];
                curr_symbol += 1;

                match symbol {
                    SymbolEntry::Decl(decl) => {
                        msg.push_str(format!("{}{}", "\t".repeat(tabs), decl.to_log_message()).as_str());
                    },
                    SymbolEntry::Scope(new_scope) => {
                        scope_stack.push(curr_scope);
                        return_stack.push(curr_symbol);
                        scope_stack.push(*new_scope);
                        break;
                    },
                    _ => {},
                }
            }

            if curr_symbol == scope.symbols.len() {
                tabs -= 1;
                msg.push_str(format!("{}}}\n", "\t".repeat(tabs)).as_str());
            }
        }

        msg
    }
}