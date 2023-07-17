use lamukoi::structures::*;
use lamukoi::error::*;
use lamukoi::*;
use std::collections::HashMap;

/*
Some built-in ideas
#EQ x y
#ADD x y
#SUB x y
True t f = t
False t f = f
// Scott list
SNil nil cons = nil
SCons x xs nil cons = cons x xs
// Church list (right fold)
CNil = \c n. n
CCons = \h t c n. c h (t c n)
Z = \f x. x
S = \n f x. f (n f x)
ToChurch i = EQ i 0 Z (S (ToChurch (SUB i 1)))
FromChurch x = x (ADD 1) 0
#CREAD // reads more from input as necessary, turning into CBit1/CBit0/CNil
CBit1 = CCons True CREAD
CBit0 = CCons False CREAD
#SREAD
SBit1 = SCons True SREAD
SBit0 = SCons False SREAD
CShow stream = stream (\x. SHOW (x 1 0)) Id
SShow stream = stream Id (\x xs. SHOW (x 1 0) (SShow xs))
#SHOW x // puts current bit to the stream, returning Id
Id x = x
*/

fn main() -> Result<()> {
    let prog = program![
        succ n f x = f (n f x);
        pair = |x y f| f x y;
        snd = |p| p (|x y| y);
        update = |p| p (|x y f| f (succ x) (x succ y));
        rangesum = |n| snd (n update (pair (|x| x) (|x y| y)));
        main = (|f x| f (f x)) rangesum;
    ];
    println!("{}", prog);
    println!();

    let processed = prog
        .into_anon()?
        .lambda_lift()
        .lambda_elim()?
        .compress();
    println!("{}", processed);
    let _processed = processed.attach_prim(&mut HashMap::new())?;
    todo!("Run program and print result");
}
