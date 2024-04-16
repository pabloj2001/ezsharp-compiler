use crate::{
    lexical_analysis::Token,
    syntax_semantic_analysis::{
        statement_tree::{
            StatementSymbol,
            StatementTree
        },
        symbol_declaration::{
            to_var_name,
            BasicType,
            SymbolDecl
        },
        symbol_table::{
            BuiltInFunc,
            BuiltInFuncType
        }
    }
};

use super::tac::{
    TacCommand,
    TacOperation,
    TacProgram,
    TacStatement,
    TacValue
};

#[derive(Debug)]
pub struct TacFunctionInfo {
    pub name: String,
    pub statements: Vec<TacStatement>,
    pub var_sizes: Vec<(String, (u32, u32))>,
    pub params: Vec<(String, u32)>,
    pub label_count: u16,
    pub temp_count: u16,
}

#[derive(Debug)]
pub struct TacProgramBuilder {
    funcs: Vec<TacFunctionInfo>,
    curr_func: usize,
}

//MARK: TacProgramBuilder
impl TacProgramBuilder {
    pub fn new() -> Self {
        let mut program_builder = TacProgramBuilder {
            funcs: Vec::new(),
            curr_func: 0,
        };

        program_builder.add_function(String::from("main"));
        program_builder
    }

    //MARK: get_program
    // Consumes self
    pub fn get_program(self) -> TacProgram {
        let mut program: TacProgram = Vec::new();
        program.push(TacStatement::Command(
            TacCommand::Goto,
            Some(TacValue::Label(String::from("main0")))
        ));

        for func in self.funcs.iter().rev() {
            // Find variable sizes
            let mut var_sizes = 0;
            for (_, size) in func.var_sizes.iter() {
                var_sizes += size.0 * size.1;
            }

            program.push(TacStatement::Label(func.name.clone()));
            program.push(TacStatement::Command(
                TacCommand::BeginFunc,
                Some(TacValue::Int(var_sizes)),
            ));

            for param in func.params.iter().rev() {
                program.push(TacStatement::Assignment(
                    param.0.clone(),
                    TacOperation {
                        op: None,
                        val1: TacValue::GetParams(param.1),
                        val2: None,
                    },
                ));
            }

            for statement in &func.statements {
                program.push(statement.clone());
            }

            program.push(TacStatement::Command(
                TacCommand::EndFunc,
                None,
            ));
        }

        program
    }

    pub fn get_next_label(&mut self, label: String) -> String {
        let mut label_with_count = label.clone();
        label_with_count.push_str(&self.funcs[self.curr_func].label_count.to_string());
        self.funcs[self.curr_func].label_count += 1;
        label_with_count
    }

    pub fn add_function(&mut self, name: String) -> usize {
        let name_with_count = format!("{}{}", name.clone(), self.funcs.len());
        self.funcs.push(TacFunctionInfo {
            name: name_with_count,
            statements: Vec::new(),
            var_sizes: Vec::new(),
            params: Vec::new(),
            label_count: 0,
            temp_count: 0,
        });
        self.curr_func = self.funcs.len() - 1;
        self.curr_func
    }

    pub fn reset_curr_func(&mut self) {
        self.curr_func = 0;
    }

    pub fn add_parameter(&mut self, decl: &SymbolDecl) {
        self.funcs[self.curr_func].params.push((decl.to_var_name(), decl.get_size()));
    }

    pub fn new_temp_var(&mut self, var_size: u32, scope: usize) -> SymbolDecl {
        let temp_var = SymbolDecl {
            name: format!("t{}_", self.funcs[self.curr_func].temp_count),
            var_type: BasicType::Int,
            scope,
        };
        self.funcs[self.curr_func].temp_count += 1;
        self.set_size(temp_var.name.clone(), var_size, 1);
        temp_var
    }

    pub fn add_statement(&mut self, statement_tree: &StatementTree, scope: usize) -> TacOperation {
        self._add_statement(statement_tree.start.unwrap(), &statement_tree, scope)
    }

