use core::panic;

use crate::lexical_analysis::Token;
use crate::syntax_semantic_analysis::statement_tree::{StatementNode, StatementSymbol, StatementTree};
use crate::syntax_semantic_analysis::symbol_declaration::{to_var_name, BasicType, SymbolDecl};
use crate::syntax_semantic_analysis::symbol_table::{BuiltInFunc, BuiltInFuncType};

#[derive(Debug, Clone)]
pub enum TacCommand {
    BeginFunc,
    EndFunc,
    PushParam,
    PopParams,
    LCall,
    IfZ,
    Goto,
    Return,
}

#[derive(Debug, Clone)]
pub enum TacValue {
    Label(String),
    Var(String),
    Int(u32),
    Double(f64),
    PointerAccess(String, Box<TacValue>),
    GetParams(u32),
    LCallArgs(String),
    IfArgs(String, String),
}

#[derive(Debug, Clone)]
pub struct TacOperation {
    pub op: Option<Token>,
    pub val1: TacValue,
    pub val2: Option<TacValue>,
}

#[derive(Debug)]
pub enum TacStatement {
    Label(String),
    Assignment(String, TacOperation),
    PointerAssignment(String, TacValue, TacOperation),
    Command(TacCommand, Option<TacValue>),
}

#[derive(Debug)]
pub struct TacFunction {
    pub name: String,
    pub statements: Vec<TacStatement>,
    pub var_sizes: Vec<(String, (u32, u32))>,
    pub label_count: u16,
    pub temp_count: u16,
}

pub type TacProgram = Vec<TacStatement>;

#[derive(Debug)]
pub struct TacProgramBuilder {
    funcs: Vec<TacFunction>,
    curr_func: usize,
}

impl TacProgramBuilder {
    pub fn new() -> Self {
        let mut program_builder = TacProgramBuilder {
            funcs: Vec::new(),
            curr_func: 0,
        };

        program_builder.add_function(String::from("main"));
        program_builder
    }

    pub fn get_program(self) -> TacProgram {
        let mut program: TacProgram = Vec::new();
        program.push(TacStatement::Command(
            TacCommand::Goto,
            Some(TacValue::Label(String::from("main0")))
        ));

        program
    }

    pub fn get_next_label(&mut self, label: String) -> String {
        let mut label_with_count = label.clone();
        label_with_count.push_str(&self.funcs[self.curr_func].label_count.to_string());
        self.funcs[self.curr_func].label_count += 1;
        label_with_count
    }

    pub fn add_function(&mut self, name: String) {
        let mut name_with_count = name.clone();
        name_with_count.push_str(self.funcs.len().to_string().as_str());

        self.funcs.push(TacFunction {
            name: name_with_count,
            statements: Vec::new(),
            var_sizes: Vec::new(),
            label_count: 0,
            temp_count: 0,
        });
        self.curr_func = self.funcs.len() - 1;
    }

    pub fn reset_curr_func(&mut self) {
        self.curr_func = 0;
    }

    pub fn add_parameter(&mut self, decl: &SymbolDecl) {
        let param = TacOperation {
            op: None,
            val1: TacValue::GetParams(
                get_decl_size(decl),
            ),
            val2: None,
        };
        self.add_assignment(decl.to_var_name(), param);
    }

    pub fn new_temp_var(&mut self, scope: usize) -> SymbolDecl {
        let temp_var = SymbolDecl {
            name: format!("_t{}", self.funcs[self.curr_func].temp_count),
            var_type: BasicType::Int,
            scope,
        };
        self.funcs[self.curr_func].temp_count += 1;
        temp_var
    }

    pub fn add_statement(&mut self, statement_tree: &StatementTree, scope: usize) -> TacOperation {
        self._add_statement(statement_tree.start.unwrap(), &statement_tree, scope)
    }

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
            let temp_var = self.new_temp_var(scope);
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

    pub fn add_pop_params(&mut self) {
        let pop_params = TacStatement::Command(
            TacCommand::PopParams,
            None,
        );
        self.funcs[self.curr_func].statements.push(pop_params);
    }

    pub fn add_call_func(&mut self, func_name: String, params: Box<[String]>, return_var: Option<String>) {
        for param in params.iter() {
            self.add_push_param(param.clone());
        }

        let print_statement = TacStatement::Command(
            TacCommand::LCall,
            Some(TacValue::Label(func_name.clone())),
        );
        self.funcs[self.curr_func].statements.push(print_statement);

        self.add_pop_params();
    }

