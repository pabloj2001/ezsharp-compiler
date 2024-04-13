use std::fmt::{self, Display, Formatter};

use crate::logger::Loggable;

#[derive(Clone, Debug, PartialEq)]
pub struct FuncInfo {
    pub return_type: Box<BasicType>,
    pub param_types: Vec<BasicType>,
    pub body_scope: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BasicType {
    Int,
    Double,
    Function(FuncInfo),
    Array(Box<BasicType>, u32),
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
            BasicType::Array(inner_type, size) => {
                format!("[{}; {}]", inner_type.to_log_message(), *size)
            }
        }
    }
}

impl Display for BasicType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_log_message())
    }
}

pub type DeclId = (String, usize);

pub fn to_var_name(decl_id: &DeclId) -> String {
    format!("{}{}", decl_id.0, decl_id.1)
}

#[derive(Clone, Debug)]
pub struct SymbolDecl {
    pub name: String,
    pub var_type: BasicType,
    pub scope: usize,
}

impl SymbolDecl {
    pub fn new(name: String, var_type: BasicType, scope: usize) -> SymbolDecl {
        SymbolDecl {
            name,
            var_type: var_type,
            scope,
        }
    }

    pub fn new_func(
        name: String,
        return_type: BasicType,
        body_scope: usize,
        scope: usize
    ) -> SymbolDecl {
        let func_info = FuncInfo {
            return_type: Box::new(return_type),
            param_types: Vec::new(),
            body_scope,
        };
        SymbolDecl {
            name,
            var_type: BasicType::Function(func_info),
            scope,
        }
    }

    pub fn get_id(&self) -> DeclId {
        (self.name.clone(), self.scope)
    }

    pub fn to_var_name(&self) -> String {
        format!("{}{}", self.name, self.scope)
    }
}

impl Loggable for SymbolDecl {
    fn to_log_message(&self) -> String {
        format!("{}: {}\n", self.name, self.var_type.to_log_message())
    }
}