    //MARK: add_assignment
    pub fn add_assignment(&mut self, var: String, op: TacOperation) {
        let size = self.get_val_size(&op.val1);
        self.set_size(var.clone(), size, 1);

        let assignment = TacStatement::Assignment(
            var,
            op,
        );
        self.funcs[self.curr_func].statements.push(assignment);
    }

    pub fn add_assignment_statement(&mut self, var: String, statement: &StatementTree, scope: usize) {
        let op = self.add_statement(statement, scope);
        self.add_assignment(var, op);
    }

    pub fn add_array_assignment(
        &mut self,
        arr: String,
        index: &StatementTree,
        statement: &StatementTree,
        arr_len: u32,
        scope: usize
    ) {
        let index_op = self.add_statement(index, scope);
        let index_val = if let Some(_) = index_op.op {
            let temp_var = self.new_temp_var(self.get_val_size(&index_op.val1), scope);
            self.add_assignment(temp_var.name.clone(), index_op);
            TacValue::Var(temp_var.name.clone())
        } else {
            index_op.val1
        };

        let statement_op = self.add_statement(statement, scope);
        self.set_size(arr.clone(), self.get_val_size(&statement_op.val1), arr_len);

        let assignment = TacStatement::PointerAssignment(
            arr,
            index_val,
            statement_op,
        );
        self.funcs[self.curr_func].statements.push(assignment);
    }

    pub fn add_push_param(&mut self, var: String) {
        let push_param = TacStatement::Command(
            TacCommand::PushParam,
            Some(TacValue::Var(var)),
        );
        self.funcs[self.curr_func].statements.push(push_param);
    }

    pub fn add_pop_params(&mut self, size: u32) {
        let pop_params = TacStatement::Command(
            TacCommand::PopParams,
            TacValue::Int(size).into(),
        );
        self.funcs[self.curr_func].statements.push(pop_params);
    }

    pub fn add_call_func(&mut self, func_name: String, params: Box<[String]>, return_var: Option<String>) {
        let mut params_size = 0;
        for param in params.iter() {
            self.add_push_param(param.clone());
            params_size += self.get_val_size(&TacValue::Var(param.clone()));
        }

        let func_call = TacStatement::Command(
            TacCommand::LCall,
            Some(TacValue::Label(func_name.clone())),
        );

        if let Some(var) = return_var {
            self.add_assignment(var, TacOperation {
                op: None,
                val1: TacValue::LCallArgs(func_name),
                val2: None,
            });
        } else {
            self.funcs[self.curr_func].statements.push(func_call);
        }

        self.add_pop_params(params_size);
    }

    // Scope required to add temp vars
    pub fn add_builtin_func(&mut self, builtin_func: &BuiltInFunc, scope: usize) -> SymbolDecl {
        let temp_var: SymbolDecl = self.new_temp_var(
            builtin_func.statement.get_type_size(),
            scope
        );
        self.add_assignment_statement(temp_var.name.clone(), &builtin_func.statement, scope);

        match &builtin_func.func_type {
            BuiltInFuncType::Return => {
                let return_statement = TacStatement::Command(
                    TacCommand::Return,
                    Some(TacValue::Var(temp_var.name.clone())),
                );
                self.funcs[self.curr_func].statements.push(return_statement);
            },
            BuiltInFuncType::Print => {
                self.add_call_func(
                    String::from("print"),
                    vec![temp_var.name.clone()].into_boxed_slice(),
                    None,
                );
            },
        };

        temp_var
    }

    pub fn add_goto(&mut self, label: String) {        
        let goto = TacStatement::Command(
            TacCommand::Goto,
            Some(TacValue::Label(label)),
        );
        self.funcs[self.curr_func].statements.push(goto);
    }

    pub fn add_label(&mut self, label: String) {
        let label = TacStatement::Label(label);
        self.funcs[self.curr_func].statements.push(label);
    }

