#[derive(Debug)]
pub struct InvalidToken {
    pub lexeme: String,
    pub line: usize,
}

#[derive(Debug)]
pub enum LexicalError {
    FileOpenError(String),
    FileReadError(String),
    InvalidTokens(Vec<InvalidToken>),
    EndOfFile,
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