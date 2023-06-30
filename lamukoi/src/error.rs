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
}

pub type Result<T> = std::result::Result<T, Error>;
