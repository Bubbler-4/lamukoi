use crate::syntax::Ident;

#[non_exhaustive]
pub enum Error {
    TopLevelNameCollision(Ident),
    ScParamNameCollision(Ident, Ident),
    UndefinedIdent(Ident),
}

pub type Result<T> = std::result::Result<T, Error>;