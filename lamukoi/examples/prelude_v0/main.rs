#![recursion_limit = "256"]
use lamukoi::structures::*;
use lamukoi::error::*;
use lamukoi::*;
use lamukoi::interpreter::tree_reducer::Node;
use std::collections::VecDeque;
use std::io::Write;

mod prelude_v0;
use prelude_v0::*;

fn main() -> Result<()> {
    let mut prog = program![
        pair = |x y f| f x y;
        snd = |p| p (|x y| y);
        update = |p| p (|x y f| f (succ x) (x succ y));
        rangesum = |n| snd (n update (pair (|x| x) (|x y| y)));
        main = (|f x| f (f x)) rangesum;
        echo = cShow READ;
    ];
    prog.defs.extend(prelude().defs);
    // println!("{}", prog);
    // println!();

    let processed = prog
        .into_anon()?
        .lambda_lift()
        .lambda_elim()?
        .compress();
    // println!("{}", processed);
    let table = processed.def_indexes();
    let _main = table["main"];
    let echo = table["echo"];
    let mut input = VecDeque::from(b"hello".to_vec());
    let mut output: Vec<u8> = vec![];
    {
        let mut primops = prelude_defs(
            &mut input,
            &mut output,
            &table
        );
        let mut processed = processed.attach_prim(&mut primops)?;
        let mut node = Node::from_sc(echo);
        processed.reduce_to_nf(&mut node)?;
    }
    let mut stdout = std::io::stdout();
    stdout.write(&output).unwrap();
    println!();

    // return Ok(());

    let mut prog = program![
        pair = |x y f| f x y;
        snd = |p| p (|x y| y);
        update = |p| p (|x y f| f (succ x) (x succ y));
        rangesum = |n| snd (n update (pair (|x| x) (|x y| y)));
        main = (|f x| f (f x)) rangesum;
        echo = cShow READ;
    ];
    prog.defs.extend(prelude().defs);

    let processed = prog
        .into_anon()?
        .lambda_lift()
        .lambda_elim()?
        .compress();
    // println!("{}", processed);
    let table = processed.def_indexes();
    let _main = table["main"];
    let echo = table["echo"];

    let mut input = std::io::repeat(b'0');
    let mut output = std::io::stdout();
    {
        let mut primops = prelude_defs(
            &mut input,
            &mut output,
            &table
        );
        let mut processed = processed.attach_prim(&mut primops)?;
        let mut node = Node::from_sc(echo);
        processed.reduce_to_nf(&mut node)?;
    }
    Ok(())
}
