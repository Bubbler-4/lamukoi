use crate::structures::*;
use slotmap::SlotMap;

// interpret: stack, heap, sc, prim
// inputs:
// SC defs (lambda lifting applied)
// entrypoint (ident)
// prim (a callback that takes ID and a forcing iterator, and forces and calculates as necessary)
