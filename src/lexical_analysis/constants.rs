use super::token::Token;

// Double buffer size
pub const BUFFER_SIZE: usize = 1024;

// Characters
pub const ALPHA: char = 0x01 as char;
pub const DIGIT: char = 0x02 as char;
pub const KEYWORDS: [(&str, Token); 16] = [
    ("if", Token::Kif),
    ("then", Token::Kthen),
    ("fi", Token::Kfi),
    ("else", Token::Kelse),
    ("while", Token::Kwhile),
    ("do", Token::Kdo),
    ("od", Token::Kod),
    ("def", Token::Kdef),
    ("fed", Token::Kfed),
    ("return", Token::Kreturn),
    ("and", Token::Kand),
    ("or", Token::Kor),
    ("not", Token::Knot),
    ("int", Token::Kint),
    ("double", Token::Kdouble),
    ("print", Token::Kprint),
];