    fn _add_if(&mut self, cond_var: String, has_else: bool) -> (String, String) {
        let cond_label = if has_else {
            self.get_next_label(String::from("else"))
        } else {
            self.get_next_label(String::from("fi"))
        };

        let ifz = TacStatement::Command(
            TacCommand::IfZ,
            TacValue::IfArgs(
                cond_var.clone(),
                cond_label.clone(),
            ).into(),
        );
        self.funcs[self.curr_func].statements.push(ifz);

        if has_else {
            let end_label = self.get_next_label(String::from("fi"));
            (cond_label, end_label)
        } else {
            (String::new(), cond_label)
        }
    }

    pub fn add_if(&mut self, condition: &StatementTree, scope: usize, has_else: bool) -> (String, String) {
        // Create condition var
        let cond_var = self.new_temp_var(condition.get_type_size(), scope);
        self.add_assignment_statement(cond_var.name.clone(), condition, scope);

        self._add_if(cond_var.name.clone(), has_else)
    }

    pub fn add_while_statement(&mut self, condition: &StatementTree, scope: usize) -> (String, String) {
        // Add while label
        let while_label = self.get_next_label(String::from("while"));
        self.add_label(while_label.clone());
        let end_while_label = self.get_next_label(String::from("od"));
        
        // Create condition var
        let cond_var = self.new_temp_var(condition.get_type_size(), scope);
        self.add_assignment_statement(cond_var.name.clone(), condition, scope);
        
        // Add if statement
        let ifz = TacStatement::Command(
            TacCommand::IfZ,
            TacValue::IfArgs(
                cond_var.name.clone(),
                end_while_label.clone(),
            ).into(),
        );
        self.funcs[self.curr_func].statements.push(ifz);

        (while_label, end_while_label)        
    }

