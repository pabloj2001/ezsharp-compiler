use crate::lexical_analysis::Token;
use super::symbols::NonTerminal;

pub enum FirstSetType {
    Terminal(Token),
    Epsilon,
}

pub struct FirstSet {
    pub non_terminal: NonTerminal,
    pub first_set: Box<[FirstSetType]>,
}

pub fn get_constant_first_sets() -> Box<[FirstSet]> {
    Box::new([
        // <program>: def, int, double, IDENTIFIER, if, while, print, return, ;, .
        FirstSet {
            non_terminal: NonTerminal::Program,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kdef),
                FirstSetType::Terminal(Token::Kint),
                FirstSetType::Terminal(Token::Kdouble),
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Kif),
                FirstSetType::Terminal(Token::Kwhile),
                FirstSetType::Terminal(Token::Kprint),
                FirstSetType::Terminal(Token::Kreturn),
                FirstSetType::Terminal(Token::Ssemicolon),
                FirstSetType::Terminal(Token::Speriod),
            ]),
        },
        // <fdecls>: def, e
        FirstSet {
            non_terminal: NonTerminal::Fdecls,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kdef),
                FirstSetType::Epsilon,
            ]),
        },
        // <fdec>: def
        FirstSet {
            non_terminal: NonTerminal::Fdec,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kdef),
            ]),
        },
        // <params>: int, double
        FirstSet {
            non_terminal: NonTerminal::Params,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kint),
                FirstSetType::Terminal(Token::Kdouble),
            ]),
        },
        // <params2>: COMMA, e
        FirstSet {
            non_terminal: NonTerminal::Params2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Scomma),
                FirstSetType::Epsilon,
            ]),
        },
        // <type_var>: int, double
        FirstSet {
            non_terminal: NonTerminal::TypeVar,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kint),
                FirstSetType::Terminal(Token::Kdouble),
            ]),
        },
        // <fname>: IDENTIFIER
        FirstSet {
            non_terminal: NonTerminal::Fname,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
            ]),
        },
        // <declarations>: int, double, e
        FirstSet {
            non_terminal: NonTerminal::Declarations,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kint),
                FirstSetType::Terminal(Token::Kdouble),
                FirstSetType::Epsilon,
            ]),
        },
        // <declarations_seq>: int, double, IDENTIFIER, if, while, print, return, ;, e
        FirstSet {
            non_terminal: NonTerminal::DeclarationsSeq,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kint),
                FirstSetType::Terminal(Token::Kdouble),
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Kif),
                FirstSetType::Terminal(Token::Kwhile),
                FirstSetType::Terminal(Token::Kprint),
                FirstSetType::Terminal(Token::Kreturn),
                FirstSetType::Terminal(Token::Ssemicolon),
                FirstSetType::Epsilon,
            ]),
        },
        // <decl>: int, double
        FirstSet {
            non_terminal: NonTerminal::Decl,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kint),
                FirstSetType::Terminal(Token::Kdouble),
            ]),
        },
        // <type>: int, double
        FirstSet {
            non_terminal: NonTerminal::Type,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kint),
                FirstSetType::Terminal(Token::Kdouble),
            ]),
        },
        // <varlist>: IDENTIFIER
        FirstSet {
            non_terminal: NonTerminal::VarList,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
            ]),
        },
        // <varlist2>: COMMA, e
        FirstSet {
            non_terminal: NonTerminal::VarList2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Scomma),
                FirstSetType::Epsilon,
            ]),
        },
        // <statement_seq>: IDENTIFIER, if, while, print, return, ;, e
        FirstSet {
            non_terminal: NonTerminal::StatementSeq,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Kif),
                FirstSetType::Terminal(Token::Kwhile),
                FirstSetType::Terminal(Token::Kprint),
                FirstSetType::Terminal(Token::Kreturn),
                FirstSetType::Terminal(Token::Ssemicolon),
                FirstSetType::Epsilon,
            ]),
        },
        // <statement_seq2>: ;, e
        FirstSet {
            non_terminal: NonTerminal::StatementSeq2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Ssemicolon),
                FirstSetType::Epsilon,
            ]),
        },
        // <statement>: IDENTIFIER, if, while, print, return, e
        FirstSet {
            non_terminal: NonTerminal::Statement,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Kif),
                FirstSetType::Terminal(Token::Kwhile),
                FirstSetType::Terminal(Token::Kprint),
                FirstSetType::Terminal(Token::Kreturn),
                FirstSetType::Epsilon,
            ]),
        },
        // <if>: if
        FirstSet {
            non_terminal: NonTerminal::If,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kif),
            ]),
        },
        // <else>: else, e
        FirstSet {
            non_terminal: NonTerminal::Else,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kelse),
                FirstSetType::Epsilon,
            ]),
        },
        // <built_in>: print, return
        FirstSet {
            non_terminal: NonTerminal::BuiltIn,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kprint),
                FirstSetType::Terminal(Token::Kreturn),
            ]),
        },
        // <expr>: IDENTIFIER, T_INT, T_DOUBLE, (
        FirstSet {
            non_terminal: NonTerminal::Expr,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Tint(0)),
                FirstSetType::Terminal(Token::Tdouble(0.0)),
                FirstSetType::Terminal(Token::Soparen),
            ]),
        },
        // <expr2>: +, -, e
        FirstSet {
            non_terminal: NonTerminal::Expr2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Oplus),
                FirstSetType::Terminal(Token::Ominus),
                FirstSetType::Epsilon,
            ]),
        },
        // <term>: IDENTIFIER, T_INT, T_DOUBLE, (
        FirstSet {
            non_terminal: NonTerminal::Term,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Tint(0)),
                FirstSetType::Terminal(Token::Tdouble(0.0)),
                FirstSetType::Terminal(Token::Soparen),
            ]),
        },
        // <term2>: *, /, %, e
        FirstSet {
            non_terminal: NonTerminal::Term2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Omultiply),
                FirstSetType::Terminal(Token::Odivide),
                FirstSetType::Terminal(Token::Omod),
                FirstSetType::Epsilon,
            ]),
        },
        // <factor>: IDENTIFIER, T_INT, T_DOUBLE, (
        FirstSet {
            non_terminal: NonTerminal::Factor,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Tint(0)),
                FirstSetType::Terminal(Token::Tdouble(0.0)),
                FirstSetType::Terminal(Token::Soparen),
            ]),
        },
        // <exprseq>: IDENTIFIER, T_INT, T_DOUBLE, (, e
        FirstSet {
            non_terminal: NonTerminal::ExprSeq,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Tint(0)),
                FirstSetType::Terminal(Token::Tdouble(0.0)),
                FirstSetType::Terminal(Token::Soparen),
                FirstSetType::Epsilon,
            ]),
        },
        // <exprseq2>: COMMA, e
        FirstSet {
            non_terminal: NonTerminal::ExprSeq2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Scomma),
                FirstSetType::Epsilon,
            ]),
        },
        // <bexpr>: (, not
        FirstSet {
            non_terminal: NonTerminal::Bexpr,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Soparen),
                FirstSetType::Terminal(Token::Knot),
            ]),
        },
        // <bexpr2>: or, e
        FirstSet {
            non_terminal: NonTerminal::Bexpr2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kor),
                FirstSetType::Epsilon,
            ]),
        },
        // <bterm>: (, not
        FirstSet {
            non_terminal: NonTerminal::Bterm,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Soparen),
                FirstSetType::Terminal(Token::Knot),
            ]),
        },
        // <bterm2>: and, e
        FirstSet {
            non_terminal: NonTerminal::Bterm2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Kand),
                FirstSetType::Epsilon,
            ]),
        },
        // <bfactor>: (, not
        FirstSet {
            non_terminal: NonTerminal::Bfactor,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Soparen),
                FirstSetType::Terminal(Token::Knot),
            ]),
        },
        // <bfactor2>: (, not, IDENTIFIER, T_INT, T_DOUBLE
        FirstSet {
            non_terminal: NonTerminal::Bfactor2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Soparen),
                FirstSetType::Terminal(Token::Knot),
                FirstSetType::Terminal(Token::Identifier(String::new())),
                FirstSetType::Terminal(Token::Tint(0)),
                FirstSetType::Terminal(Token::Tdouble(0.0)),
            ]),
        },
        // <comp>: LT, GT, EQUAL, LTE, GTE, NOT
        FirstSet {
            non_terminal: NonTerminal::Comp,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Oequal),
                FirstSetType::Terminal(Token::Olt),
                FirstSetType::Terminal(Token::Ogt),
                FirstSetType::Terminal(Token::Olte),
                FirstSetType::Terminal(Token::Ogte),
                FirstSetType::Terminal(Token::Onot),
            ]),
        },
        // <var>: IDENTIFIER
        FirstSet {
            non_terminal: NonTerminal::Var,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
            ]),
        },
        // <var2>: [, e
        FirstSet {
            non_terminal: NonTerminal::Var2,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Sobracket),
                FirstSetType::Epsilon,
            ]),
        },
        // <id>: IDENTIFIER
        FirstSet {
            non_terminal: NonTerminal::Id,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Identifier(String::new())),
            ]),
        },
        // <number>: T_INT, T_DOUBLE
        FirstSet {
            non_terminal: NonTerminal::Number,
            first_set: Box::new([
                FirstSetType::Terminal(Token::Tint(0)),
                FirstSetType::Terminal(Token::Tdouble(0.0)),
            ]),
        },
    ])
}