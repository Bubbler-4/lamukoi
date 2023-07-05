// lambda elimination
// assumption: lambda appears only at the top level of each definition body
// (i.e. lambda lifting has been performed)

use crate::error::*;
use crate::structures::*;

impl AnonExpr {
    fn lambda_elim(self, args: usize) -> Result<(ScExpr, usize)> {
        let mut cur_term = self;
        let mut lambdas = 0usize;
        while let AnonExpr::Lam(e) = cur_term {
            cur_term = *e;
            lambdas += 1;
        }
        let term = cur_term.try_into_scexpr(args, lambdas)?;
        Ok((term, args + lambdas))
    }

    fn try_into_scexpr(self, args: usize, lambdas: usize) -> Result<ScExpr> {
        match self {
            AnonExpr::DefId(i) => Ok(ScExpr::DefId(i)),
            AnonExpr::ArgId(i) => Ok(ScExpr::ArgId(i)),
            AnonExpr::Prim(i) => Ok(ScExpr::Prim(i)),
            AnonExpr::DeBruijn(i) => Ok(ScExpr::ArgId(args + lambdas - 1 - i)),
            AnonExpr::App(e1, e2) => {
                let e1 = e1.try_into_scexpr(args, lambdas)?;
                let e2 = e2.try_into_scexpr(args, lambdas)?;
                Ok(ScExpr::App(Box::new(e1), Box::new(e2)))
            }
            AnonExpr::Lam(_) => Err(Error::UnexpectedLambda {
                def_name: Name::Unnamed(0),
            }),
        }
    }
}

impl AnonDef {
    fn lambda_elim(self) -> Result<ScDef> {
        let Self { name, params, body } = self;
        if let Some(body) = body {
            let res = body.lambda_elim(params);
            let Ok((new_body, new_params)) = res else {
                return Err(Error::UnexpectedLambda { def_name: name });
            };
            Ok(ScDef {
                name,
                params: new_params,
                body: Some(new_body),
            })
        } else {
            Ok(ScDef {
                name,
                params,
                body: None,
            })
        }
    }
}

impl AnonProgram {
    pub fn lambda_elim(self) -> Result<ScProgram> {
        let defs = self
            .defs
            .into_iter()
            .map(|def| def.lambda_elim())
            .collect::<Result<_>>()?;
        Ok(ScProgram { defs })
    }
}
