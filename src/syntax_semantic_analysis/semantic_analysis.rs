use crate::lexical_analysis::Token;
use crate::logger::Loggable;

use super::{statement_tree::{StatementSymbol, StatementTree, StatementTreeInfo}, symbol_declaration::{BasicType, SymbolDecl}, symbol_table::{ScopeType, SymbolTable, GLOBAL_SCOPE}};

#[derive(Debug)]
pub enum SemanticErrorType {
    UndefinedVariable(String),
    TypeMismatch(String),
    InvalidType(String),
    DuplicateDeclaration(String),
    MissingParameters(String),
}

pub struct SemanticError {
    error_type: SemanticErrorType,
    line: usize,
}

impl SemanticError {
    pub fn new(error_type: SemanticErrorType, line: usize) -> Self {
        Self {
            error_type,
            line,
        }
    }
}

impl Loggable for SemanticError {
    fn to_log_message(&self) -> String {
        match &self.error_type {
            SemanticErrorType::DuplicateDeclaration(id) => format!("Duplicate declaration on line {}: {}", self.line, id),
            SemanticErrorType::UndefinedVariable(id) => format!("Undefined variable on line {}: {}", self.line, id),
            SemanticErrorType::TypeMismatch(types_comp) => format!("Type mismatch on line {}: {}", self.line, types_comp),
            SemanticErrorType::InvalidType(msg) => format!("Invalid type on line {}: {}", self.line, msg),
            SemanticErrorType::MissingParameters(msg) => format!("Missing parameters on line {}: {}", self.line, msg),
        }
    }
}

impl Loggable for Box<[SemanticError]> {
    fn to_log_message(&self) -> String {
        let mut msg = String::new();
        for error in self.iter() {
            msg.push_str(&error.to_log_message());
            msg.push('\n');
        }
        msg
    }
}

#[derive(Debug, Clone)]
pub enum SemanticAction {
    SetFunc,
    AddFuncDecl,
    NewScope,
    PopScope,
    AddParam,
    SetType,
    AddVarDecl,
    ClearVarDecl,
    StartTypeTree,
    CheckVarType,
    CheckType,
    SplitTree,
    AddOperator,
    AddTypeTree,
    SetLiteral,
    AddFuncCheck,
    PopFuncCheck,
    CheckParamType,
    CheckIndexType,
    SetIsArray,
    SetId,
    CheckReturnType,
}

#[derive(Debug)]
struct FuncCheck {
    func_decl: SymbolDecl,
    param_index: usize,
    params: Vec<StatementTree>,
}

pub struct SemanticInfo {
    pub symbol_table: SymbolTable,
    pub curr_scope: usize,
    pub curr_id: Option<String>,
    pub curr_type: Option<BasicType>,
    pub is_array: bool,
    pub curr_func: Option<SymbolDecl>,
    pub func_scope: usize,
    func_return_type: Option<(BasicType, bool)>,
    pub type_trees: Vec<StatementTreeInfo>,
    func_checks: Vec<FuncCheck>,
    check_return_type: bool,
}

