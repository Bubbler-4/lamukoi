use std::fmt::Display;

pub type Ident = String;

#[derive(Debug)]
pub enum Expr {
    Id(Ident),
    Int(i64),
    App(Box<Self>, Box<Self>),
    Lam(Vec<Ident>, Box<Self>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::App(e1, e2) => {
                if matches!(&**e1, Expr::Lam(_,_)) {
                    write!(f, "({}) ", e1)?;
                } else {
                    write!(f, "{} ", e1)?;
                }
                if matches!(&**e2, Expr::App(_,_) | Expr::Lam(_,_)) {
                    write!(f, "({})", e2)?;
                } else {
                    write!(f, "{}", e2)?;
                }
            }
            Expr::Id(ident) => write!(f, "{}", ident)?,
            Expr::Int(i) => write!(f, "{}", i)?,
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
    Int(i64),
    App(Box<Self>, Box<Self>),
    Lam(Box<Self>),
}

impl AnonExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, root: &AnonProgram, depth: usize) -> std::fmt::Result {
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
            AnonExpr::Int(i) => {
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
                if matches!(&**e2, AnonExpr::App(_,_) | AnonExpr::Lam(_)) {
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

#[derive(Debug)]
pub struct AnonDef {
    pub name: Ident,
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
        Expr::Int($num)
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
