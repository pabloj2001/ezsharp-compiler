use crate::lexical_analysis::Token;
use crate::logger::Loggable;
use super::semantic_actions::SemanticAction;

use super::symbol_declaration::DeclId;
use super::symbol_table::{BuiltInFunc, BuiltInFuncType, ConditionalStatement, ConditionalStatementType};
use super::{
    statement_tree::{
        StatementSymbol,
        StatementTree,
        StatementTreeInfo
    },
    symbol_declaration::{
        BasicType,
        SymbolDecl
    },
    symbol_table::{
        ScopeType,
        SymbolTable,
        GLOBAL_SCOPE
    }
};

#[derive(Debug)]
pub enum SemanticErrorType {
    UndefinedVariable(String),
    TypeMismatch(String),
    InvalidType(String),
    DuplicateDeclaration(String),
    MissingParameters(String),
    InvalidArraySize(String),
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
            SemanticErrorType::InvalidArraySize(msg) => format!("Invalid array size on line {}: {}", self.line, msg),
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

#[derive(Debug)]
struct FuncCheck {
    func_id: DeclId,
    param_index: usize,
    params: Vec<StatementTree>,
}

struct PartialConditionalStatement {
    statement_type: ConditionalStatementType,
    condition: Option<StatementTree>,
    body_scope: Option<usize>,
}

impl PartialConditionalStatement {
    fn to_conditional_statement(&self) -> Option<ConditionalStatement> {
        if let Some(body_scope) = self.body_scope {
            return Some(ConditionalStatement {
                statement_type: self.statement_type.clone(),
                condition: self.condition.clone(),
                body_scope,
            });
        }
        None
    }
}

pub struct SemanticInfo {
    pub symbol_table: SymbolTable,
    curr_scope: usize,
    curr_id: Option<String>,
    curr_type: Option<BasicType>,
    curr_func: Option<SymbolDecl>,
    func_scope: usize,
    func_return_type: Option<BasicType>,
    type_trees: Vec<StatementTreeInfo>,
    func_checks: Vec<FuncCheck>,
    check_return_type: bool,
    build_assignment: bool,
    curr_var: Option<SymbolDecl>,
    curr_conditional_statements: Vec<PartialConditionalStatement>,
    curr_builtin_func: Option<BuiltInFuncType>,
    curr_array_index: Option<StatementTree>,
}

impl SemanticInfo {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            curr_scope: GLOBAL_SCOPE,
            curr_id: None,
            curr_type: None,
            curr_func: None,
            func_scope: 0,
            func_return_type: None,
            type_trees: Vec::new(),
            func_checks: Vec::new(),
            check_return_type: false,
            build_assignment: false,
            curr_var: None,
            curr_conditional_statements: Vec::new(),
            curr_builtin_func: None,
            curr_array_index: None,
        }
    }

    pub fn perform_action(&mut self, action: &SemanticAction, prev_terminal: &Option<Token>) -> Result<(), SemanticErrorType> {
        match action {
            SemanticAction::SetId => {
                self.set_id(prev_terminal)?;
            },
            SemanticAction::SetType => {
                self.set_type(prev_terminal);
            },
            SemanticAction::SetArray => {
                self.set_array()?;
            },
            SemanticAction::SetLiteral => {
                self.set_literal(prev_terminal);
            },
            SemanticAction::AddVarDecl => {
                self.add_var_decl()?;
            },
            SemanticAction::ClearVarDecl => {
                self.curr_id = None;
                self.curr_type = None;
            },
            SemanticAction::NewScope => {
                self.new_scope();
            },
            SemanticAction::PopScope => {
                self.pop_scope();
            },
            SemanticAction::SetFunc => {
                self.set_func(prev_terminal);
            },
            SemanticAction::AddParam => {
                self.add_param()?;
            },
            SemanticAction::AddFuncDecl => {
                self.add_func_decl()?;
            },
            SemanticAction::StartTypeTree => {
                self.type_trees.push(StatementTreeInfo::new());
            },
            SemanticAction::AddTypeTree => {
                self.add_type_tree()?;
            },
            SemanticAction::SplitTree => {
                self.split_trees(prev_terminal);
            },
            SemanticAction::AddOperator => {
                self.add_operator(prev_terminal);
            },
            SemanticAction::CheckType => {
                self.check_type()?;
            },
            SemanticAction::CheckVarType => {
                self.check_var_type()?;
            },
            SemanticAction::AddFuncCheck => {
                self.add_func_check()?;
            },
            SemanticAction::PopFuncCheck => {
                self.pop_func_check()?;
            },
            SemanticAction::CheckParamType => {
                self.check_param_type()?;
            },
            SemanticAction::StartAssignment => {
                self.build_assignment = true;
            },
            SemanticAction::AddAssignment => {
                self.add_assignment();
            },
            SemanticAction::AddCondition => {
                self.add_condition();
            },
            SemanticAction::AddCondStatement => {
                self.add_conditional_statement();
            },
            SemanticAction::StartIf => {
                self.start_conditional_statement(ConditionalStatementType::If);
            },
            SemanticAction::StartElse => {
                self.start_conditional_statement(ConditionalStatementType::Else);
            },
            SemanticAction::StartWhile => {
                self.start_conditional_statement(ConditionalStatementType::While);
            },
            SemanticAction::StartReturn => {
                self.check_return_type = true;
                self.curr_builtin_func = Some(BuiltInFuncType::Return);
            },
            SemanticAction::StartPrint => {
                self.curr_builtin_func = Some(BuiltInFuncType::Print);
            },
        }
        Ok(())
    }

    fn set_id(&mut self, prev_terminal: &Option<Token>) -> Result<(), SemanticErrorType> {
        if let Some(Token::Identifier(id)) = prev_terminal {
            if let Some(tree_info) = self.type_trees.last_mut() {
                let symbol_found = self.symbol_table.find_decl(id, self.curr_scope);
                if let Some(symbol) = symbol_found {
                    let new_node = tree_info.tree.add_node(
                        StatementSymbol::Decl(symbol.get_id()),
                        tree_info.curr_node
                    );

                    tree_info.tree.nodes[new_node].node_type = Some(symbol.var_type.clone());

                    if let Some(curr_node) = tree_info.curr_node {
                        if let StatementSymbol::SingleChildOperator(_) = tree_info.tree.nodes[curr_node].symbol {
                            tree_info.curr_node = Some(new_node);
                        }
                    } else {
                        tree_info.curr_node = Some(new_node);
                    }
                } else {
                    self.type_trees.pop();
                    return Err(SemanticErrorType::UndefinedVariable(id.clone()));
                }
            } else {
                self.curr_id = Some(id.clone());
                if self.build_assignment && self.curr_var.is_none() {
                    if let Some(decl) = self.symbol_table.find_decl(id, self.curr_scope) {
                        self.curr_var = Some(decl.clone());
                    } else {
                        return Err(SemanticErrorType::UndefinedVariable(id.clone()));
                    }
                }
            }
        }
        Ok(())
    }

    fn set_type(&mut self, prev_terminal: &Option<Token>) {
        if let Some(token) = prev_terminal {
            match token {
                Token::Kint => self.curr_type = Some(BasicType::Int),
                Token::Kdouble => self.curr_type = Some(BasicType::Double),
                _ => {}
            }
        }
    }

    fn set_array(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(index_tree_info) = self.type_trees.pop() {
            // Check that index type is Int
            if let Some(start) = index_tree_info.tree.start {
                if let Some(node_type) = &index_tree_info.tree.nodes[start].node_type {
                    if *node_type != BasicType::Int {
                        return Err(SemanticErrorType::TypeMismatch(
                            format!("Array index type must be int, found {}", node_type)
                        ));
                    }
                }
            }

            if self.type_trees.len() > 0 {
                // Using array value in expression
                let mut err: Option<Result<(), SemanticErrorType>> = None;
                if let Some(arr_tree_info) = self.type_trees.last_mut() {
                    if arr_tree_info.tree.nodes.len() > 0 {
                        let curr_node = arr_tree_info.tree.nodes.len() - 1;
                        if let StatementSymbol::Decl(id) = &arr_tree_info.tree.nodes[curr_node].symbol {
                            if let Some(decl) = self.symbol_table.find_decl_by_id(id) {
                                if let BasicType::Array(inner_type, _) = &decl.var_type {
                                    arr_tree_info.tree.nodes[curr_node].symbol =
                                        StatementSymbol::ArrayAccess(id.clone(), index_tree_info.tree);
                                    arr_tree_info.tree.nodes[curr_node].node_type = Some(*inner_type.clone());
                                } else {
                                    err = Some(Err(SemanticErrorType::InvalidType(
                                        format!("Variable {} is not an array", id.0.clone())
                                    )));
                                }
                            }
                        }
                    }
                }

                if let Some(error) = err {
                    self.type_trees.pop();
                    return error;
                }
            } else {
                if self.build_assignment {
                    // Assigning value to array
                    if let Some(var) = &self.curr_var {
                        if let BasicType::Array(arr_type, _) = &var.var_type {
                            self.curr_var = Some(SymbolDecl::new(
                                var.name.clone(),
                                *arr_type.clone(),
                                var.scope,
                            ));
                            self.curr_array_index = Some(index_tree_info.tree);
                        } else {
                            return Err(SemanticErrorType::InvalidType(
                                "Variable is not an array".to_string()
                            ));
                        }
                    }
                } else {
                    // Defining array
                    if let Some(basic_type) = &self.curr_type {
                        let size = index_tree_info.tree
                                .calculate_array_size()
                                .map_err(|e| SemanticErrorType::InvalidArraySize(e))?;
                        self.curr_type = Some(
                            BasicType::Array(
                                Box::new(basic_type.clone()),
                                size,
                            )
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn set_literal(&mut self, prev_terminal: &Option<Token>) {
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
    }

    fn add_var_decl(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(id) = &self.curr_id {
            if let Some(basic_type) = &self.curr_type {
                // Add variable to symbol table
                self.symbol_table.add_declaration(SymbolDecl::new(
                    id.clone(),
                    basic_type.clone(),
                    self.curr_scope,
                ))?;

                // Go back to regular type after array assignment
                if let BasicType::Array(old_type, _) = basic_type {
                    self.curr_type = Some(*old_type.clone());
                }
            }
        }
        Ok(())
    }

    fn new_scope(&mut self) {
        self.curr_scope = self.symbol_table.add_scope(ScopeType::Local, self.curr_scope);
        if self.curr_conditional_statements.len() > 0 {
            if let Some(cond_info) = self.curr_conditional_statements.last_mut() {
                if cond_info.body_scope.is_none() {
                    cond_info.body_scope = Some(self.curr_scope);
                }
            }
        }
    }

    fn pop_scope(&mut self) {
        self.curr_scope = self.symbol_table.get_parent_scope(self.curr_scope);
        self.func_return_type = None;
    }

    fn set_func(&mut self, prev_terminal: &Option<Token>) {
        if let Some(Token::Identifier(id)) = prev_terminal {
            if let Some(basic_type) = &self.curr_type {
                // Create new scope for function
                self.func_scope = self.symbol_table.add_scope(ScopeType::Function, self.curr_scope);

                // Set current function
                self.curr_func = Some(SymbolDecl::new_func(
                    id.clone(),
                    basic_type.clone(),
                    self.func_scope,
                    self.curr_scope,
                ));
            }
        }
    }

    fn add_param(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(func) = &mut self.curr_func {
            if let SymbolDecl{ var_type: BasicType::Function(func_info), .. } = func {
                if let Some(basic_type) = &self.curr_type {
                    if let Some(id) = &self.curr_id {
                        // Add parameter type to current function
                        func_info.param_types.push(basic_type.clone());

                        // Add parameter to function scope
                        self.symbol_table.add_parameter(SymbolDecl::new(
                            id.clone(),
                            basic_type.clone(),
                            self.func_scope,
                        ))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn add_func_decl(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(func) = &self.curr_func {
            // Add function to symbol table
            self.symbol_table.add_declaration(func.clone())?;

            // Set current scope to function scope
            self.curr_scope = self.func_scope;

            // Reset current function
            self.func_return_type = Some(func.var_type.clone());
            self.curr_func = None;
            self.func_scope = GLOBAL_SCOPE;
        }
        Ok(())
    }

    fn add_type_tree(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(func_check) = self.func_checks.last_mut() {
            if let Some(tree_info) = self.type_trees.pop() {
                func_check.params.push(tree_info.tree);
            }
        } else {
            if let Some(tree_info) = self.type_trees.pop() {
                if let Some(builtin_type) = &self.curr_builtin_func {
                    match builtin_type {
                        BuiltInFuncType::Return => {
                            self.check_return(tree_info)?;
                        },
                        BuiltInFuncType::Print => {
                            self.check_print(tree_info);
                        }
                    }
                    self.curr_builtin_func = None;
                } else {
                    self.symbol_table.add_type_tree(tree_info.tree, self.curr_scope);
                }
            }
        }
        Ok(())
    }

    fn check_return(&mut self, tree_info: StatementTreeInfo) -> Result<(), SemanticErrorType> {
        if let Some(func_return_type) = &self.func_return_type {
            if let Some(node) = tree_info.tree.nodes.last() {
                if let Some(node_type) = &node.node_type {
                    if let BasicType::Function(func_info) = &func_return_type {
                        if *func_info.return_type != *node_type {
                            return Err(SemanticErrorType::TypeMismatch(
                                format!("Wrong return type, {} != {}", *func_info.return_type, node_type)
                            ));
                        }

                        self.symbol_table.add_builtin_func(
                            BuiltInFunc {
                                func_type: BuiltInFuncType::Return,
                                statement: tree_info.tree,
                            },
                            self.curr_scope,
                        );
                    }
                }
            }
        } else {
            return Err(SemanticErrorType::InvalidType(
                "Function return type not found".to_string()
            ));
        }
        Ok(())
    }

    fn check_print(&mut self, tree_info: StatementTreeInfo) {
        self.symbol_table.add_builtin_func(
            BuiltInFunc {
                func_type: BuiltInFuncType::Print,
                statement: tree_info.tree,
            },
            self.curr_scope,
        );
    }

    fn split_trees(&mut self, prev_terminal: &Option<Token>) {
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
    }

    fn add_operator(&mut self, prev_terminal: &Option<Token>) {
        if let Some(tree_info) = self.type_trees.last_mut() {
            if let Some(token) = prev_terminal {
                tree_info.curr_node = tree_info.tree.add_node(
                    StatementSymbol::SingleChildOperator(token.clone()),
                    tree_info.curr_node
                ).into();
            }
        }
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

                if let StatementSymbol::Operator(op) = &tree_info.tree.nodes[node].symbol {
                    if let Some(left) = tree_info.tree.nodes[node].left {
                        if let Some(right) = tree_info.tree.nodes[node].right {
                            // Check if left and right nodes have the same type
                            let left_type = tree_info.tree.nodes[left].node_type.clone();
                            let right_type = tree_info.tree.nodes[right].node_type.clone();
    
                            if left_type.is_some() && right_type.is_some() {
                                let left_type = left_type.unwrap();
                                let right_type = right_type.unwrap();
                                if *op != Token::Kand && *op != Token::Kor {
                                    if left_type != right_type {
                                        self.type_trees.pop();
                                        return Err(SemanticErrorType::TypeMismatch(
                                            format!("{} != {}", left_type, right_type)
                                        ));
                                    }
                                }
    
                                // Set current node's type
                                match op {
                                    Token::Kand |
                                    Token::Kor |
                                    Token::Oequal |
                                    Token::Onot |
                                    Token::Olt |
                                    Token::Ogt |
                                    Token::Olte |
                                    Token::Ogte => {
                                        tree_info.tree.nodes[node].node_type = Some(BasicType::Int);
                                    },
                                    _ => {
                                        tree_info.tree.nodes[node].node_type = left_type.clone().into();
                                    }
                                }
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
                if let Some(var_type) = &self.curr_var {
                    if var_type.var_type != node_type {
                        self.type_trees.pop();
                        return Err(SemanticErrorType::TypeMismatch(
                            format!("{} != {}", node_type, var_type.var_type)
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

    fn add_func_check(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(tree_info) = self.type_trees.last_mut() {
            if let Some(node) = tree_info.tree.nodes.pop() {
                if let StatementSymbol::Decl(func_id) = node.symbol {
                    if let Some(decl) = self.symbol_table.find_decl_by_id(&func_id) {
                        if let BasicType::Function(_) = decl.var_type {
                            self.func_checks.push(FuncCheck {
                                func_id,
                                param_index: 0,
                                params: Vec::new(),
                            });
                        } else {
                            self.type_trees.pop();
                            return Err(SemanticErrorType::InvalidType(
                                format!("{} is not a function", decl.name)
                            ));
                        }
                    } else {
                        self.type_trees.pop();
                        return Err(SemanticErrorType::UndefinedVariable(
                            func_id.0.clone()
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn pop_func_check(&mut self) -> Result<(), SemanticErrorType> {
        if let Some(tree_info) = self.type_trees.last_mut() {
            if let Some(func_check) = self.func_checks.pop() {
                if let Some(decl) = self.symbol_table.find_decl_by_id(&func_check.func_id) {
                    if let BasicType::Function(func_info) = &decl.var_type {
                        if func_info.param_types.len() != func_check.param_index {
                            return Err(SemanticErrorType::MissingParameters(
                                format!("Expected {} parameters, found {}", func_info.param_types.len(), func_check.param_index)
                            ));
                        }

                        tree_info.curr_node = tree_info.tree.add_node(
                            StatementSymbol::FunctionCall(func_check.func_id.clone(), func_check.params),
                            tree_info.curr_node
                        ).into();
                        tree_info.tree.nodes[tree_info.curr_node.unwrap()].node_type = Some(*func_info.return_type.clone());
                    }
                }
            }
        }
        Ok(())
    }

    fn check_param_type(&mut self) -> Result<(), SemanticErrorType> {
        let mut err: Option<SemanticErrorType> = None;
        if let Some(func_check) = self.func_checks.last_mut() {
            if let Some(decl) = self.symbol_table.find_decl_by_id(&func_check.func_id) {
                if let BasicType::Function(func_info) = &decl.var_type {
                    if let Some(tree_info) = self.type_trees.last() {
                        if let Some(node) = tree_info.tree.start {
                            if let Some(node_type) = &tree_info.tree.nodes[node].node_type {
                                if func_info.param_types[func_check.param_index] != *node_type {
                                    if let BasicType::Function(func_info) = &decl.var_type {
                                        err = Some(SemanticErrorType::TypeMismatch(
                                            format!("{} != {}", func_info.param_types[func_check.param_index], node_type)
                                        ));
                                    } else {
                                        err = Some(SemanticErrorType::TypeMismatch(
                                            format!("{} != {}", decl.var_type, node_type)
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
        }

        if let Some(error) = err {
            self.type_trees.pop();
            self.func_checks.pop();
            return Err(error);
        }
        Ok(())
    }

    fn add_assignment(&mut self) {
        if let Some(tree_info) = self.type_trees.pop() {
            if let Some(var) = &self.curr_var {
                if let Some(arr_index) = &self.curr_array_index {
                    self.symbol_table.add_assignment(var.clone(), Some(arr_index.clone()), tree_info.tree);
                    self.curr_array_index = None;
                } else {
                    self.symbol_table.add_assignment(var.clone(), None, tree_info.tree);
                }
            }
        }
        self.build_assignment = false;
        self.curr_var = None;
    }

    fn start_conditional_statement(&mut self, statement_type: ConditionalStatementType) {
        self.curr_conditional_statements.push(PartialConditionalStatement {
            statement_type,
            condition: None,
            body_scope: None,
        });
    }

    fn add_condition(&mut self) {
        if let Some(tree_info) = self.type_trees.pop() {
            if let Some(cond_info) = self.curr_conditional_statements.last_mut() {
                cond_info.condition = Some(tree_info.tree);
            }
        }
    }

    fn add_conditional_statement(&mut self) {
        self.pop_scope();
        if let Some(partial_statement) = &self.curr_conditional_statements.pop() {
            if let Some(statement) = partial_statement.to_conditional_statement() {
                self.symbol_table.add_conditional_statement(statement, self.curr_scope);
            }
        }
    }
}