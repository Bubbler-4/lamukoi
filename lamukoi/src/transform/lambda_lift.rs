// lambda lifting with MFE extraction
// each innermost lambda is analyzed and replaced with MFEs
// MFE (maximal free expression): any subexpression that does not contain DeBruijn(0) but contains DeBruijn(i)
// \x. \y. f (f x) y (g y x)
// -> $f = \e1 e2 y. e1 y (g y e2); \x. $f (f (f x)) x

use crate::structures::*;

#[derive(Debug, Clone)]
pub enum MfeExpr {
    DefId(usize),
    ArgId(usize),
    DeBruijn(usize),
    MfeId(usize),
    Int(i64),
    App(Box<Self>, Box<Self>),
}

impl MfeExpr {
    fn weaken(&mut self) {
        match self {
            MfeExpr::DeBruijn(i) => {
                *i -= 1;
            }
            MfeExpr::App(e1, e2) => {
                e1.weaken();
                e2.weaken();
            }
            _ => {}
        }
    }

    fn into_anon(self) -> AnonExpr {
        match self {
            MfeExpr::DefId(i) => AnonExpr::DefId(i),
            MfeExpr::ArgId(i) => AnonExpr::ArgId(i), // parent args
            MfeExpr::DeBruijn(i) => AnonExpr::DeBruijn(i),
            MfeExpr::MfeId(i) => AnonExpr::ArgId(i), // args of new sc
            MfeExpr::Int(i) => AnonExpr::Int(i),
            MfeExpr::App(e1, e2) => {
                AnonExpr::App(Box::new(e1.into_anon()), Box::new(e2.into_anon()))
            }
        }
    }
}

#[derive(Debug)]
pub struct LiftDef {
    pub name: Ident,
    pub params: usize,
    pub body: Option<MfeExpr>,
}

#[derive(Debug)]
pub struct LiftProgram {
    pub defs: Vec<LiftDef>,
}

#[derive(Clone, Copy)]
enum VarState {
    NoVar,
    Free,
    Bound,
}

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
    pub fn lambda_lift(self, next_def_id: usize) -> (AnonExpr, Vec<AnonDef>) {
        match self {
            AnonExpr::DefId(i) => (AnonExpr::DefId(i), vec![]),
            AnonExpr::ArgId(i) => (AnonExpr::ArgId(i), vec![]),
            AnonExpr::DeBruijn(i) => (AnonExpr::DeBruijn(i), vec![]),
            AnonExpr::Int(i) => (AnonExpr::Int(i), vec![]),
            AnonExpr::App(e1, e2) => {
                let (e1, mut defs1) = e1.lambda_lift(next_def_id);
                let (e2, defs2) = e2.lambda_lift(next_def_id + defs1.len());
                defs1.extend(defs2);
                (AnonExpr::App(Box::new(e1), Box::new(e2)), defs1)
            }
            AnonExpr::Lam(e) => {
                let (e, mut defs) = e.lambda_lift(next_def_id);
                let mut mfes = vec![];
                let (e, _) = e.extract_mfe(&mut mfes);
                let cur_def_id = next_def_id + defs.len();
                defs.push(AnonDef {
                    name: Name::Unnamed(cur_def_id),
                    params: mfes.len(),
                    body: Some(AnonExpr::Lam(Box::new(e.into_anon()))),
                });
                let mut new_e = AnonExpr::DefId(cur_def_id);
                for mfe in mfes {
                    new_e = AnonExpr::App(Box::new(new_e), Box::new(mfe));
                }
                (new_e, defs)
            }
        }
    }

    // lambda body (that does not contain lambdas)
    // -> (transformed self, does self contain free or bound vars?)
    // App (e1, e2):
    // NoVar, NoVar -> NoVar; nothing special
    // (Free | NoVar), (Free | NoVar) -> Free; part of mfe as a whole
    // (Bound | NoVar), (Bound | NoVar) -> Bound; not part of mfe
    // Free, Bound -> extract left as mfe, Bound
    // Bound, Free -> extract right as mfe, Bound
    // DefId, Int: NoVar
    // DeBruijn 0: Bound, DeBruijn _, ArgId: Free
    // Lam e: should not appear here
    fn extract_mfe(self, args: &mut Vec<AnonExpr>) -> (MfeExpr, VarState) {
        match self {
            AnonExpr::DefId(i) => (MfeExpr::DefId(i), VarState::NoVar),
            AnonExpr::Int(i) => (MfeExpr::Int(i), VarState::NoVar),
            AnonExpr::ArgId(i) => (MfeExpr::ArgId(i), VarState::Free),
            AnonExpr::DeBruijn(i) => {
                let state = if i == 0 {
                    VarState::Bound
                } else {
                    VarState::Free
                };
                (MfeExpr::DeBruijn(i), state)
            }
            AnonExpr::App(e1, e2) => {
                let (e1, state1) = e1.extract_mfe(args);
                let (e2, state2) = e2.extract_mfe(args);
                match (state1, state2) {
                    (VarState::NoVar, VarState::NoVar) => {
                        (MfeExpr::App(Box::new(e1), Box::new(e2)), VarState::NoVar)
                    }
                    (VarState::NoVar, VarState::Free)
                    | (VarState::Free, VarState::NoVar)
                    | (VarState::Free, VarState::Free) => {
                        (MfeExpr::App(Box::new(e1), Box::new(e2)), VarState::Free)
                    }
                    (VarState::NoVar, VarState::Bound)
                    | (VarState::Bound, VarState::NoVar)
                    | (VarState::Bound, VarState::Bound) => {
                        (MfeExpr::App(Box::new(e1), Box::new(e2)), VarState::Bound)
                    }
                    (VarState::Free, VarState::Bound) => {
                        let new_e1 = MfeExpr::MfeId(args.len());
                        let mut e1 = e1;
                        e1.weaken();
                        args.push(e1.into_anon());
                        (
                            MfeExpr::App(Box::new(new_e1), Box::new(e2)),
                            VarState::Bound,
                        )
                    }
                    (VarState::Bound, VarState::Free) => {
                        let new_e2 = MfeExpr::MfeId(args.len());
                        let mut e2 = e2;
                        e2.weaken();
                        args.push(e2.into_anon());
                        (
                            MfeExpr::App(Box::new(e1), Box::new(new_e2)),
                            VarState::Bound,
                        )
                    }
                }
            }
            AnonExpr::Lam(_) => {
                unreachable!("extract_mfe should only get passed lambda-free expressions")
            }
        }
    }
}

impl AnonDef {
    // return: transformed self, extracted defs
    // transform body (expr) and get extracted defs
    pub fn lambda_lift(self, next_def_id: usize) -> (AnonDef, Vec<AnonDef>) {
        let Self { name, params, body } = self;
        let Some(body) = body else {
            return (AnonDef { name, params, body: None }, vec![]);
        };
        let (body, defs) = body.lambda_lift(next_def_id);
        (
            AnonDef {
                name,
                params,
                body: Some(body),
            },
            defs,
        )
    }
}

impl AnonProgram {
    pub fn lambda_lift(self) -> AnonProgram {
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
        AnonProgram { defs: transformed }
    }
}
