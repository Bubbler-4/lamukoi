// lambda lifting with MFE extraction
// each innermost lambda is analyzed and replaced with MFEs
// MFE (maximal free expression): any subexpression that does not contain DeBruijn(0) but contains DeBruijn(i)
// \x. \y. f (f x) y (g y x)
// -> $f = \e1 e2 y. e1 y (g y e2); \x. $f (f (f x)) x

use crate::transform::anonymize::*;
use crate::error::{Error, Result};
use std::collections::HashMap;

impl AnonExpr {
    // Lam: when the body is done transforming, transform self
    // - extract MFEs as right arms of Apps;
    // - leave DeBruijns at the spot;
    // - wrap with enough Lams;
    // - move it into a fresh def;
    // - change self into DefId e1 e2 ...
    // - note: need to handle f1 = f2 later
    // App: transform both branches
    // Others: keep intact
    pub fn lambda_lift(self, next_def_id: usize) -> (Self, Vec<AnonDef>) {
        todo!()
    }

    // (transformed self, does self contain free vars?, does self contain bound var?)
    // App (e1, e2):
    // (false, false), (false, false) -> (false, false); nothing special
    // (_, false), (_, false) -> (true, false); part of mfe as a whole
    // (false, _), (false, _) -> (false, true); not part of mfe
    // (true, false), (false, true) -> extract left as mfe, (false, true)
    // (false, true), (true, false) -> extract right as mfe, (false, true)
    fn extract_mfe(self, args: &mut Vec<Self>) -> (Self, bool) {

        todo!()
    }
}

impl AnonDef {
    // return: transformed self, extracted defs
    // transform body (expr) and get extracted defs
    pub fn lambda_lift(self, next_def_id: usize) -> (Self, Vec<Self>) {
        let Self { name, params, body } = self;
        let Some(body) = body else {
            return (Self { name, params, body: None }, vec![]);
        };
        let (body, defs) = body.lambda_lift(next_def_id);
        (Self { name, params, body: Some(body) }, defs)
    }
}

impl AnonProgram {
    pub fn lambda_lift(self) -> Self {
        let mut transformed = vec![];
        let mut next_def_id = self.defs.len();
        let mut extracted = vec![];
        for def in self.defs {
            let (transformed_def, extracted_defs) = def.lambda_lift(next_def_id);
            transformed.push(transformed_def);
            next_def_id += extracted_defs.len();
            extracted.extend(extracted_defs);
        }
        transformed.extend(extracted);
        Self { defs: transformed }
    }
}