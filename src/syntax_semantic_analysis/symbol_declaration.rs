use std::fmt::{self, Display, Formatter};

use crate::logger::Loggable;

#[derive(Clone, Debug, PartialEq)]
pub struct FuncInfo {
    pub return_type: Box<BasicType>,
    pub param_types: Vec<BasicType>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BasicType {
    Int,
    Double,
    Function(FuncInfo),
}

impl Loggable for BasicType {
    fn to_log_message(&self) -> String {
        match self {
            BasicType::Int => String::from("int"),
            BasicType::Double => String::from("double"),
            BasicType::Function(func_info) => {
                let mut message = String::from("func(");
                if func_info.param_types.len() > 0 {
                    for param in &func_info.param_types {
                        message.push_str(&param.to_log_message());
                        message.push_str(", ");
                    }
                    message.pop();
                    message.pop();
                }
                message.push_str(") -> ");
                message.push_str(&func_info.return_type.to_log_message());
                message
            }
        }
    }
}

impl Display for BasicType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_log_message())
    }
}

#[derive(Clone, Debug)]
pub struct SymbolDecl {
    pub name: String,
    pub var_type: (BasicType, bool),
    pub scope: usize,
}

impl SymbolDecl {
    pub fn new(name: String, var_type: BasicType, is_array: bool, scope: usize) -> SymbolDecl {
        SymbolDecl {
            name,
            var_type: (var_type, is_array),
            scope,
        }
    }

    pub fn new_func(
        name: String,
        return_type: BasicType,
        return_type_is_array: bool,
        scope: usize
    ) -> SymbolDecl {
        let func_info = FuncInfo {
            return_type: Box::new(return_type),
            param_types: Vec::new(),
        };
        SymbolDecl {
            name,
            var_type: (BasicType::Function(func_info), return_type_is_array),
            scope,
        }
    }
}

impl Loggable for SymbolDecl {
    fn to_log_message(&self) -> String {
        format!(
            "{}: {}{}\n",
            self.name,
            self.var_type.0.to_log_message(),
            if self.var_type.1 { "[]" } else { "" }
        )
    }
}