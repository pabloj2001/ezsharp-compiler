use crate::logger::Loggable;

#[derive(Clone)]
pub enum DeclarationValue {
    Function,
    Int(Option<i32>),
    Double(Option<f64>),
}

pub struct ParsedDeclaration {
    pub name: String,
    pub value: DeclarationValue,
    pub line: usize,
}

pub struct PartialDeclaration {
    pub name: Option<String>,
    pub value: Option<DeclarationValue>,
    pub line: Option<usize>,
}

impl Loggable for ParsedDeclaration {
    fn to_log_message(&self) -> String {
        match &self.value {
            DeclarationValue::Function => format!("Line {}: func {}", self.line, self.name),
            DeclarationValue::Int(value) => {
                if let Some(val) = value {
                    format!("Line {}: int {} = {}", self.line, self.name, val)
                } else {
                    format!("Line {}: int {}", self.line, self.name)
                }
            },
            DeclarationValue::Double(value) => {
                if let Some(val) = value {
                    format!("Line {}: double {} = {}", self.line, self.name, val)
                } else {
                    format!("Line {}: double {}", self.line, self.name)
                }
            },
        }
    }
}

impl PartialDeclaration {
    pub fn new() -> PartialDeclaration {
        PartialDeclaration {
            name: None,
            value: None,
            line: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.name.is_some() && self.value.is_some() && self.line.is_some()
    }

    pub fn to_parsed_declaration(&self) -> ParsedDeclaration {
        ParsedDeclaration {
            name: self.name.clone().unwrap(),
            value: self.value.clone().unwrap(),
            line: self.line.unwrap(),
        }
    }
}