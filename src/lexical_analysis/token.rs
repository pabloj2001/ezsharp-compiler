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
    Sobracket,
    Scbracket,
}

impl Loggable for Token {
    fn to_log_message(&self) -> String {
        format!("{:?}", self)
    }
}

impl Token {
    pub fn to_index(&self) -> usize {
        match self {
            Token::Identifier(_) => 0,
            Token::Tint(_) => 1,
            Token::Tdouble(_) => 2,
            Token::Kif => 3,
            Token::Kthen => 4,
            Token::Kelse => 5,
            Token::Kfi => 6,
            Token::Kwhile => 7,
            Token::Kdo => 8,
            Token::Kod => 9,
            Token::Kdef => 10,
            Token::Kfed => 11,
            Token::Kreturn => 12,
            Token::Kand => 13,
            Token::Kor => 14,
            Token::Knot => 15,
            Token::Kint => 16,
            Token::Kdouble => 17,
            Token::Kprint => 18,
            Token::Oplus => 19,
            Token::Ominus => 20,
            Token::Omultiply => 21,
            Token::Odivide => 22,
            Token::Omod => 23,
            Token::Oassign => 24,
            Token::Oequal => 25,
            Token::Olt => 26,
            Token::Olte => 27,
            Token::Ogt => 28,
            Token::Ogte => 29,
            Token::Onot => 30,
            Token::Scomma => 31,
            Token::Ssemicolon => 32,
            Token::Speriod => 33,
            Token::Soparen => 34,
            Token::Scparen => 35,
            Token::Sobracket => 36,
            Token::Scbracket => 37,
        }
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