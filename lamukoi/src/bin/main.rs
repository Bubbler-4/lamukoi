use lamukoi::*;
use lamukoi::syntax::*;
use lamukoi::transform::anonymize::AnonProgram;

fn main() {
    let prog = program![
        succ n f x = f (n f x);
        pair = |x y f| f x y;
        snd = |p| p (|x y| y);
        update = |p| p (|x y f| f (succ x) (x succ y));
        rangesum = |n| snd (n update (pair (|x| x) (|x y| y)));
        main = (|f x| f (f x)) rangesum;
    ];
    println!("{:?}", prog);
    let prog2 = AnonProgram::from_program(prog);
    println!("{:?}", prog2);
}