pub const NUM_NON_TERMINALS: usize = 43;
pub const NUM_TERMINALS: usize = 39;

macro_rules! enum_slice {
    ($(#[$derives:meta])* $visibility:vis enum $name:ident($count:expr) { $($member:ident,)* }) => {
        $(#[$derives])*
        $visibility enum $name {
            $($member,)*
        }
        impl $name {
            $visibility const fn values() -> [$name; $count] {
                [$($name::$member,)*]
            }
        }
    };
}

enum_slice! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum NonTerminal(NUM_NON_TERMINALS) {
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
        Expr,
        Expr2,
        Term,
        Term2,
        Factor,
        Factor2,
        ExprSeq,
        ExprSeq2,
        Bexpr,
        Bexpr2,
        Bterm,
        Bterm2,
        Bfactor,
        Bfactor2,
        Exprb,
        Exprb2,
        Termb,
        Termb2,
        Factorb,
        Comp,
        Var,
        Var2,
        Id,
        Number,
    }
}

impl NonTerminal {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}