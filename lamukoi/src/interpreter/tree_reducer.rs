// interpret: tree reduction
// primops are strict (forces the arguments), others are lazy
// run: run upto WHNF
// reduce: reduce once

use crate::structures::*;
use crate::error::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Node {
    head: Atom,
    stack: VecDeque<Node>,
}

impl Node {
    fn substitute(expr: &ScExpr, args: &[Node]) -> Self {
        let mut node = Node { head: Atom::Prim(0), stack: VecDeque::new() };
        node.substitute_into(expr, args);
        node
    }

    fn substitute_into(&mut self, expr: &ScExpr, args: &[Node]) {
        match expr {
            ScExpr::DefId(i) => {
                self.head = Atom::Sc(*i);
            }
            ScExpr::ArgId(i) => {
                let cur = args[*i].clone();
                let Node { head, stack } = cur;
                for node in stack.into_iter().rev() {
                    self.stack.push_front(node);
                }
                self.head = head;
            }
            ScExpr::Prim(i) => {
                self.head = Atom::Prim(*i);
            }
            ScExpr::App(e1, e2) => {
                self.stack.push_front(Node::substitute(e2, args));
                self.substitute_into(e1, args);
            }
        }
    }
}

impl ScPrimProgram {
    pub fn reduce_to_nf(&mut self, root: &mut Node) -> Result<()> {
        self.reduce_to_whnf(root)?;
        for child in &mut root.stack {
            self.reduce_to_nf(child)?;
        }
        Ok(())
    }

    pub fn reduce_to_whnf(&mut self, root: &mut Node) -> Result<()> {
        while self.reduce_head_once(root)? {}
        Ok(())
    }

    pub fn reduce_head_once(&mut self, root: &mut Node) -> Result<bool> {
        match root.head {
            Atom::Sc(i) => {
                let ScPrimDef { params, ref mut body, .. } = self.defs[i];
                if root.stack.len() >= params {
                    match body {
                        ScBody::Body(body) => {
                            // if sc, reduce using its body
                            let mut args = vec![];
                            for _ in 0..params {
                                args.push(root.stack.pop_front().unwrap());
                            }
                            let node = Node::substitute(body, &args);
                            let Node { head, stack } = node;
                            for node in stack.into_iter().rev() {
                                root.stack.push_front(node);
                            }
                            root.head = head;
                            Ok(true)
                        }
                        ScBody::Prim(_) => {
                            // if primop, whnf its arguments first, check all args are prim without args, and
                            // reduce using the given primop
                            let mut prim_arg = vec![];
                            for arg in root.stack.iter_mut().take(params) {
                                self.reduce_to_whnf(arg)?;
                                if let (Atom::Prim(i), true) = (&arg.head, arg.stack.is_empty()) {
                                    prim_arg.push(*i);
                                } else {
                                    let prim_name = self.defs[i].name.to_string();
                                    let arg = self.whnf_to_string(arg);
                                    return Err(Error::UnexpectedPrimApp { prim_name, arg });
                                }
                            }
                            let prim = match self.defs[i].body {
                                ScBody::Prim(ref mut prim) => prim,
                                _ => unreachable!()
                            };
                            let Some(result) = (prim)(&prim_arg) else {
                                return Err(Error::PrimopFailure { def_name: self.defs[i].name.to_string(), arg: format!("{:?}", prim_arg) });
                            };
                            for _ in 0..params {
                                root.stack.pop_front();
                            }
                            root.head = result;
                            Ok(true)
                        }
                    }
                } else {
                    // unable to reduce head
                    Ok(false)
                }
            }
            Atom::Prim(_) => Ok(false),
        }
    }

    fn whnf_to_string(&self, node: &Node) -> String {
        let head = match node.head {
            Atom::Sc(i) => self.defs[i].name.to_string(),
            Atom::Prim(i) => i.to_string(),
        };
        let body = " (..)".repeat(node.stack.len());
        format!("{}{}", head, body)
    }
}
