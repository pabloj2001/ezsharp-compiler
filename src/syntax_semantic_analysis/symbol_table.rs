use crate::{lexical_analysis::Token, logger::Loggable};

use super::{
    semantic_analysis::SemanticErrorType,
    statement_tree::{StatementNode, StatementSymbol, StatementTree},
    symbol_declaration::{
        DeclId,
        SymbolDecl
    }
};

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
pub struct AssignmentInfo {
    var: DeclId,
    index: Option<StatementTree>,
    assignment: StatementTree,
}

#[derive(Debug, Clone)]
pub enum ConditionalStatementType {
    If,
    Else,
    While,
}

#[derive(Debug)]
pub struct ConditionalStatement {
    pub statement_type: ConditionalStatementType,
    pub condition: Option<StatementTree>,
    pub body_scope: usize,
}

#[derive(Debug)]
pub enum BuiltInFuncType {
    Print,
    Return,
}

#[derive(Debug)]
pub struct BuiltInFunc {
    pub func_type: BuiltInFuncType,
    pub statement: StatementTree,
}

#[derive(Debug)]
pub enum SymbolEntry {
    Decl(DeclId),
    Scope(usize),
    Parameter(DeclId),
    StatementTree(StatementTree),
    Assignment(AssignmentInfo),
    ConditionalStatement(ConditionalStatement),
    BuiltInFunction(BuiltInFunc),
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

    pub fn add_declaration(&mut self, id: DeclId) {
        self.symbols.push(SymbolEntry::Decl(id));
    }

    pub fn add_decl_new_scope(&mut self, id: DeclId, new_scope: usize) {
        self.symbols.push(SymbolEntry::Decl(id));
        self.symbols.push(SymbolEntry::Scope(new_scope));
    }

    pub fn add_parameter(&mut self, id: DeclId) {
        self.symbols.push(SymbolEntry::Parameter(id));
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

    pub fn get_scope(&self, scope: usize) -> &SymbolScope {
        &self.scopes[scope]
    }

    pub fn get_parent_scope(&mut self, curr_scope: usize) -> usize {
        self.scopes[curr_scope].parent_scope
    }

    pub fn add_declaration(&mut self, symbol_decl: SymbolDecl) -> Result<(), SemanticErrorType> {
        self.insert_decl(symbol_decl.clone())?;
        self.scopes[symbol_decl.scope].add_declaration(symbol_decl.get_id());
        Ok(())
    }

    pub fn add_decl_new_scope(&mut self, symbol_decl: SymbolDecl, new_scope: usize) -> Result<(), SemanticErrorType> {
        self.insert_decl(symbol_decl.clone())?;
        self.scopes[symbol_decl.scope].add_decl_new_scope(symbol_decl.get_id(), new_scope);
        Ok(())
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
        
        match greatest_index {
            Some(_) => Some(&self.decls[greatest_index.unwrap()]),
            None => None,
        }
    }

    pub fn find_decl_by_id(&self, id: &DeclId) -> Option<&SymbolDecl> {
        self.find_decl(&id.0, id.1)
    }

    pub fn add_parameter(&mut self, symbol_decl: SymbolDecl) -> Result<(), SemanticErrorType> {
        self.insert_decl(symbol_decl.clone())?;
        self.scopes[symbol_decl.scope].add_parameter(symbol_decl.get_id());
        Ok(())
    }

    pub fn add_type_tree(&mut self, tree: StatementTree, scope: usize) {
        self.scopes[scope].symbols.push(SymbolEntry::StatementTree(tree));
    }

    pub fn add_assignment(&mut self, var: SymbolDecl, index: Option<StatementTree>, assignment: StatementTree) {
        self.scopes[var.scope].symbols.push(
            SymbolEntry::Assignment(
                AssignmentInfo {
                    var: var.get_id(),
                    index,
                    assignment,
                }
            )
        );
    }

    pub fn add_conditional_statement(&mut self, statement: ConditionalStatement, curr_scope: usize) {
        self.scopes[curr_scope].symbols.push(
            SymbolEntry::ConditionalStatement(statement)
        );
    }

    pub fn add_builtin_func(&mut self, func: BuiltInFunc, curr_scope: usize) {
        self.scopes[curr_scope].symbols.push(
            SymbolEntry::BuiltInFunction(func)
        );
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
                    SymbolEntry::Decl(decl_id) | SymbolEntry::Parameter(decl_id) => {
                        if let Some(decl) = self.find_decl_by_id(decl_id) {
                            msg.push_str(format!("{}{}", "\t".repeat(tabs), decl.to_log_message()).as_str());
                        }
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