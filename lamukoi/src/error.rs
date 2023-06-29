use crate::structures::Ident;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    TopLevelNameCollision(Ident),
    ScParamNameCollision(Ident, Ident),
    UndefinedIdent(Ident),
}

pub type Result<T> = std::result::Result<T, Error>;