use crate::lexical_analysis::Token;
use super::non_terminals::NonTerminal;

#[derive(Debug, Clone)]
pub enum ProductionType {
    NonTerminal(NonTerminal),
    Terminal(Token),
}

#[derive(Clone)]
pub struct Production {
    pub left: NonTerminal,
    pub right: Box<[ProductionType]>,
}

pub fn get_constant_productions() -> Box<[Production]> {
    vec![
        // <program> ::= <fdecls> <declarations_seq>.
        Production {
            left: NonTerminal::Program,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Fdecls),
                ProductionType::NonTerminal(NonTerminal::DeclarationsSeq),
                ProductionType::Terminal(Token::Speriod),
            ].into_boxed_slice(),
        },
        // <fdecls> ::= <fdec>; <fdecls>
        Production {
            left: NonTerminal::Fdecls,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Fdec),
                ProductionType::Terminal(Token::Ssemicolon),
                ProductionType::NonTerminal(NonTerminal::Fdecls),
            ].into_boxed_slice(),
        },
        // <fdec> ::= def <type> <fname> ( <params> ) <declarations_seq> fed
        Production {
            left: NonTerminal::Fdec,
            right: vec![
                ProductionType::Terminal(Token::Kdef),
                ProductionType::NonTerminal(NonTerminal::Type),
                ProductionType::NonTerminal(NonTerminal::Fname),
                ProductionType::Terminal(Token::Soparen),
                ProductionType::NonTerminal(NonTerminal::Params),
                ProductionType::Terminal(Token::Scparen),
                ProductionType::NonTerminal(NonTerminal::DeclarationsSeq),
                ProductionType::Terminal(Token::Kfed),
            ].into_boxed_slice(),
        },
        // <params> ::= <type_var><params2>
        Production {
            left: NonTerminal::Params,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::TypeVar),
                ProductionType::NonTerminal(NonTerminal::Params2),
            ].into_boxed_slice(),
        },
        // <params2> ::= , <params>
        Production {
            left: NonTerminal::Params2,
            right: vec![
                ProductionType::Terminal(Token::Scomma),
                ProductionType::NonTerminal(NonTerminal::Params),
            ].into_boxed_slice(),
        },
        // <type_var> ::= <type> <var>
        Production {
            left: NonTerminal::TypeVar,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Type),
                ProductionType::NonTerminal(NonTerminal::Var),
            ].into_boxed_slice(),
        },
        // <fname> ::= <id>
        Production {
            left: NonTerminal::Fname,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Id),
            ].into_boxed_slice(),
        },
        // <declarations> ::= <decl>; <declarations>
        Production {
            left: NonTerminal::Declarations,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Decl),
                ProductionType::Terminal(Token::Ssemicolon),
                ProductionType::NonTerminal(NonTerminal::Declarations),
            ].into_boxed_slice(),
        },
        // <declarations_seq> ::= <declarations> <statement_seq>
        Production {
            left: NonTerminal::DeclarationsSeq,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Declarations),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
            ].into_boxed_slice(),
        },
        // <decl> := <type> <varlist>
        Production {
            left: NonTerminal::Decl,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Type),
                ProductionType::NonTerminal(NonTerminal::VarList),
            ].into_boxed_slice(),
        },
        // <type> := int
        Production {
            left: NonTerminal::Type,
            right: vec![
                ProductionType::Terminal(Token::Kint),
            ].into_boxed_slice(),
        },
        // <type> := double
        Production {
            left: NonTerminal::Type,
            right: vec![
                ProductionType::Terminal(Token::Kdouble),
            ].into_boxed_slice(),
        },
        // <varlist> ::= <var><varlist2>
        Production {
            left: NonTerminal::VarList,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Var),
                ProductionType::NonTerminal(NonTerminal::VarList2),
            ].into_boxed_slice(),
        },
        // <varlist2> ::= , <varlist>
        Production {
            left: NonTerminal::VarList2,
            right: vec![
                ProductionType::Terminal(Token::Scomma),
                ProductionType::NonTerminal(NonTerminal::VarList),
            ].into_boxed_slice(),
        },
        // <statement_seq> ::= <statement><statement_seq2>
        Production {
            left: NonTerminal::StatementSeq,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Statement),
                ProductionType::NonTerminal(NonTerminal::StatementSeq2),
            ].into_boxed_slice(),
        },
        // <statement_seq2> ::= ; <statement_seq>
        Production {
            left: NonTerminal::StatementSeq2,
            right: vec![
                ProductionType::Terminal(Token::Ssemicolon),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
            ].into_boxed_slice(),
        },
        // <statement> ::= <var> = <expr>
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Var),
                ProductionType::Terminal(Token::Oassign),
                ProductionType::NonTerminal(NonTerminal::Expr),
            ].into_boxed_slice(),
        },
        // <statement> ::= <if>
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::If),
            ].into_boxed_slice(),
        },
        // <statement> ::= while <bexpr> do <statement_seq> od
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::Terminal(Token::Kwhile),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Terminal(Token::Kdo),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
                ProductionType::Terminal(Token::Kod),
            ].into_boxed_slice(),
        },
        // <statement> ::= <built_in> <expr>
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::BuiltIn),
                ProductionType::NonTerminal(NonTerminal::Expr),
            ].into_boxed_slice(),
        },
        // <if> ::= if <bexpr> then <statement_seq> <else> fi
        Production {
            left: NonTerminal::If,
            right: vec![
                ProductionType::Terminal(Token::Kif),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Terminal(Token::Kthen),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
                ProductionType::NonTerminal(NonTerminal::Else),
                ProductionType::Terminal(Token::Kfi),
            ].into_boxed_slice(),
        },
        // <else> ::= else <statement_seq>
        Production {
            left: NonTerminal::Else,
            right: vec![
                ProductionType::Terminal(Token::Kelse),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
            ].into_boxed_slice(),
        },
        // <built_in> ::= print
        Production {
            left: NonTerminal::BuiltIn,
            right: vec![
                ProductionType::Terminal(Token::Kprint),
            ].into_boxed_slice(),
        },
        // <built_in> ::= return
        Production {
            left: NonTerminal::BuiltIn,
            right: vec![
                ProductionType::Terminal(Token::Kreturn),
            ].into_boxed_slice(),
        },
        // <expr> ::= <term> <expr2>
        Production {
            left: NonTerminal::Expr,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Term),
                ProductionType::NonTerminal(NonTerminal::Expr2),
            ].into_boxed_slice(),
        },
        // <expr2> ::= + <expr>
        Production {
            left: NonTerminal::Expr2,
            right: vec![
                ProductionType::Terminal(Token::Oplus),
                ProductionType::NonTerminal(NonTerminal::Expr),
            ].into_boxed_slice(),
        },
        // <expr2> ::= - <expr>
        Production {
            left: NonTerminal::Expr2,
            right: vec![
                ProductionType::Terminal(Token::Ominus),
                ProductionType::NonTerminal(NonTerminal::Expr),
            ].into_boxed_slice(),
        },
        // <term> ::= <factor> <term2>
        Production {
            left: NonTerminal::Term,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Factor),
                ProductionType::NonTerminal(NonTerminal::Term2),
            ].into_boxed_slice(),
        },
        // <term2> ::= * <term>
        Production {
            left: NonTerminal::Term2,
            right: vec![
                ProductionType::Terminal(Token::Omultiply),
                ProductionType::NonTerminal(NonTerminal::Term),
            ].into_boxed_slice(),
        },
        // <term2> ::= / <term>
        Production {
            left: NonTerminal::Term2,
            right: vec![
                ProductionType::Terminal(Token::Odivide),
                ProductionType::NonTerminal(NonTerminal::Term),
            ].into_boxed_slice(),
        },
        // <term2> ::= % <term>
        Production {
            left: NonTerminal::Term2,
            right: vec![
                ProductionType::Terminal(Token::Omod),
                ProductionType::NonTerminal(NonTerminal::Term),
            ].into_boxed_slice(),
        },
        // <factor> ::= <id><factor2>
        Production {
            left: NonTerminal::Factor,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Id),
                ProductionType::NonTerminal(NonTerminal::Factor2),
            ].into_boxed_slice(),
        },
        // <factor> ::= <number>
        Production {
            left: NonTerminal::Factor,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Number),
            ].into_boxed_slice(),
        },
        // <factor> ::= (<expr>)
        Production {
            left: NonTerminal::Factor,
            right: vec![
                ProductionType::Terminal(Token::Soparen),
                ProductionType::NonTerminal(NonTerminal::Expr),
                ProductionType::Terminal(Token::Scparen),
            ].into_boxed_slice(),
        },
        // <factor2> ::= <var2>
        Production {
            left: NonTerminal::Factor2,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Var2),
            ].into_boxed_slice(),
        },
        // <factor2> ::= (<exprseq>)
        Production {
            left: NonTerminal::Factor2,
            right: vec![
                ProductionType::Terminal(Token::Soparen),
                ProductionType::NonTerminal(NonTerminal::ExprSeq),
                ProductionType::Terminal(Token::Scparen),
            ].into_boxed_slice(),
        },
        // <exprseq> ::= <expr><exprseq2>
        Production {
            left: NonTerminal::ExprSeq,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Expr),
                ProductionType::NonTerminal(NonTerminal::ExprSeq2),
            ].into_boxed_slice(),
        },
        // <exprseq2> ::= , <exprseq>
        Production {
            left: NonTerminal::ExprSeq2,
            right: vec![
                ProductionType::Terminal(Token::Scomma),
                ProductionType::NonTerminal(NonTerminal::ExprSeq),
            ].into_boxed_slice(),
        },
        // <bexpr> ::= <bterm> <bexpr2>
        Production {
            left: NonTerminal::Bexpr,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Bterm),
                ProductionType::NonTerminal(NonTerminal::Bexpr2),
            ].into_boxed_slice(),
        },
        // <bexpr2> ::= or <bexpr>
        Production {
            left: NonTerminal::Bexpr2,
            right: vec![
                ProductionType::Terminal(Token::Kor),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
            ].into_boxed_slice(),
        },
        // <bterm> ::= <bfactor> <bterm2>
        Production {
            left: NonTerminal::Bterm,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Bfactor),
                ProductionType::NonTerminal(NonTerminal::Bterm2),
            ].into_boxed_slice(),
        },
        // <bterm2> ::= and <bterm>
        Production {
            left: NonTerminal::Bterm2,
            right: vec![
                ProductionType::Terminal(Token::Kand),
                ProductionType::NonTerminal(NonTerminal::Bterm),
            ].into_boxed_slice(),
        },
        // <bfactor> ::= (<bfactor2>)
        Production {
            left: NonTerminal::Bfactor,
            right: vec![
                ProductionType::Terminal(Token::Soparen),
                ProductionType::NonTerminal(NonTerminal::Bfactor2),
                ProductionType::Terminal(Token::Scparen),
            ].into_boxed_slice(),
        },
        // <bfactor> ::= not <bfactor>
        Production {
            left: NonTerminal::Bfactor,
            right: vec![
                ProductionType::Terminal(Token::Knot),
                ProductionType::NonTerminal(NonTerminal::Bfactor),
            ].into_boxed_slice(),
        },
        // <bfactor2> ::= <bexpr>
        Production {
            left: NonTerminal::Bfactor2,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Bexpr),
            ].into_boxed_slice(),
        },
        // <bfactor2> ::= <exprb> <comp> <exprb>
        Production {
            left: NonTerminal::Bfactor2,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Exprb),
                ProductionType::NonTerminal(NonTerminal::Comp),
                ProductionType::NonTerminal(NonTerminal::Exprb),
            ].into_boxed_slice(),
        },
        // <exprb> ::= <termb> <exprb2>
        Production {
            left: NonTerminal::Exprb,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Termb),
                ProductionType::NonTerminal(NonTerminal::Exprb2),
            ].into_boxed_slice(),
        },
        // <exprb2> ::= + <exprb>
        Production {
            left: NonTerminal::Exprb2,
            right: vec![
                ProductionType::Terminal(Token::Oplus),
                ProductionType::NonTerminal(NonTerminal::Exprb),
            ].into_boxed_slice(),
        },
        // <exprb2> ::= - <exprb>
        Production {
            left: NonTerminal::Exprb2,
            right: vec![
                ProductionType::Terminal(Token::Ominus),
                ProductionType::NonTerminal(NonTerminal::Exprb),
            ].into_boxed_slice(),
        },
        // <termb> ::= <factorb> <termb2>
        Production {
            left: NonTerminal::Termb,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Factorb),
                ProductionType::NonTerminal(NonTerminal::Termb2),
            ].into_boxed_slice(),
        },
        // <termb2> ::= * <termb>
        Production {
            left: NonTerminal::Termb2,
            right: vec![
                ProductionType::Terminal(Token::Omultiply),
                ProductionType::NonTerminal(NonTerminal::Termb),
            ].into_boxed_slice(),
        },
        // <termb2> ::= / <termb>
        Production {
            left: NonTerminal::Termb2,
            right: vec![
                ProductionType::Terminal(Token::Odivide),
                ProductionType::NonTerminal(NonTerminal::Termb),
            ].into_boxed_slice(),
        },
        // <termb2> ::= % <termb>
        Production {
            left: NonTerminal::Termb2,
            right: vec![
                ProductionType::Terminal(Token::Omod),
                ProductionType::NonTerminal(NonTerminal::Termb),
            ].into_boxed_slice(),
        },
        // <factorb> ::= <id><factor2>
        Production {
            left: NonTerminal::Factorb,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Id),
                ProductionType::NonTerminal(NonTerminal::Factor2),
            ].into_boxed_slice(),
        },
        // <factorb> ::= <number>
        Production {
            left: NonTerminal::Factorb,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Number),
            ].into_boxed_slice(),
        },
        // <comp> ::= LT
        Production {
            left: NonTerminal::Comp,
            right: vec![
                ProductionType::Terminal(Token::Olt),
            ].into_boxed_slice(),
        },
        // <comp> ::= GT
        Production {
            left: NonTerminal::Comp,
            right: vec![
                ProductionType::Terminal(Token::Ogt),
            ].into_boxed_slice(),
        },
        // <comp> ::= EQUAL
        Production {
            left: NonTerminal::Comp,
            right: vec![
                ProductionType::Terminal(Token::Oequal),
            ].into_boxed_slice(),
        },
        // <comp> ::= LTE
        Production {
            left: NonTerminal::Comp,
            right: vec![
                ProductionType::Terminal(Token::Olte),
            ].into_boxed_slice(),
        },
        // <comp> ::= GTE
        Production {
            left: NonTerminal::Comp,
            right: vec![
                ProductionType::Terminal(Token::Ogte),
            ].into_boxed_slice(),
        },
        // <comp> ::= NOT
        Production {
            left: NonTerminal::Comp,
            right: vec![
                ProductionType::Terminal(Token::Onot),
            ].into_boxed_slice(),
        },
        // <var> ::= <id><var2>
        Production {
            left: NonTerminal::Var,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Id),
                ProductionType::NonTerminal(NonTerminal::Var2),
            ].into_boxed_slice(),
        },
        // <var2> ::= [<expr>]
        Production {
            left: NonTerminal::Var2,
            right: vec![
                ProductionType::Terminal(Token::Sobracket),
                ProductionType::NonTerminal(NonTerminal::Expr),
                ProductionType::Terminal(Token::Scbracket),
            ].into_boxed_slice(),
        },
        // <id> ::= Identifier
        Production {
            left: NonTerminal::Id,
            right: vec![
                ProductionType::Terminal(Token::Identifier(String::new())),
            ].into_boxed_slice(),
        },
        // <number> ::= T_INT
        Production {
            left: NonTerminal::Number,
            right: vec![
                ProductionType::Terminal(Token::Tint(0)),
            ].into_boxed_slice(),
        },
        // <number> ::= T_DOUBLE
        Production {
            left: NonTerminal::Number,
            right: vec![
                ProductionType::Terminal(Token::Tdouble(0.0)),
            ].into_boxed_slice(),
        },
    ].into_boxed_slice()
}