    //MARK: _add_statement
    fn _add_statement(&mut self, curr_node: usize, nodes: &StatementTree, scope: usize) -> TacOperation {
        let node = &nodes.nodes[curr_node];
        match &node.symbol {
            StatementSymbol::Decl(decl_id) => {
                TacOperation {
                    op: None,
                    val1: TacValue::Var(to_var_name(decl_id)),
                    val2: None,
                }
            },
            StatementSymbol::Literal(literal) => {
                match literal {
                    Token::Tint(int) => {
                        TacOperation {
                            op: None,
                            val1: TacValue::Int(*int),
                            val2: None,
                        }
                    },
                    Token::Tdouble(double) => {
                        TacOperation {
                            op: None,
                            val1: TacValue::Double(*double),
                            val2: None,
                        }
                    },
                    _ => {
                        panic!("Invalid literal type");
                    },
                }
            },
            StatementSymbol::ArrayAccess(arr_id, index_statement) => {
                // Get index into temp var
                let temp_var: SymbolDecl = self.new_temp_var(4, scope);
                self.add_assignment_statement(temp_var.name.clone(), index_statement, scope);

                TacOperation {
                    op: None,
                    val1: TacValue::PointerAccess(
                        to_var_name(arr_id),
                        TacValue::Var(temp_var.name.clone()).into(),
                    ),
                    val2: None,
                }
            },
            StatementSymbol::FunctionCall(func_id, args) => {
                // Create temp vars for args
                let mut temp_vars: Vec<String> = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    let temp_var: SymbolDecl = self.new_temp_var(arg.get_type_size(), scope);
                    self.add_assignment_statement(temp_var.name.clone(), arg, scope);
                    temp_vars.push(temp_var.name.clone());
                }
                
                let return_var = self.new_temp_var(4, scope);
                self.add_call_func(
                    func_id.0.clone(),
                    temp_vars.into_boxed_slice(),
                    Some(return_var.name.clone()),
                );

                TacOperation {
                    op: None,
                    val1: TacValue::Var(return_var.name.clone()),
                    val2: None,
                }
            },
            StatementSymbol::SingleChildOperator(token) => {
                let child_op = self._add_statement(
                    node.left.unwrap(),
                    nodes,
                    scope
                );
                let child_var = self.new_temp_var(self.get_val_size(&child_op.val1), scope);
                self.add_assignment(child_var.name.clone(), child_op.clone());

                match token {
                    Token::Ominus => {
                        TacOperation {
                            op: Some(token.clone()),
                            val1: TacValue::Int(0),
                            val2: Some(TacValue::Var(child_var.name.clone())),
                        }
                    },
                    Token::Soparen => {
                        TacOperation {
                            op: None,
                            val1: TacValue::Var(child_var.name.clone()),
                            val2: None,
                        }
                    },
                    Token::Knot => {
                        // Check if var greater than 0
                        let temp_var = self.new_temp_var(child_var.get_size(), scope);
                        let op = TacOperation {
                            op: Some(Token::Ogt),
                            val1: TacValue::Var(child_var.name.clone()),
                            val2: Some(TacValue::Int(0)),
                        };
                        self.add_assignment(temp_var.name.clone(), op);

                        // Add if statement
                        let (else_label, end_label) = self._add_if(temp_var.name.clone(), true);
                        
                        // Create result var
                        let result_var = self.new_temp_var(4, scope);

                        // Add true assignment (set to 0)
                        let true_assignment = TacOperation {
                            op: None,
                            val1: TacValue::Int(0),
                            val2: None,
                        };
                        self.add_assignment(result_var.name.clone(), true_assignment);
                        self.add_goto(end_label.clone());

                        // Add false assignment (set to 1)
                        self.add_label(else_label.clone());
                        let false_assignment = TacOperation {
                            op: None,
                            val1: TacValue::Int(1),
                            val2: None,
                        };
                        self.add_assignment(result_var.name.clone(), false_assignment);

                        // Add end label
                        self.add_label(end_label.clone());

                        // Return result var
                        TacOperation {
                            op: None,
                            val1: TacValue::Var(result_var.name.clone()),
                            val2: None,
                        }
                    },
                    _ => {
                        panic!("Invalid single child operator");
                    },
                }
            },
            StatementSymbol::Operator(token) => {
                let left_op = self._add_statement(
                    node.left.unwrap(),
                    nodes,
                    scope
                );
                let right_op = self._add_statement(
                    node.right.unwrap(),
                    nodes,
                    scope
                );

                let left_val = if let Some(_) = left_op.op {
                    let temp_var = self.new_temp_var(self.get_val_size(&left_op.val1), scope);
                    self.add_assignment(temp_var.name.clone(), left_op.clone());
                    TacValue::Var(temp_var.name.clone())
                } else {
                    left_op.val1
                };

                let right_val = if let Some(_) = right_op.op {
                    let temp_var = self.new_temp_var(self.get_val_size(&right_op.val1), scope);
                    self.add_assignment(temp_var.name.clone(), right_op.clone());
                    TacValue::Var(temp_var.name.clone())
                } else {
                    right_op.val1
                };

                match token {
                    Token::Oplus |
                    Token::Ominus |
                    Token::Omultiply |
                    Token::Odivide |
                    Token::Omod |
                    Token::Oequal |
                    Token::Onot |
                    Token::Olt |
                    Token::Ogt |
                    Token::Olte |
                    Token::Ogte => {
                        TacOperation {
                            op: Some(token.clone()),
                            val1: left_val,
                            val2: Some(right_val),
                        }
                    },
                    Token::Kand => {
                        // Check if greater than 0
                        let op = TacOperation {
                            op: Some(Token::Ogt),
                            val1: left_val,
                            val2: Some(TacValue::Int(0)),
                        };
                        let temp_var = self.new_temp_var(self.get_val_size(&op.val1), scope);
                        self.add_assignment(temp_var.name.clone(), op);

                        // Add if statement
                        let (else_label, end_label) = self._add_if(temp_var.name.clone(), true);
                        
                        // Create result var
                        let result_var = self.new_temp_var(4, scope);

                        // Add true assignment (check right statement)
                        let true_assignment = TacOperation {
                            op: Some(Token::Ogt),
                            val1: right_val,
                            val2: Some(TacValue::Int(0)),
                        };
                        self.add_assignment(result_var.name.clone(), true_assignment);
                        self.add_goto(end_label.clone());

                        // Add false assignment (set to 0)
                        self.add_label(else_label.clone());
                        let false_assignment = TacOperation {
                            op: None,
                            val1: TacValue::Int(0),
                            val2: None,
                        };
                        self.add_assignment(result_var.name.clone(), false_assignment);

                        // Add end label
                        self.add_label(end_label.clone());

                        // Return result var
                        TacOperation {
                            op: None,
                            val1: TacValue::Var(result_var.name.clone()),
                            val2: None,
                        }
                    },
                    Token::Kor => {
                        // Check if greater than 0
                        let op = TacOperation {
                            op: Some(Token::Ogt),
                            val1: left_val,
                            val2: Some(TacValue::Int(0)),
                        };
                        let temp_var = self.new_temp_var(self.get_val_size(&op.val1), scope);
                        self.add_assignment(temp_var.name.clone(), op);

                        // Add if statement
                        let (else_label, end_label) = self._add_if(temp_var.name.clone(), true);
                        
                        // Create result var
                        let result_var = self.new_temp_var(4, scope);

                        // Add true assignment (set to 1)
                        let true_assignment = TacOperation {
                            op: None,
                            val1: TacValue::Int(1),
                            val2: None,
                        };
                        self.add_assignment(result_var.name.clone(), true_assignment);
                        self.add_goto(end_label.clone());

                        // Add false assignment (check right statement)
                        self.add_label(else_label.clone());
                        let false_assignment = TacOperation {
                            op: Some(Token::Ogt),
                            val1: right_val,
                            val2: Some(TacValue::Int(0)),
                        };
                        self.add_assignment(result_var.name.clone(), false_assignment);

                        // Add end label
                        self.add_label(end_label.clone());

                        // Return result var
                        TacOperation {
                            op: None,
                            val1: TacValue::Var(result_var.name.clone()),
                            val2: None,
                        }
                    },
                    token => {
                        panic!("Invalid operator {:?}", token);
                    },
                }
            },
        }
    }

    fn set_size(&mut self, var: String, size: u32, instances: u32) {
        if self.funcs[self.curr_func].var_sizes.len() == 0 {
            self.funcs[self.curr_func].var_sizes.push((var, (size, instances)));
            return;
        }

        let mut low = 0;
        let mut high = self.funcs[self.curr_func].var_sizes.len();

        while low < high {
            let mid = (low + high) / 2;
            if self.funcs[self.curr_func].var_sizes[mid].0 == var {
                if self.funcs[self.curr_func].var_sizes[mid].1.0 < size {
                    self.funcs[self.curr_func].var_sizes[mid].1.0 = size;
                }
                return;
            } else if self.funcs[self.curr_func].var_sizes[mid].0 < var {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        self.funcs[self.curr_func].var_sizes.insert(low, (var, (size, instances)));
    }

    pub fn get_val_size(&self, val: &TacValue) -> u32 {
        match val {
            TacValue::Int(_) => 4,
            TacValue::Double(_) => 4,
            TacValue::Var(var) => {
                let mut low = 0;
                let mut high = self.funcs[self.curr_func].var_sizes.len();

                while low < high {
                    let mid = (low + high) / 2;
                    if self.funcs[self.curr_func].var_sizes[mid].0 == *var {
                        let size = self.funcs[self.curr_func].var_sizes[mid].1;
                        return size.0 * size.1;
                    } else if self.funcs[self.curr_func].var_sizes[mid].0 < *var {
                        low = mid + 1;
                    } else {
                        high = mid;
                    }
                }

                // Check parameters
                for param in self.funcs[self.curr_func].params.iter() {
                    if param.0 == *var {
                        return param.1;
                    }
                }

                return 0;
            },
            _ => 0,
        }
    }
}