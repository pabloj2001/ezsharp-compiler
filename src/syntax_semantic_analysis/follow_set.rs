use crate::lexical_analysis::Token;
use super::non_terminals::NonTerminal;

pub enum FollowSetType {
    Terminal(Token),
    EndOfInput,
}

pub struct FollowSet {
    pub non_terminal: NonTerminal,
    pub follow_set: Box<[FollowSetType]>,
}

pub fn get_constant_follow_sets() -> Box<[FollowSet]> {
    Box::new([
        // <program>: $
        FollowSet {
            non_terminal: NonTerminal::Program,
            follow_set: Box::new([FollowSetType::EndOfInput]),
        },
        // <fdecls>: int, double, IDENTIFIER, if, while, print, return, ;, .
        FollowSet {
            non_terminal: NonTerminal::Fdecls,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Kint),
                FollowSetType::Terminal(Token::Kdouble),
                FollowSetType::Terminal(Token::Identifier(String::new())),
                FollowSetType::Terminal(Token::Kif),
                FollowSetType::Terminal(Token::Kwhile),
                FollowSetType::Terminal(Token::Kprint),
                FollowSetType::Terminal(Token::Kreturn),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Speriod),
            ]),
        },
        // <fdec>: ;
        FollowSet {
            non_terminal: NonTerminal::Fdec,
            follow_set: Box::new([FollowSetType::Terminal(Token::Ssemicolon)]),
        },
        // <params>: )
        FollowSet {
            non_terminal: NonTerminal::Params,
            follow_set: Box::new([FollowSetType::Terminal(Token::Scparen)]),
        },
        // <params2>: )
        FollowSet {
            non_terminal: NonTerminal::Params2,
            follow_set: Box::new([FollowSetType::Terminal(Token::Scparen)]),
        },
        // <type_var>: ), COMMA
        FollowSet {
            non_terminal: NonTerminal::TypeVar,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
            ]),
        },
        // <fname>: (
        FollowSet {
            non_terminal: NonTerminal::Fname,
            follow_set: Box::new([FollowSetType::Terminal(Token::Soparen)]),
        },
        // <declarations>: IDENTIFIER, if, while, print, return, ;, ., fed
        FollowSet {
            non_terminal: NonTerminal::Declarations,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Identifier(String::new())),
                FollowSetType::Terminal(Token::Kif),
                FollowSetType::Terminal(Token::Kwhile),
                FollowSetType::Terminal(Token::Kprint),
                FollowSetType::Terminal(Token::Kreturn),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
            ]),
        },
        // <declarations_seq>: ., fed
        FollowSet {
            non_terminal: NonTerminal::DeclarationsSeq,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
            ]),
        },
        // <decl>: ;
        FollowSet {
            non_terminal: NonTerminal::Decl,
            follow_set: Box::new([FollowSetType::Terminal(Token::Ssemicolon)]),
        },
        // <type>: IDENTIFIER
        FollowSet {
            non_terminal: NonTerminal::Type,
            follow_set: Box::new([FollowSetType::Terminal(Token::Identifier(String::new()))]),
        },
        // <varlist>: ;
        FollowSet {
            non_terminal: NonTerminal::VarList,
            follow_set: Box::new([FollowSetType::Terminal(Token::Ssemicolon)]),
        },
        // <varlist2>: ;
        FollowSet {
            non_terminal: NonTerminal::VarList2,
            follow_set: Box::new([FollowSetType::Terminal(Token::Ssemicolon)]),
        },
        // <statement_seq>: ., fed, od, else, fi
        FollowSet {
            non_terminal: NonTerminal::StatementSeq,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
            ]),
        },
        // <statement_seq2>: ., fed, od, else, fi
        FollowSet {
            non_terminal: NonTerminal::StatementSeq2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
            ]),
        },
        // <statement>: ., fed, od, else, fi, ;
        FollowSet {
            non_terminal: NonTerminal::Statement,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
            ]),
        },
        // <if>: ., fed, od, else, fi, ;
        FollowSet {
            non_terminal: NonTerminal::If,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
            ]),
        },
        // <else>: fi
        FollowSet {
            non_terminal: NonTerminal::Else,
            follow_set: Box::new([FollowSetType::Terminal(Token::Kfi)]),
        },
        // <built_in>: IDENTIFIER, T_INT, T_DOUBLE, (, not
        FollowSet {
            non_terminal: NonTerminal::BuiltIn,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Identifier(String::new())),
                FollowSetType::Terminal(Token::Tint(0)),
                FollowSetType::Terminal(Token::Tdouble(0.0)),
                FollowSetType::Terminal(Token::Soparen),
                FollowSetType::Terminal(Token::Knot),
            ]),
        },
        // <bexpr>: ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Bexpr,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <bexpr2>: ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Bexpr2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <bterm>: or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Bterm,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <bterm2>: or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Bterm2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <bfactor>: and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Bfactor,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <bfactor2>: and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Bfactor2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <expr>: LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Expr,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <expr2>: LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Expr2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <term>: +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Term,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <term2>: +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Term2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <neg_factor>: *, /, %, +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::NegFactor,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Omultiply),
                FollowSetType::Terminal(Token::Odivide),
                FollowSetType::Terminal(Token::Omod),
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <factor>: *, /, %, +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Factor,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Omultiply),
                FollowSetType::Terminal(Token::Odivide),
                FollowSetType::Terminal(Token::Omod),
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <factor2>: *, /, %, +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Factor2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Omultiply),
                FollowSetType::Terminal(Token::Odivide),
                FollowSetType::Terminal(Token::Omod),
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <exprseq>: )
        FollowSet {
            non_terminal: NonTerminal::ExprSeq,
            follow_set: Box::new([FollowSetType::Terminal(Token::Scparen)]),
        },
        // <exprseq2>: )
        FollowSet {
            non_terminal: NonTerminal::ExprSeq2,
            follow_set: Box::new([FollowSetType::Terminal(Token::Scparen)]),
        },
        // <comp>: IDENTIFIER, T_INT, T_DOUBLE, (
        FollowSet {
            non_terminal: NonTerminal::Comp,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Identifier(String::new())),
                FollowSetType::Terminal(Token::Tint(0)),
                FollowSetType::Terminal(Token::Tdouble(0.0)),
                FollowSetType::Terminal(Token::Soparen),
            ]),
        },
        // <var>: ), COMMA, ;, =
        FollowSet {
            non_terminal: NonTerminal::Var,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Oassign),
            ]),
        },
        // <var2>: *, /, %, +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ], =
        FollowSet {
            non_terminal: NonTerminal::Var2,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Oassign),
                FollowSetType::Terminal(Token::Omultiply),
                FollowSetType::Terminal(Token::Odivide),
                FollowSetType::Terminal(Token::Omod),
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <id>: (, [, *, /, %, +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ], =
        FollowSet {
            non_terminal: NonTerminal::Id,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Oassign),
                FollowSetType::Terminal(Token::Soparen),
                FollowSetType::Terminal(Token::Sobracket),
                FollowSetType::Terminal(Token::Omultiply),
                FollowSetType::Terminal(Token::Odivide),
                FollowSetType::Terminal(Token::Omod),
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
        // <number>: *, /, %, +, -, LT, GT, EQUAL, LTE, GTE, NOT, and, or, ., fed, od, else, fi, ;, do, then, ), COMMA, ]
        FollowSet {
            non_terminal: NonTerminal::Number,
            follow_set: Box::new([
                FollowSetType::Terminal(Token::Omultiply),
                FollowSetType::Terminal(Token::Odivide),
                FollowSetType::Terminal(Token::Omod),
                FollowSetType::Terminal(Token::Oplus),
                FollowSetType::Terminal(Token::Ominus),
                FollowSetType::Terminal(Token::Oequal),
                FollowSetType::Terminal(Token::Olt),
                FollowSetType::Terminal(Token::Ogt),
                FollowSetType::Terminal(Token::Olte),
                FollowSetType::Terminal(Token::Ogte),
                FollowSetType::Terminal(Token::Onot),
                FollowSetType::Terminal(Token::Kand),
                FollowSetType::Terminal(Token::Kor),
                FollowSetType::Terminal(Token::Speriod),
                FollowSetType::Terminal(Token::Kfed),
                FollowSetType::Terminal(Token::Kod),
                FollowSetType::Terminal(Token::Kelse),
                FollowSetType::Terminal(Token::Kfi),
                FollowSetType::Terminal(Token::Ssemicolon),
                FollowSetType::Terminal(Token::Kdo),
                FollowSetType::Terminal(Token::Kthen),
                FollowSetType::Terminal(Token::Scparen),
                FollowSetType::Terminal(Token::Scomma),
                FollowSetType::Terminal(Token::Scbracket),
            ]),
        },
    ])
}