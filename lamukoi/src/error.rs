use crate::structures::*;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    TopLevelNameCollision {
        name: Ident,
    },
    ScParamNameCollision {
        def_name: Ident,
        param_name: Ident,
    },
    UndefinedIdent {
        def_name: Ident,
        undefined_name: Ident,
    },
    UnexpectedLambda {
        def_name: Name,
    },
    UnexpectedPrimApp {
        prim_name: Ident,
        arg: String,
    },
    UnknownPrimop {
        def_name: Ident,
    },
    PrimopFailure {
        def_name: Ident,
        arg: String,
    },
    UnnamedPrimop {
        def_no: usize,
    }
}

pub type Result<T> = std::result::Result<T, Error>;