    // Scope required to add temp vars
    pub fn add_builtin_func(&mut self, builtin_func: &BuiltInFunc, scope: usize) -> SymbolDecl {
        let temp_var: SymbolDecl = self.new_temp_var(scope);
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

    pub fn add_if(&mut self, cond_var: String, scope: usize) -> (String, String) {
        let else_label = self.get_next_label(String::from("else"));
        let ifz = TacStatement::Command(
            TacCommand::IfZ,
            TacValue::IfArgs(
                cond_var.clone(),
                else_label.clone(),
            ).into(),
        );
        self.funcs[self.curr_func].statements.push(ifz);

        let end_label = self.get_next_label(String::from("fi"));
        (else_label, end_label)
    }

    pub fn add_goto(&mut self, label: String) {        
        let goto = TacStatement::Command(
            TacCommand::Goto,
            Some(TacValue::Label(self.get_next_label(label))),
        );
        self.funcs[self.curr_func].statements.push(goto);
    }

    pub fn add_label(&mut self, label: String) {
        let label = TacStatement::Label(self.get_next_label(label));
        self.funcs[self.curr_func].statements.push(label);
    }

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
                let temp_var: SymbolDecl = self.new_temp_var(scope);
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
                    let temp_var: SymbolDecl = self.new_temp_var(scope);
                    self.add_assignment_statement(temp_var.name.clone(), arg, scope);
                    temp_vars.push(temp_var.name.clone());
                }
                
                let return_var = self.new_temp_var(scope);
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
                let child_var = self.new_temp_var(scope);
                self.add_assignment(child_var.name.clone(), child_op.clone());

                match token {
                    Token::Ominus => {
                        let zero = match child_op.val1 {
                            TacValue::Int(_) => TacValue::Int(0),
                            TacValue::Double(_) => TacValue::Double(0.0),
                            _ => panic!("Invalid single child operator"),
                        }; 
                        TacOperation {
                            op: Some(token.clone()),
                            val1: zero,
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
                        let temp_var = self.new_temp_var(scope);
                        let op = TacOperation {
                            op: Some(Token::Ogt),
                            val1: TacValue::Var(child_var.name.clone()),
                            val2: Some(TacValue::Int(0)),
                        };
                        self.add_assignment(temp_var.name.clone(), op);

                        // Add if statement
                        let (else_label, end_label) =
                                self.add_if(temp_var.name.clone(), scope);
                        
                        // Create result var
                        let result_var = self.new_temp_var(scope);

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
                    let temp_var = self.new_temp_var(scope);
                    self.add_assignment(temp_var.name.clone(), left_op.clone());
                    TacValue::Var(temp_var.name.clone())
                } else {
                    left_op.val1
                };

                let right_val = if let Some(_) = right_op.op {
                    let temp_var = self.new_temp_var(scope);
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
                        let temp_var = self.new_temp_var(scope);
                        let op = TacOperation {
                            op: Some(Token::Ogt),
                            val1: left_val,
                            val2: Some(TacValue::Int(0)),
                        };
                        self.add_assignment(temp_var.name.clone(), op);

                        // Add if statement
                        let (else_label, end_label) =
                                self.add_if(temp_var.name.clone(), scope);
                        
                        // Create result var
                        let result_var = self.new_temp_var(scope);

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
                        let temp_var = self.new_temp_var(scope);
                        let op = TacOperation {
                            op: Some(Token::Ogt),
                            val1: left_val,
                            val2: Some(TacValue::Int(0)),
                        };
                        self.add_assignment(temp_var.name.clone(), op);

                        // Add if statement
                        let (else_label, end_label) =
                                self.add_if(temp_var.name.clone(), scope);
                        
                        // Create result var
                        let result_var = self.new_temp_var(scope);

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
                    _ => {
                        panic!("Invalid operator");
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
            } else if self.funcs[self.curr_func].var_sizes[mid].0 < var {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        self.funcs[self.curr_func].var_sizes.push((var, (size, instances)));
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

                0
            },
            _ => 0,
        }
    }
}

pub fn get_decl_size(decl: &SymbolDecl) -> u32 {
    match &decl.var_type {
        BasicType::Array(arr_type, size) => {
            match **arr_type {
                BasicType::Int => {
                    4 * (*size)
                },
                BasicType::Double => {
                    4 * (*size)
                },
                _ => 0,
            }
        },
        BasicType::Int => {
            4
        },
        BasicType::Double => {
            4
        },
        _ => 0,
    }
}