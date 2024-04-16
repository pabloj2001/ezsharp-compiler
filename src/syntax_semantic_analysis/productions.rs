use crate::lexical_analysis::Token;
use super::non_terminals::NonTerminal;
use super::semantic_actions::SemanticAction;

#[derive(Debug, Clone)]
pub enum ProductionType {
    NonTerminal(NonTerminal),
    Terminal(Token),
    Action(SemanticAction),
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
        // <fdec> ::= def <type> <fname> [SET_FUNC] ( <params> ) [ADD_FUNC_DECL] <declarations_seq> fed [CHECK_RETURN_TYPE] [POP_FUNC]
        Production {
            left: NonTerminal::Fdec,
            right: vec![
                ProductionType::Terminal(Token::Kdef),
                ProductionType::NonTerminal(NonTerminal::Type),
                ProductionType::NonTerminal(NonTerminal::Fname),
                ProductionType::Action(SemanticAction::SetFunc),
                ProductionType::Terminal(Token::Soparen),
                ProductionType::NonTerminal(NonTerminal::Params),
                ProductionType::Terminal(Token::Scparen),
                ProductionType::Action(SemanticAction::AddFuncDecl),
                ProductionType::NonTerminal(NonTerminal::DeclarationsSeq),
                ProductionType::Terminal(Token::Kfed),
                ProductionType::Action(SemanticAction::PopFunc),
            ].into_boxed_slice(),
        },
        // <params> ::= <type_var> [ADD_PARAM] <params2>
        Production {
            left: NonTerminal::Params,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::TypeVar),
                ProductionType::Action(SemanticAction::AddParam),
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
        // <decl> := <type> <varlist> [CLEAR_VAR_DECL]
        Production {
            left: NonTerminal::Decl,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Type),
                ProductionType::NonTerminal(NonTerminal::VarList),
                ProductionType::Action(SemanticAction::ClearVarDecl),
            ].into_boxed_slice(),
        },
        // <type> := int [SET_TYPE]
        Production {
            left: NonTerminal::Type,
            right: vec![
                ProductionType::Terminal(Token::Kint),
                ProductionType::Action(SemanticAction::SetType),
            ].into_boxed_slice(),
        },
        // <type> := double [SET_TYPE]
        Production {
            left: NonTerminal::Type,
            right: vec![
                ProductionType::Terminal(Token::Kdouble),
                ProductionType::Action(SemanticAction::SetType),
            ].into_boxed_slice(),
        },
        // <varlist> ::= <var> [ADD_VAR_DECL] <varlist2>
        Production {
            left: NonTerminal::VarList,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Var),
                ProductionType::Action(SemanticAction::AddVarDecl),
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
        // <statement> ::= [START_ASSIGNMENT] <var> = [START_TYPE_TREE] <bexpr> [CHECK_VAR_TYPE] [ADD_ASSIGNMENT]
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::Action(SemanticAction::StartAssignment),
                ProductionType::NonTerminal(NonTerminal::Var),
                ProductionType::Terminal(Token::Oassign),
                ProductionType::Action(SemanticAction::StartTypeTree),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Action(SemanticAction::CheckVarType),
                ProductionType::Action(SemanticAction::AddAssignment),
            ].into_boxed_slice(),
        },
        // <statement> ::= <if>
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::If),
            ].into_boxed_slice(),
        },
        // <statement> ::= while [START_WHILE] [START_TYPE_TREE] <bexpr> [ADD_CONDITION] do [NEW_SCOPE] <statement_seq> [ADD_COND_STATEMENT] od
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::Terminal(Token::Kwhile),
                ProductionType::Action(SemanticAction::StartWhile),
                ProductionType::Action(SemanticAction::StartTypeTree),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Action(SemanticAction::AddCondition),
                ProductionType::Terminal(Token::Kdo),
                ProductionType::Action(SemanticAction::NewScope),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
                ProductionType::Action(SemanticAction::AddCondStatement),
                ProductionType::Terminal(Token::Kod),
            ].into_boxed_slice(),
        },
        // <statement> ::= <built_in> [START_TYPE_TREE] <bexpr>  [ADD_TYPE_TREE]
        Production {
            left: NonTerminal::Statement,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::BuiltIn),
                ProductionType::Action(SemanticAction::StartTypeTree),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Action(SemanticAction::AddTypeTree),
            ].into_boxed_slice(),
        },
        // <if> ::= if [START_IF] [START_TYPE_TREE] <bexpr> [ADD_CONDITION] then [NEW_SCOPE] <statement_seq> [ADD_COND_STATEMENT] <else> fi
        Production {
            left: NonTerminal::If,
            right: vec![
                ProductionType::Terminal(Token::Kif),
                ProductionType::Action(SemanticAction::StartIf),
                ProductionType::Action(SemanticAction::StartTypeTree),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Action(SemanticAction::AddCondition),
                ProductionType::Terminal(Token::Kthen),
                ProductionType::Action(SemanticAction::NewScope),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
                ProductionType::Action(SemanticAction::AddCondStatement),
                ProductionType::NonTerminal(NonTerminal::Else),
                ProductionType::Terminal(Token::Kfi),
            ].into_boxed_slice(),
        },
        // <else> ::= else [START_ELSE] [NEW_SCOPE] <statement_seq> [ADD_COND_STATEMENT]
        Production {
            left: NonTerminal::Else,
            right: vec![
                ProductionType::Terminal(Token::Kelse),
                ProductionType::Action(SemanticAction::StartElse),
                ProductionType::Action(SemanticAction::NewScope),
                ProductionType::NonTerminal(NonTerminal::StatementSeq),
                ProductionType::Action(SemanticAction::AddCondStatement),
            ].into_boxed_slice(),
        },
        // <built_in> ::= print [START_PRINT]
        Production {
            left: NonTerminal::BuiltIn,
            right: vec![
                ProductionType::Terminal(Token::Kprint),
                ProductionType::Action(SemanticAction::StartPrint),
            ].into_boxed_slice(),
        },
        // <built_in> ::= return [START_RETURN]
        Production {
            left: NonTerminal::BuiltIn,
            right: vec![
                ProductionType::Terminal(Token::Kreturn),
                ProductionType::Action(SemanticAction::StartReturn),
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
        // <bexpr2> ::= or [SPLIT_TREE] <bexpr> [CHECK_TYPE]
        Production {
            left: NonTerminal::Bexpr2,
            right: vec![
                ProductionType::Terminal(Token::Kor),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Action(SemanticAction::CheckType),
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
        // <bterm2> ::= and [SPLIT_TREE] <bterm> [CHECK_TYPE]
        Production {
            left: NonTerminal::Bterm2,
            right: vec![
                ProductionType::Terminal(Token::Kand),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Bterm),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <bfactor> ::= <expr> <bfactor2>
        Production {
            left: NonTerminal::Bfactor,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Expr),
                ProductionType::NonTerminal(NonTerminal::Bfactor2),
            ].into_boxed_slice(),
        },
        // <bfactor> ::= not [ADD_OPERATOR] <bfactor> [CHECK_TYPE]
        Production {
            left: NonTerminal::Bfactor,
            right: vec![
                ProductionType::Terminal(Token::Knot),
                ProductionType::Action(SemanticAction::AddOperator),
                ProductionType::NonTerminal(NonTerminal::Bfactor),
                ProductionType::Action(SemanticAction::CheckType),
                
            ].into_boxed_slice(),
        },
        // <bfactor2> ::= <comp> [SPLIT_TREE] <expr> [CHECK_TYPE]
        Production {
            left: NonTerminal::Bfactor2,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Comp),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Expr),
                ProductionType::Action(SemanticAction::CheckType),
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
        // <expr2> ::= + [SPLIT_TREE] <expr> [CHECK_TYPE]
        Production {
            left: NonTerminal::Expr2,
            right: vec![
                ProductionType::Terminal(Token::Oplus),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Expr),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <expr2> ::= - [SPLIT_TREE] <expr> [CHECK_TYPE]
        Production {
            left: NonTerminal::Expr2,
            right: vec![
                ProductionType::Terminal(Token::Ominus),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Expr),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <term> ::= <neg_factor> <term2>
        Production {
            left: NonTerminal::Term,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::NegFactor),
                ProductionType::NonTerminal(NonTerminal::Term2),
            ].into_boxed_slice(),
        },
        // <term2> ::= * [SPLIT_TREE] <term> [CHECK_TYPE]
        Production {
            left: NonTerminal::Term2,
            right: vec![
                ProductionType::Terminal(Token::Omultiply),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Term),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <term2> ::= / [SPLIT_TREE] <term> [CHECK_TYPE]
        Production {
            left: NonTerminal::Term2,
            right: vec![
                ProductionType::Terminal(Token::Odivide),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Term),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <term2> ::= % [SPLIT_TREE] <term> [CHECK_TYPE]
        Production {
            left: NonTerminal::Term2,
            right: vec![
                ProductionType::Terminal(Token::Omod),
                ProductionType::Action(SemanticAction::SplitTree),
                ProductionType::NonTerminal(NonTerminal::Term),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <neg_factor> ::= - [ADD_OPERATOR] <factor> [CHECK_TYPE]
        Production {
            left: NonTerminal::NegFactor,
            right: vec![
                ProductionType::Terminal(Token::Ominus),
                ProductionType::Action(SemanticAction::AddOperator),
                ProductionType::NonTerminal(NonTerminal::Factor),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <neg_factor> ::= <factor>
        Production {
            left: NonTerminal::NegFactor,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Factor),
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
        // <factor> ::= <number> [SET_LITERAL]
        Production {
            left: NonTerminal::Factor,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Number),
                ProductionType::Action(SemanticAction::SetLiteral),
            ].into_boxed_slice(),
        },
        // <factor> ::= ( [ADD_OPERATOR] <bexpr> ) [CHECK_TYPE]
        Production {
            left: NonTerminal::Factor,
            right: vec![
                ProductionType::Terminal(Token::Soparen),
                ProductionType::Action(SemanticAction::AddOperator),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Terminal(Token::Scparen),
                ProductionType::Action(SemanticAction::CheckType),
            ].into_boxed_slice(),
        },
        // <factor2> ::= <var2>
        Production {
            left: NonTerminal::Factor2,
            right: vec![
                ProductionType::NonTerminal(NonTerminal::Var2),
            ].into_boxed_slice(),
        },
        // <factor2> ::= [ADD_FUNC_CHECK] (<exprseq>) [POP_FUNC_CHECK]
        Production {
            left: NonTerminal::Factor2,
            right: vec![
                ProductionType::Action(SemanticAction::AddFuncCheck),
                ProductionType::Terminal(Token::Soparen),
                ProductionType::NonTerminal(NonTerminal::ExprSeq),
                ProductionType::Terminal(Token::Scparen),
                ProductionType::Action(SemanticAction::PopFuncCheck),
            ].into_boxed_slice(),
        },
        // <exprseq> ::= [START_TYPE_TREE] <bexpr> [CHECK_PARAM_TYPE] [ADD_TYPE_TREE] <exprseq2>
        Production {
            left: NonTerminal::ExprSeq,
            right: vec![
                ProductionType::Action(SemanticAction::StartTypeTree),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Action(SemanticAction::CheckParamType),
                ProductionType::Action(SemanticAction::AddTypeTree),
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
        // <var2> ::= [ [START_TYPE_TREE] <bexpr> [SET_ARRAY] ]
        Production {
            left: NonTerminal::Var2,
            right: vec![
                ProductionType::Terminal(Token::Sobracket),
                ProductionType::Action(SemanticAction::StartTypeTree),
                ProductionType::NonTerminal(NonTerminal::Bexpr),
                ProductionType::Action(SemanticAction::SetArray),
                ProductionType::Terminal(Token::Scbracket),
            ].into_boxed_slice(),
        },
        // <id> ::= Identifier [SET_ID]
        Production {
            left: NonTerminal::Id,
            right: vec![
                ProductionType::Terminal(Token::Identifier(String::new())),
                ProductionType::Action(SemanticAction::SetId),
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