use crate::{lexical_analysis::Token, logger::Loggable};

#[derive(Debug)]
pub enum SyntaxErrorType {
    ExpectedToken(Token, Token),
    UnexpectedToken(Token),
    UnexpectedEndOfFile,
}

pub struct SyntaxError {
    error_type: SyntaxErrorType,
    line: usize,
}

impl SyntaxError {
    pub fn new(error_type: SyntaxErrorType, line: usize) -> Self {
        SyntaxError {
            error_type,
            line,
        }
    }
}

impl Loggable for SyntaxError {
    fn to_log_message(&self) -> String {
        match &self.error_type {
            SyntaxErrorType::ExpectedToken(expected, found) => format!("Expected token {:?} but found {:?} on line {}", expected, found, self.line),
            SyntaxErrorType::UnexpectedToken(token) => format!("Unexpected token {:?} on line {}", token, self.line),
            SyntaxErrorType::UnexpectedEndOfFile => format!("Unexpected end of file on line {}", self.line),
        }
    }
}

impl Loggable for Box<[SyntaxError]> {
    fn to_log_message(&self) -> String {
        let mut msg = String::new();
        for error in self.iter() {
            msg.push_str(&error.to_log_message());
            msg.push('\n');
        }
        msg
    }
}