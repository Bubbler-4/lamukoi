# lamukoi

Lambda calculus compiler and interpreter, written in Rust

The name "lamukoi" is a shorthand for "lambda compiler/interpreter" in Japanese style.

## Goals

* Implement various backends of lambda calculus such as [graph reduction, G-machine, TIM](https://www.microsoft.com/en-us/research/publication/implementing-functional-languages-a-tutorial/) and maybe [GRIN](https://github.com/grin-compiler/grin), and compare their performance
    * Includes interpreters and compilers for WASM target
* Write an example frontend on the web