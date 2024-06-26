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
    InvalidToken(InvalidToken),
    NoValidTokens,
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
            LexicalError::InvalidToken(token) => format!("Invalid token: {:?}", token),
            LexicalError::EmptyFile => String::from("File is empty"),
            LexicalError::EndOfFile => String::from("End of file"),
            LexicalError::NoValidTokens => String::from("No valid tokens found"),
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Tint(u32),
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

impl Token {
    pub fn equals_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Tint(_), Token::Tint(_)) => true,
            (Token::Tdouble(_), Token::Tdouble(_)) => true,
            (x, y) => x == y,
        }
    }

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

    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier(s) => s.clone(),
            Token::Tint(i) => i.to_string(),
            Token::Tdouble(d) => d.to_string(),
            Token::Kif => String::from("if"),
            Token::Kthen => String::from("then"),
            Token::Kelse => String::from("else"),
            Token::Kfi => String::from("fi"),
            Token::Kwhile => String::from("while"),
            Token::Kdo => String::from("do"),
            Token::Kod => String::from("od"),
            Token::Kdef => String::from("def"),
            Token::Kfed => String::from("fed"),
            Token::Kreturn => String::from("return"),
            Token::Kand => String::from("and"),
            Token::Kor => String::from("or"),
            Token::Knot => String::from("not"),
            Token::Kint => String::from("int"),
            Token::Kdouble => String::from("double"),
            Token::Kprint => String::from("print"),
            Token::Oplus => String::from("+"),
            Token::Ominus => String::from("-"),
            Token::Omultiply => String::from("*"),
            Token::Odivide => String::from("/"),
            Token::Omod => String::from("%"),
            Token::Oassign => String::from("="),
            Token::Oequal => String::from("=="),
            Token::Olt => String::from("<"),
            Token::Olte => String::from("<="),
            Token::Ogt => String::from(">"),
            Token::Ogte => String::from(">="),
            Token::Onot => String::from("<>"),
            Token::Scomma => String::from(","),
            Token::Ssemicolon => String::from(";"),
            Token::Speriod => String::from("."),
            Token::Soparen => String::from("("),
            Token::Scparen => String::from(")"),
            Token::Sobracket => String::from("["),
            Token::Scbracket => String::from("]"),
        }
    
    }
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
            log_message.push_str(token.to_log_message().as_str());
        }
        log_message
    }
}

#[derive(Debug, Clone)]
pub struct ParsedToken {
    pub token: Token,
    pub line: usize,
}

impl Loggable for ParsedToken {
    fn to_log_message(&self) -> String {
        format!("{:?} on line {}\n", self.token, self.line)
    }
}

impl Loggable for Vec<ParsedToken> {
    fn to_log_message(&self) -> String {
        let mut log_message = String::from(self[0].to_log_message().as_str());
        for token in self.iter().skip(1) {
            log_message.push_str(token.to_log_message().as_str());
        }
        log_message
    }
}