use crate::syntax::*;

enum LiftedExpr {
    Sc(usize),
    Param(usize),
    App(Box<Self>, Box<Self>),
}

struct LiftedSc {
    name: Ident,
    arity: usize,
    body: LiftedExpr,
}

// lambda lifting and encoding
// 