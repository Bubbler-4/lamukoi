use std::collections::HashMap;

use crate::structures::*;
use crate::error::*;

impl ScDef {
    fn attach_prim(self, primops: &mut HashMap<&'static str, Primop>) -> Result<ScPrimDef> {
        let Self { name, params, body } = self;
        let body = if let Some(body) = body {
            ScBody::Body(body)
        } else {
            match name {
                Name::Named(ref name) => {
                    if let Some(primop) = primops.remove(&**name) {
                        ScBody::Prim(primop)
                    } else {
                        return Err(Error::UnknownPrimop { def_name: name.to_string() });
                    }
                }
                Name::Unnamed(id) => {
                    return Err(Error::UnnamedPrimop { def_no: id });
                }
            }
        };
        Ok(ScPrimDef { name, params, body })
    }
}

impl ScProgram {
    pub fn attach_prim(self, primops: &mut HashMap<&'static str, Primop>) -> Result<ScPrimProgram> {
        Ok(ScPrimProgram {
            defs: self.defs.into_iter().map(
                |def| def.attach_prim(primops)
            ).collect::<Result<_>>()?,
        })
    }
}