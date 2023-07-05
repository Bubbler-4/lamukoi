// supercombinator compression
// 1) detect `sc1 = sc2` type expressions and copy sc2's arg length and body to sc1
// 2) detect identical bodies and only keep necessary ones
// (all explicitly named ones, or the first unnamed one if all duplicates are unnamed)

use crate::structures::*;
use std::collections::HashMap;

impl ScExpr {
    fn renumber(&mut self, table: &[usize]) {
        match self {
            ScExpr::Prim(_) => {}
            ScExpr::ArgId(_) => {}
            ScExpr::App(e1, e2) => {
                e1.renumber(table);
                e2.renumber(table);
            }
            ScExpr::DefId(i) => {
                *i = table[*i];
            }
        }
    }
}

impl ScProgram {
    pub fn compress(self) -> Self {
        let mut defs = self.defs;
        loop {
            let len = defs.len();
            for i in 0..len {
                if defs[i].params == 0 {
                    if let Some(ScExpr::DefId(j)) = &defs[i].body {
                        let j = *j;
                        defs[i].params = defs[j].params;
                        defs[i].body = defs[j].body.clone();
                    }
                }
            }
            let mut hash = HashMap::new();
            for i in 0..len {
                if let Some(body) = &defs[i].body {
                    hash.entry((defs[i].params, body)).or_insert(vec![]).push(i);
                }
            }
            let mut keep = vec![true; len];
            let mut renumber = (0..len).collect::<Vec<_>>();
            for v in hash.into_values() {
                let mut named = vec![];
                let mut unnamed = vec![];
                for &x in &v {
                    if matches!(defs[x].name, Name::Named(_)) {
                        named.push(x);
                    } else {
                        unnamed.push(x);
                    }
                }
                let start_idx = if named.is_empty() { 1 } else { 0 };
                let target = if named.is_empty() {
                    unnamed[0]
                } else {
                    named[0]
                };
                for &x in &unnamed[start_idx..] {
                    keep[x] = false;
                    renumber[x] = target;
                }
            }
            let mut next_id = 0usize;
            for i in 0..len {
                if keep[i] {
                    renumber[i] = next_id;
                    next_id += 1;
                } else {
                    renumber[i] = renumber[renumber[i]];
                }
            }
            let mut next_defs = vec![];
            for (i, mut def) in defs.into_iter().enumerate() {
                if keep[i] {
                    if let Some(body) = &mut def.body {
                        body.renumber(&renumber);
                    }
                    next_defs.push(def);
                }
            }
            let mut next_unnamed_id = 0usize;
            for def in &mut next_defs {
                if let Name::Unnamed(i) = &mut def.name {
                    *i = next_unnamed_id;
                }
                next_unnamed_id += 1;
            }
            defs = next_defs;
            if len == defs.len() {
                break;
            }
        }
        Self { defs }
    }
}
