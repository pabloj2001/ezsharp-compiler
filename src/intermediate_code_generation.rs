mod tac;
mod tac_program_builder;

use crate::syntax_semantic_analysis::{
    symbol_declaration::{
        to_var_name,
        BasicType
    },
    symbol_table::{
        ConditionalStatementType,
        SymbolEntry,
        SymbolTable
    }
};
use tac::TacProgram;

use self::tac_program_builder::TacProgramBuilder;

pub fn perform_intermediate_code_generation(table: SymbolTable) -> TacProgram {
    let mut program_builder = TacProgramBuilder::new();

    generate_scope_code(0, &table, &mut program_builder);

    program_builder.get_program()
}

fn generate_scope_code(curr_scope: usize, table: &SymbolTable, program_builder: &mut TacProgramBuilder) {
    let scope = table.get_scope(curr_scope);
    let mut skip_else = false;
    for i in 0..scope.get_symbols().len() {
        if skip_else {
            skip_else = false;
            continue;
        }

        let symbol = &scope.get_symbols()[i];
        match symbol {
            SymbolEntry::Decl(decl_id) => {
                if let Some(decl) = table.find_decl_by_id(decl_id) {
                    if let BasicType::Function(func_info) = &decl.var_type {
                        // Function declaration
                        program_builder.add_function(decl.name.clone());
                        generate_scope_code(func_info.body_scope, table, program_builder);
                        // End function
                        program_builder.reset_curr_func();
                    }
                }
            },
            SymbolEntry::Parameter(decl_id) => {
                if let Some(decl) = table.find_decl_by_id(decl_id) {
                    program_builder.add_parameter(decl);
                }
            },
            SymbolEntry::Scope(new_scope) => {
                generate_scope_code(*new_scope, table, program_builder);
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
            SymbolEntry::ConditionalStatement(cond) => {
                match cond.statement_type {
                    ConditionalStatementType::While => {
                        // Generate code for the condition
                        let (while_label, end_while_label) = program_builder.add_while_statement(
                            &cond.condition.clone().unwrap(),
                            curr_scope
                        );

                        // Generate code for the body
                        generate_scope_code(cond.body_scope, table, program_builder);
                    
                        // Generate code for the end of the while loop
                        program_builder.add_goto(while_label);
                        program_builder.add_label(end_while_label);    
                    },
                    ConditionalStatementType::If => {
                        // Check for else statement
                        let mut else_body = None;
                        if scope.get_symbols().len() > i + 1 {
                            if let SymbolEntry::ConditionalStatement(next_cond) = &scope.get_symbols()[i + 1] {
                                if matches!(next_cond.statement_type, ConditionalStatementType::Else) {
                                    else_body = Some(next_cond.body_scope);
                                    skip_else = true;
                                }
                            }
                        }

                        // Generate code for the condition
                        let (else_label, end_if_label) = program_builder.add_if(
                            &cond.condition.clone().unwrap(),
                            curr_scope,
                            else_body.is_some()
                        );

                        // Generate code for the body
                        generate_scope_code(cond.body_scope, table, program_builder);
                        
                        if let Some(body_scope) = else_body {
                            // Add a goto statement to skip the else body
                            program_builder.add_goto(end_if_label.clone());
                            
                            // Generate code for the else start label
                            program_builder.add_label(else_label);
                            
                            // Generate code for the else body
                            generate_scope_code(body_scope, table, program_builder);
                        }

                        // Generate code for the end of the if statement
                        program_builder.add_label(end_if_label);
                    },
                    _ => {}
                }
            },
            _ => {},
        }
    }
}