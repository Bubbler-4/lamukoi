use std::fmt::Display;

pub type Ident = String;

#[derive(Debug)]
pub enum Expr {
    Id(Ident),
    Prim(i64),
    App(Box<Self>, Box<Self>),
    Lam(Vec<Ident>, Box<Self>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::App(e1, e2) => {
                if matches!(&**e1, Expr::Lam(_, _)) {
                    write!(f, "({}) ", e1)?;
                } else {
                    write!(f, "{} ", e1)?;
                }
                if matches!(&**e2, Expr::App(_, _) | Expr::Lam(_, _)) {
                    write!(f, "({})", e2)?;
                } else {
                    write!(f, "{}", e2)?;
                }
            }
            Expr::Id(ident) => write!(f, "{}", ident)?,
            Expr::Prim(i) => write!(f, "{}", i)?,
            Expr::Lam(idents, e) => {
                write!(f, "λ{}", idents[0])?;
                for ident in &idents[1..] {
                    write!(f, " {}", ident)?;
                }
                write!(f, ". {}", e)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Def {
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: Option<Expr>,
}

impl Display for Def {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Def { name, params, body } = self;
        write!(f, "{}", name)?;
        for param in params {
            write!(f, " {}", param)?;
        }
        if let Some(body) = body {
            write!(f, " = {}", body)?;
        } else {
            write!(f, " = <builtin>")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Program {
    pub defs: Vec<Def>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.defs.is_empty() {
            write!(f, "{}", self.defs[0])?;
            for def in &self.defs[1..] {
                write!(f, "\n{}", def)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum AnonExpr {
    DefId(usize),
    ArgId(usize),
    DeBruijn(usize),
    Prim(i64),
    App(Box<Self>, Box<Self>),
    Lam(Box<Self>),
}

impl AnonExpr {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        root: &AnonProgram,
        depth: usize,
    ) -> std::fmt::Result {
        match self {
            AnonExpr::DefId(i) => {
                let name = &root.defs[*i].name;
                write!(f, "{}", name)?;
            }
            AnonExpr::ArgId(i) => {
                write!(f, "x{}", i)?;
            }
            AnonExpr::DeBruijn(i) => {
                write!(f, "v{}", depth - 1 - i)?;
            }
            AnonExpr::Prim(i) => {
                write!(f, "i{}", i)?;
            }
            AnonExpr::App(e1, e2) => {
                if matches!(&**e1, AnonExpr::Lam(_)) {
                    write!(f, "(")?;
                    e1.fmt(f, root, depth)?;
                    write!(f, ") ")?;
                } else {
                    e1.fmt(f, root, depth)?;
                    write!(f, " ")?;
                }
                if matches!(&**e2, AnonExpr::App(_, _) | AnonExpr::Lam(_)) {
                    write!(f, "(")?;
                    e2.fmt(f, root, depth)?;
                    write!(f, ")")?;
                } else {
                    e2.fmt(f, root, depth)?;
                }
            }
            AnonExpr::Lam(e) => {
                write!(f, "λv{}. ", depth)?;
                e.fmt(f, root, depth + 1)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Name {
    Named(String),
    Unnamed(usize),
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Name::Named(s) => write!(f, "{}", s),
            Name::Unnamed(i) => write!(f, "?{}", i),
        }
    }
}

#[derive(Debug)]
pub struct AnonDef {
    pub name: Name,
    pub params: usize,
    pub body: Option<AnonExpr>,
}

impl AnonDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, root: &AnonProgram) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        for i in 0..self.params {
            write!(f, " x{}", i)?;
        }
        write!(f, " = ")?;
        if let Some(body) = &self.body {
            body.fmt(f, root, 0)?;
        } else {
            write!(f, "<builtin>")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct AnonProgram {
    pub defs: Vec<AnonDef>,
}

impl Display for AnonProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.defs.is_empty() {
            self.defs[0].fmt(f, self)?;
            for def in &self.defs[1..] {
                write!(f, "\n")?;
                def.fmt(f, self)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScExpr {
    DefId(usize),
    ArgId(usize),
    Prim(i64),
    App(Box<Self>, Box<Self>),
}

impl ScExpr {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        root: &ScProgram,
        depth: usize,
    ) -> std::fmt::Result {
        match self {
            ScExpr::DefId(i) => {
                let name = &root.defs[*i].name;
                write!(f, "{}", name)?;
            }
            ScExpr::ArgId(i) => {
                write!(f, "x{}", i)?;
            }
            ScExpr::Prim(i) => {
                write!(f, "i{}", i)?;
            }
            ScExpr::App(e1, e2) => {
                e1.fmt(f, root, depth)?;
                write!(f, " ")?;
                if matches!(&**e2, ScExpr::App(_, _)) {
                    write!(f, "(")?;
                    e2.fmt(f, root, depth)?;
                    write!(f, ")")?;
                } else {
                    e2.fmt(f, root, depth)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ScDef {
    pub name: Name,
    pub params: usize,
    pub body: Option<ScExpr>,
}

impl ScDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, root: &ScProgram) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        for i in 0..self.params {
            write!(f, " x{}", i)?;
        }
        write!(f, " = ")?;
        if let Some(body) = &self.body {
            body.fmt(f, root, 0)?;
        } else {
            write!(f, "<builtin>")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ScProgram {
    pub defs: Vec<ScDef>,
}

impl Display for ScProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.defs.is_empty() {
            self.defs[0].fmt(f, self)?;
            for def in &self.defs[1..] {
                write!(f, "\n")?;
                def.fmt(f, self)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum Atom {
    Sc(usize),
    Prim(i64),
    IoRes(i64),
}

pub type Primop<'a> = Box<dyn FnMut(&[i64]) -> Option<Atom> + 'a>;

pub enum ScBody<'a> {
    Body(ScExpr),
    Prim(Primop<'a>),
}

pub struct ScPrimDef<'a> {
    pub name: Name,
    pub params: usize,
    pub body: ScBody<'a>,
}

pub struct ScPrimProgram<'a> {
    pub defs: Vec<ScPrimDef<'a>>,
}

#[macro_export]
macro_rules! expr {
    (| $($params: ident)+ | $($tail: tt)+ ) => {
        Expr::Lam(vec![$(stringify!($params).to_string()),+], Box::new(expr!($($tail)+)))
    };
    (($($tok: tt)+)) => {
        expr!($($tok)+)
    };
    ($tok1: tt $tok2: tt $($tail: tt)+) => {
        expr!(($tok1 $tok2) $($tail)+)
    };
    ($tok1: tt $tok2: tt) => {
        Expr::App(Box::new(expr!($tok1)), Box::new(expr!($tok2)))
    };
    ($num: literal) => {
        Expr::Prim($num)
    };
    ($id: tt) => {
        Expr::Id(stringify!($id).to_string())
    };
}

#[macro_export]
macro_rules! lambda {
    ($id: ident $($params: ident)* = $($expr: tt)+) => {
        Def {
            name: stringify!($id).to_string(),
            params: vec![$(stringify!($params).to_string()),*],
            body: Some(expr!($($expr)+)),
        }
    };
    (# $id: ident $($params: ident)*) => {
        Def {
            name: stringify!($id).to_string(),
            params: vec![$(stringify!($params).to_string()),*],
            body: None
        }
    };
}

#[macro_export]
macro_rules! program {
    (@ ($($h: tt)*) ($($e: expr,)*) ; $($t: tt)*) => {
        program!(@ () ($($e,)* lambda!($($h)*),) $($t)*)
    };
    (@ ($($h: tt)*) ($($e: expr,)*) $t: tt $($t2: tt)*) => {
        program!(@ ($($h)* $t) ($($e,)*) $($t2)*)
    };
    (@ () ($($e: expr,)*)) => {
        Program { defs: vec![$($e),*] }
    };
    ($($t: tt)*) => {
        program!(@ () () $($t)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_compiles() {
        let _result = program![
            #mul x y;
            square x = mul x x;
            square2 = (|x| mul x x);
            main = square (square2 3);
        ];
    }
}
