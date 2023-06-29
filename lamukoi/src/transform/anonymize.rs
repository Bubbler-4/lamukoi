// anonymization pass
// top-level names are assigned top-level indexes as identifiers
// supercombinator arguments are assigned sc indexes
// lambda-local vars become de Bruijn indexes
// lambdas become single-layered

use crate::structures::*;
use crate::error::{Error, Result};
use std::collections::HashMap;

impl Expr {
    fn into_anon_inner(self, name2id: &HashMap<String, AnonExpr>, index: &mut Vec<String>) -> Result<AnonExpr> {
        let expr = self;
        match expr {
            Expr::Id(ident) => {
                if let Some(pos) = index.iter().rev().position(|x| x == &ident) {
                    return Ok(AnonExpr::DeBruijn(pos));
                }
                let Some(e) = name2id.get(&ident) else {
                    return Err(Error::UndefinedIdent(ident));
                };
                Ok(e.clone())
            }
            Expr::Int(int) => {
                Ok(AnonExpr::Int(int))
            }
            Expr::App(e1, e2) => {
                let e1 = e1.into_anon_inner(name2id, index)?;
                let e2 = e2.into_anon_inner(name2id, index)?;
                Ok(AnonExpr::App(Box::new(e1), Box::new(e2)))
            }
            Expr::Lam(idents, e) => {
                let prev_index_len = index.len();
                let idents_len = idents.len();
                index.extend(idents);
                let mut e = e.into_anon_inner(name2id, index)?;
                index.truncate(prev_index_len);
                for _ in 0..idents_len {
                    e = AnonExpr::Lam(Box::new(e));
                }
                Ok(e)
            }
        }
    }
    pub fn into_anon(self, name2id: &HashMap<String, AnonExpr>) -> Result<AnonExpr> {
        let expr = self;
        expr.into_anon_inner(name2id, &mut vec![])
    }
}

impl Def {
    pub fn into_anon(self, name2id: &mut HashMap<String, AnonExpr>) -> Result<AnonDef> {
        let def = self;
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
        let body = body.into_anon(name2id)?;
        for (k, v) in to_restore {
            name2id.insert(k, v);
        }
        Ok(AnonDef { name, params: params_len, body: Some(body) })
    }
}

impl Program {
    pub fn into_anon(self) -> Result<AnonProgram> {
        let program = self;
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
            anon_defs.push(def.into_anon(&mut name2id)?);
        }
        Ok(AnonProgram { defs: anon_defs })
    }
}