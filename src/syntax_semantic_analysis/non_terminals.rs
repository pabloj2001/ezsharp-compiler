pub const NUM_NON_TERMINALS: usize = 39;
pub const NUM_TERMINALS: usize = 39;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NonTerminal {
    Program,
    Fdecls,
    Fdec,
    Params,
    Params2,
    TypeVar,
    Fname,
    Declarations,
    DeclarationsSeq,
    Decl,
    Type,
    VarList,
    VarList2,
    StatementSeq,
    StatementSeq2,
    Statement,
    If,
    Else,
    BuiltIn,
    Bexpr,
    Bexpr2,
    Bterm,
    Bterm2,
    Bfactor,
    Bfactor2,
    Expr,
    Expr2,
    Term,
    Term2,
    NegFactor,
    Factor,
    Factor2,
    ExprSeq,
    ExprSeq2,
    Comp,
    Var,
    Var2,
    Id,
    Number,
}

impl NonTerminal {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}