impl SemanticInfo {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            curr_scope: GLOBAL_SCOPE,
            curr_id: None,
            curr_type: None,
            is_array: false,
            curr_func: None,
            func_scope: 0,
            func_return_type: None,
            type_trees: Vec::new(),
            func_checks: Vec::new(),
            check_return_type: false,
        }
    }

    pub fn perform_action(&mut self, action: &SemanticAction, prev_terminal: &Option<Token>) -> Result<(), SemanticErrorType> {
        match action {
            SemanticAction::SetId => {
                if let Some(Token::Identifier(id)) = prev_terminal {
                    if let Some(tree_info) = self.type_trees.last_mut() {
                        let symbol_found = self.symbol_table.find_decl(id, self.curr_scope);
                        if let Some(symbol) = symbol_found {
                            let new_node = tree_info.tree.add_node(
                                StatementSymbol::Decl(symbol.clone()),
                                tree_info.curr_node
                            );

                            tree_info.tree.nodes[new_node].node_type = Some(symbol.var_type.0.clone());

                            if let Some(curr_node) = tree_info.curr_node {
                                if let StatementSymbol::SingleChildOperator(_) = tree_info.tree.nodes[curr_node].symbol {
                                    tree_info.curr_node = Some(new_node);
                                }
                            } else {
                                tree_info.curr_node = Some(new_node);
                            }
                        } else {
                            return Err(SemanticErrorType::UndefinedVariable(id.clone()));
                        }
                    } else {
                        self.curr_id = Some(id.clone());
                        self.is_array = false;
                    }
                }
            },
            SemanticAction::SetType => {
                if let Some(token) = prev_terminal {
                    match token {
                        Token::Kint => self.curr_type = Some(BasicType::Int),
                        Token::Kdouble => self.curr_type = Some(BasicType::Double),
                        _ => {}
                    }
                }
            },
            SemanticAction::SetIsArray => {
                self.is_array = true;
            },
            SemanticAction::SetLiteral => {
                if let Some(token) = prev_terminal {
                    if let Some(tree_info) = self.type_trees.last_mut() {
                        let new_node = tree_info.tree.add_node(
                            StatementSymbol::Literal(token.clone()),
                            tree_info.curr_node
                        );

                        match token {
                            Token::Tint(_) => tree_info.tree.nodes[new_node].node_type = Some(BasicType::Int),
                            Token::Tdouble(_) => tree_info.tree.nodes[new_node].node_type = Some(BasicType::Double),
                            _ => {}
                        }

                        if let Some(curr_node) = tree_info.curr_node {
                            if let StatementSymbol::SingleChildOperator(_) = tree_info.tree.nodes[curr_node].symbol {
                                tree_info.curr_node = Some(new_node);
                            }
                        } else {
                            tree_info.curr_node = Some(new_node);
                        }
                    }
                }
            },
            SemanticAction::AddVarDecl => {
                if let Some(id) = &self.curr_id {
                    if let Some(basic_type) = &self.curr_type {
                        // Add variable to symbol table
                        self.symbol_table.add_declaration(SymbolDecl::new(
                            id.clone(),
                            basic_type.clone(),
                            self.is_array,
                            self.curr_scope,
                        ))?;
                    }
                }
            },
            SemanticAction::ClearVarDecl => {
                self.curr_id = None;
                self.curr_type = None;
            },
            SemanticAction::NewScope => {
                self.curr_scope = self.symbol_table.add_scope(ScopeType::Local, self.curr_scope);
            },
            SemanticAction::PopScope => {
                self.curr_scope = self.symbol_table.get_parent_scope(self.curr_scope);
            },
            SemanticAction::SetFunc => {
                if let Some(Token::Identifier(id)) = prev_terminal {
                    if let Some(basic_type) = &self.curr_type {
                        // Create new scope for function
                        self.func_scope = self.symbol_table.add_scope(ScopeType::Function, self.curr_scope);

                        // Set current function
                        self.curr_func = Some(SymbolDecl::new_func(
                            id.clone(),
                            basic_type.clone(),
                            false,
                            self.curr_scope,
                        ));
                    }
                }
            },
            SemanticAction::AddParam => {
                if let Some(func) = &mut self.curr_func {
                    if let SymbolDecl{ var_type: (BasicType::Function(func_info), _), .. } = func {
                        if let Some(basic_type) = &self.curr_type {
                            if let Some(id) = &self.curr_id {
                                // Add parameter type to current function
                                func_info.param_types.push(basic_type.clone());

                                // Add parameter to function scope
                                self.symbol_table.add_declaration(SymbolDecl::new(
                                    id.clone(),
                                    basic_type.clone(),
                                    self.is_array,
                                    self.func_scope,
                                ))?;
                            }
                        }
                    }
                }
            },
            SemanticAction::AddFuncDecl => {
                if let Some(func) = &self.curr_func {
                    // Add function to symbol table
                    self.symbol_table.add_decl_new_scope(func.clone(), self.func_scope)?;

                    // Set current scope to function scope
                    self.curr_scope = self.func_scope;

                    // Reset current function
                    self.func_return_type = Some(func.var_type.clone());
                    self.curr_func = None;
                    self.func_scope = GLOBAL_SCOPE;
                }
            },
            SemanticAction::StartTypeTree => {
                self.type_trees.push(StatementTreeInfo::new());
            },
            SemanticAction::AddTypeTree => {
                if let Some(func_check) = self.func_checks.last_mut() {
                    if let Some(tree_info) = self.type_trees.pop() {
                        func_check.params.push(tree_info.tree);
                    }
                } else {
                    if let Some(tree_info) = self.type_trees.pop() {                        
                        if self.check_return_type {
                            self.check_return_type = false;
                            if let Some(func_return_type) = &self.func_return_type {
                                if let Some(node) = tree_info.tree.nodes.last() {
                                    if let Some(node_type) = &node.node_type {
                                        if let BasicType::Function(func_info) = &func_return_type.0 {
                                            if *func_info.return_type != *node_type {
                                                return Err(SemanticErrorType::TypeMismatch(
                                                    format!("Wrong return type, {} != {}", *func_info.return_type, node_type)
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        self.symbol_table.add_type_tree(tree_info.tree, self.curr_scope);
                    }
                }
            },
            SemanticAction::SplitTree => {
                if let Some(tree_info) = self.type_trees.last_mut() {
                    if let Some(token) = prev_terminal {
                        if let Some(mut node) = tree_info.curr_node {
                            if let StatementSymbol::SingleChildOperator(_) = tree_info.tree.nodes[node].symbol {
                                // If current node is a single child operator, split the tree on the left child
                                node = tree_info.tree.nodes[node].left.unwrap();
                            } else if tree_info.tree.nodes[node].has_both_children() {
                                // If current node has both children, split the tree on the right child
                                node = tree_info.tree.nodes[node].right.unwrap();
                            }

                            tree_info.curr_node = tree_info.tree.split_tree(
                                StatementSymbol::Operator(token.clone()),
                                node
                            ).into();
                        } else {
                            if let Some(start_node) = tree_info.tree.start {
                                // Create new node as start node
                                tree_info.curr_node = tree_info.tree.add_node(
                                    StatementSymbol::Operator(token.clone()),
                                    None
                                ).into();
                                
                                // Add prev start node as left child
                                tree_info.tree.nodes[tree_info.curr_node.unwrap()].left = Some(start_node);
                            }
                        }
                    }
                }
            },
            SemanticAction::AddOperator => {
                if let Some(tree_info) = self.type_trees.last_mut() {
                    if let Some(token) = prev_terminal {
                        tree_info.curr_node = tree_info.tree.add_node(
                            StatementSymbol::SingleChildOperator(token.clone()),
                            tree_info.curr_node
                        ).into();
                    }
                }
            },
            SemanticAction::CheckType => {
                self.check_type()?;
            },
            SemanticAction::CheckVarType => {
                self.check_var_type()?;
            },
            SemanticAction::AddFuncCheck => {
                if let Some(tree_info) = self.type_trees.last_mut() {
                    if let Some(node) = tree_info.tree.nodes.pop() {
                        if let StatementSymbol::Decl(func_decl) = node.symbol {
                            if let BasicType::Function(_) = func_decl.var_type.0 {
                                self.func_checks.push(FuncCheck {
                                    func_decl: func_decl,
                                    param_index: 0,
                                    params: Vec::new(),
                                });
                            } else {
                                self.type_trees.pop();
                                return Err(SemanticErrorType::InvalidType(
                                    format!("{} is not a function", func_decl.name)
                                ));
                            }
                        }
                    }
                }
            },
            SemanticAction::PopFuncCheck => {
                if let Some(tree_info) = self.type_trees.last_mut() {
                    if let Some(func_check) = self.func_checks.pop() {
                        if let BasicType::Function(func_info) = &func_check.func_decl.var_type.0 {
                            if func_info.param_types.len() != func_check.param_index {
                                return Err(SemanticErrorType::MissingParameters(
                                    format!("Expected {} parameters, found {}", func_info.param_types.len(), func_check.param_index)
                                ));
                            }

                            tree_info.curr_node = tree_info.tree.add_node(
                                StatementSymbol::FunctionCall(func_check.func_decl.clone(), func_check.params),
                                tree_info.curr_node
                            ).into();
                            tree_info.tree.nodes[tree_info.curr_node.unwrap()].node_type = Some(*func_info.return_type.clone());
                        }
                    }
                }
            },
            SemanticAction::CheckParamType => {
                let mut err: Option<SemanticErrorType> = None;
                if let Some(func_check) = self.func_checks.last_mut() {
                    if let BasicType::Function(func_info) = &func_check.func_decl.var_type.0 {
                        if let Some(tree_info) = self.type_trees.last() {
                            if let Some(node) = tree_info.tree.start {
                                if let Some(node_type) = &tree_info.tree.nodes[node].node_type {
                                    if func_info.param_types[func_check.param_index] != *node_type {
                                        if let BasicType::Function(func_info) = &func_check.func_decl.var_type.0 {
                                            err = Some(SemanticErrorType::TypeMismatch(
                                                format!("{} != {}", func_info.param_types[func_check.param_index], node_type)
                                            ));
                                        } else {
                                            err = Some(SemanticErrorType::TypeMismatch(
                                                format!("{} != {}", func_check.func_decl.var_type.0, node_type)
                                            ));
                                        }
                                    } else {
                                        func_check.param_index += 1;
                                    }
                                } else {
                                    err = Some(SemanticErrorType::InvalidType(
                                        "Type not found".to_string()
                                    ));
                                }
                            }
                        }
                    }
                }

                if let Some(error) = err {
                    self.type_trees.pop();
                    self.func_checks.pop();
                    return Err(error);
                }
            },
            SemanticAction::CheckReturnType => {
                self.check_return_type = true;
            },
            _ => {}
        }
        Ok(())
    }

    fn check_type(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(tree_info) = self.type_trees.last_mut() {
            if let Some(mut node) = tree_info.curr_node {
                while match tree_info.tree.nodes[node].symbol {
                    StatementSymbol::Operator(_) => false,
                    StatementSymbol::SingleChildOperator(_) => false,
                    _ => true,
                } {
                    tree_info.curr_node = tree_info.tree.nodes[node].parent;
                    if tree_info.curr_node.is_none() {
                        break;
                    }
                    node = tree_info.curr_node.unwrap();
                }

                if let StatementSymbol::Operator(_) = tree_info.tree.nodes[node].symbol {
                    if let Some(left) = tree_info.tree.nodes[node].left {
                        if let Some(right) = tree_info.tree.nodes[node].right {
                            // Check if left and right nodes have the same type
                            let left_type = tree_info.tree.nodes[left].node_type.clone();
                            let right_type = tree_info.tree.nodes[right].node_type.clone();
    
                            if left_type.is_some() && right_type.is_some() {
                                let left_type = left_type.unwrap();
                                let right_type = right_type.unwrap();
                                if left_type != right_type {
                                    self.type_trees.pop();
                                    return Err(SemanticErrorType::TypeMismatch(
                                        format!("{} != {}", left_type, right_type)
                                    ));
                                }
    
                                // Set current node's type
                                tree_info.tree.nodes[node].node_type = left_type.clone().into();
                            } else {
                                self.type_trees.pop();
                                return Err(SemanticErrorType::InvalidType(
                                    "Type not found".to_string()
                                ));
                            }
                        }
                    }
                    tree_info.curr_node = tree_info.tree.nodes[node].parent;
                } else if let StatementSymbol::SingleChildOperator(_) = tree_info.tree.nodes[node].symbol {
                    if let Some(left) = tree_info.tree.nodes[node].left {
                        let left_type = tree_info.tree.nodes[left].node_type.clone();
                        if left_type.is_some() {
                            tree_info.tree.nodes[node].node_type = left_type.clone().into();
                        } else {
                            self.type_trees.pop();
                            return Err(SemanticErrorType::InvalidType(
                                "Type not found".to_string()
                            ));
                        }
                    }
                }
                tree_info.curr_node = tree_info.tree.nodes[node].parent;
            }
        }
        Ok(())
    }

    fn check_var_type(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(tree_info) = self.type_trees.last_mut() {
            if let Some(node) = tree_info.tree.start {
                if tree_info.tree.nodes[node].node_type.is_none() || self.curr_id.is_none() {
                    self.type_trees.pop();
                    return Err(SemanticErrorType::InvalidType(
                        "Type not found".to_string()
                    ));
                }

                let node_type = tree_info.tree.nodes[node].node_type.clone().unwrap();
                let decl = self.symbol_table.find_decl(
                    self.curr_id.as_ref().unwrap(),
                    self.curr_scope
                );

                if let Some(var_type) = decl {
                    if var_type.var_type.0 != node_type {
                        self.type_trees.pop();
                        return Err(SemanticErrorType::TypeMismatch(
                            format!("{} != {}", node_type, var_type.var_type.0)
                        ));
                    }
                } else {
                    self.type_trees.pop();
                    return Err(SemanticErrorType::UndefinedVariable(
                        self.curr_id.as_ref().unwrap().clone()
                    ));
                }
            }
        }
        Ok(())
    }
}