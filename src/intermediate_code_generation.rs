mod tac;

use crate::syntax_semantic_analysis::{symbol_declaration::{to_var_name, BasicType, SymbolDecl}, symbol_table::{BuiltInFuncType, SymbolEntry, SymbolTable}};
use tac::TacProgram;

pub fn perform_intermediate_code_generation(table: SymbolTable) -> TacProgram {
    let mut program_builder = tac::TacProgramBuilder::new();
    
    let mut last_symbol_stack: Vec<usize> = Vec::new();
    let mut scope_stack: Vec<usize> = vec![0];

    while scope_stack.len() > 0 {
        let curr_scope = scope_stack.pop().unwrap();
        let scope = table.get_scope(curr_scope);

        let stack_size = scope_stack.len();
        let mut start_index = 0;
        if last_symbol_stack.len() == stack_size + 1 {
            start_index = last_symbol_stack.pop().unwrap();
        }

        println!("Scope: {}", curr_scope);
        println!("Start index: {}", start_index);

        for i in start_index..scope.get_symbols().len() {
            let symbol = &scope.get_symbols()[i];
            match symbol {
                SymbolEntry::Decl(decl_id) => {
                    if let Some(decl) = table.find_decl_by_id(decl_id) {
                        if let BasicType::Function(func_info) = &decl.var_type {
                            // Function declaration
                            program_builder.add_function(decl.name.clone());

                            scope_stack.push(curr_scope);
                            last_symbol_stack.push(i + 1);
                            scope_stack.push(func_info.body_scope);
                            break;
                        }
                    }
                },
                SymbolEntry::Parameter(decl_id) => {
                    if let Some(decl) = table.find_decl_by_id(decl_id) {
                        program_builder.add_parameter(decl);
                    }
                },
                SymbolEntry::Scope(new_scope) => {
                    scope_stack.push(curr_scope);
                    last_symbol_stack.push(i + 1);
                    scope_stack.push(*new_scope);
                    break;
                },
                SymbolEntry::BuiltInFunction(builtin_func) => {
                    program_builder.add_builtin_func(builtin_func, curr_scope);
                },
                SymbolEntry::Assignment(assignment_info) => {
                    if let Some(index) = &assignment_info.index {
                        if let Some(arr) = table.find_decl_by_id(&assignment_info.var) {
                            if let BasicType::Array(_, size) = &arr.var_type {
                                program_builder.add_array_assignment(
                                    to_var_name(&assignment_info.var),
                                    index,
                                    &assignment_info.assignment,
                                    *size,
                                    curr_scope
                                );
                            }
                        }
                    } else {
                        program_builder.add_assignment_statement(
                            to_var_name(&assignment_info.var),
                            &assignment_info.assignment,
                            curr_scope
                        );
                    }
                },
                _ => {},
            }
        }

        if stack_size == scope_stack.len() {
            // End function
            program_builder.reset_curr_func();
        }
    }

    dbg!(&program_builder);
    let program = program_builder.get_program();
    program
}