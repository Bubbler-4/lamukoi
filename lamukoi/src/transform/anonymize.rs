// anonymization pass
// top-level names are assigned top-level indexes as identifiers
// supercombinator arguments are assigned sc indexes
// lambda-local vars become de Bruijn indexes
// lambdas become single-layered

use crate::syntax::*;
use crate::error::{Error, Result};
use std::collections::HashMap;

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
    pub fn from_expr(expr: Expr, name2id: &mut HashMap<String, AnonExpr>) -> Result<Self> {
        match expr {
            Expr::Id(ident) => {
                let Some(e) = name2id.get(&ident) else {
                    return Err(Error::UndefinedIdent(ident));
                };
                Ok(e.clone())
            }
            Expr::Int(int) => {
                Ok(AnonExpr::Int(int))
            }
            Expr::App(e1, e2) => {
                let e1 = AnonExpr::from_expr(*e1, name2id)?;
                let e2 = AnonExpr::from_expr(*e2, name2id)?;
                Ok(AnonExpr::App(Box::new(e1), Box::new(e2)))
            }
            Expr::Lam(idents, e) => {
                todo!()
            }
        }
    }
}

#[derive(Debug)]
pub struct AnonDef {
    pub name: Ident,
    pub params: usize,
    pub body: Option<AnonExpr>,
}

impl AnonDef {
    pub fn from_def(def: Def, name2id: &mut HashMap<String, AnonExpr>) -> Result<Self> {
        let Def { name, params, body } = def;
        let Some(body) = body else {
            // Primitive
            return Ok(AnonDef { name, params: params.len(), body: None });
        };
        let params_len = params.len();
        let mut to_restore = vec![];
        for (id, param) in params.into_iter().enumerate() {
            let prev_expr = name2id.insert(param.clone(), AnonExpr::ArgId(id));
            if let Some(prev_expr) = prev_expr {
                if let AnonExpr::ArgId(_) = prev_expr {
                    return Err(Error::ScParamNameCollision(name, param));
                }
                to_restore.push((param, prev_expr));
            }
        }
        let body = AnonExpr::from_expr(body, name2id)?;
        for (k, v) in to_restore {
            name2id.insert(k, v);
        }
        Ok(Self { name, params: params_len, body: Some(body) })
    }
}

#[derive(Debug)]
pub struct AnonProgram {
    pub defs: Vec<AnonDef>,
}

impl AnonProgram {
    pub fn from_program(program: Program) -> Result<Self> {
        let mut name2id = HashMap::new();
        for (id, def) in program.defs.iter().enumerate() {
            let current_name = def.name.clone();
            if name2id.contains_key(&current_name) {
                return Err(Error::TopLevelNameCollision(current_name));
            }
            name2id.insert(current_name, AnonExpr::DefId(id));
        }
        let mut anon_defs = vec![];
        for def in program.defs {
            anon_defs.push(AnonDef::from_def(def, &mut name2id)?);
        }
        Ok(Self { defs: anon_defs })
    }
}