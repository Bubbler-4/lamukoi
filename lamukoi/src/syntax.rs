pub type Ident = String;

#[derive(Debug)]
pub enum Expr {
    Id(Ident),
    Int(i64),
    App(Box<Self>, Box<Self>),
    Lam(Vec<Ident>, Box<Self>),
}

#[derive(Debug)]
pub struct Def {
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: Option<Expr>,
}

#[derive(Debug)]
pub struct Program {
    pub defs: Vec<Def>,
}

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
