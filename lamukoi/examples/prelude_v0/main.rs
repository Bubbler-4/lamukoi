//! This example produces infinite output to stdout very quickly.
//! Be sure to terminate it quick, or pipe the output through tools such as `pv`
//! to observe the program's streaming speed instead:
//! 
//! ```sh
//! cargo run --example prelude_v0 --release | pv > /dev/null
//! ```

#![recursion_limit = "256"]
use lamukoi::structures::*;
use lamukoi::error::*;
use lamukoi::*;
use lamukoi::interpreter::tree_reducer::Node;
use std::collections::VecDeque;
use std::io::{Read, Write};

mod prelude_v0;
use prelude_v0::*;

fn test_echo<I: Read + 'static, O: Write + 'static>(input: &mut I, output: &mut O) -> Result<()> {
    let mut prog = program![
        echo = cShow READ;
    ];
    prog.defs.extend(prelude().defs);

    let processed = prog
        .into_anon()?
        .lambda_lift()
        .lambda_elim()?
        .compress();
    let table = processed.def_indexes();
    let echo = table["echo"];
    let mut primops = prelude_defs(
        input,
        output,
        &table
    );
    let mut processed = processed.attach_prim(&mut primops)?;
    let mut node = Node::from_sc(echo);
    processed.reduce_to_nf(&mut node)?;
    Ok(())
}

fn main() -> Result<()> {
    // test finite string input "hello"
    let mut input = VecDeque::from(b"hello".to_vec());
    let mut output: Vec<u8> = vec![];
    test_echo(&mut input, &mut output)?;
    let mut stdout = std::io::stdout();
    stdout.write(&output).unwrap();
    println!();

    // test infinite string input of repeated zeros
    let mut input = std::io::repeat(b'0');
    let mut output = std::io::stdout();
    test_echo(&mut input, &mut output)?;
    Ok(())
}
