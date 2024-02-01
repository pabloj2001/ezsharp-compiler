use crate::logger::Loggable;

#[derive(Debug)]
pub struct InvalidToken {
    pub lexeme: String,
    pub line: usize,
}

#[derive(Debug)]
pub enum LexicalError {
    FileOpenError(String),
    FileReadError(String),
    EmptyFile,
    EndOfFile,
    InvalidTokens(Vec<InvalidToken>),
}

impl Loggable for LexicalError {
    fn to_log_message(&self) -> String {
        match self {
            LexicalError::FileOpenError(e) => format!("File open error: {}", e),
            LexicalError::FileReadError(e) => format!("File read error: {}", e),
            LexicalError::InvalidTokens(tokens) => {
                if tokens.is_empty() {
                    return String::new();
                }

                let mut log_message = String::from("Invalid tokens:");
                for token in tokens {
                    log_message.push_str(format!("\n{:?}", token).as_str());
                }
                log_message
            }
            LexicalError::EmptyFile => String::from("File is empty"),
            LexicalError::EndOfFile => String::from("End of file"),
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Tint(i32),
    Tdouble(f64),
    Kif,
    Kthen,
    Kelse,
    Kfi,
    Kwhile,
    Kdo,
    Kod,
    Kdef,
    Kfed,
    Kreturn,
    Kand,
    Kor,
    Knot,
    Kint,
    Kdouble,
    Kprint,
    Oplus,
    Ominus,
    Omultiply,
    Odivide,
    Omod,
    Oassign,
    Oequal,
    Olt,
    Olte,
    Ogt,
    Ogte,
    Onot,
    Scomma,
    Ssemicolon,
    Speriod,
    Soparen,
    Scparen,
}

impl Loggable for Token {
    fn to_log_message(&self) -> String {
        format!("{:?}", self)
    }
}

impl Loggable for Vec<Token> {
    fn to_log_message(&self) -> String {
        let mut log_message = String::from(self[0].to_log_message().as_str());
        for token in self.iter().skip(1) {
            log_message.push_str(format!("\n{:?}", token).as_str());
        }
        log_message
    }
}