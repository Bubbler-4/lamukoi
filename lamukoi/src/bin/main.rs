use lamukoi::*;
use lamukoi::structures::*;

fn main() {
    let prog = program![
        succ n f x = f (n f x);
        pair = |x y f| f x y;
        snd = |p| p (|x y| y);
        update = |p| p (|x y f| f (succ x) (x succ y));
        rangesum = |n| snd (n update (pair (|x| x) (|x y| y)));
        main = (|f x| f (f x)) rangesum;
    ];
    println!("Original:");
    println!("{}", prog);
    let prog2 = prog.into_anon().unwrap();
    println!("After anonymization:");
    println!("{}", prog2);
    let prog3 = prog2.lambda_lift();
    println!("After lambda lifting:");
    println!("{:?}", prog3);
    println!("{}", prog3);
}