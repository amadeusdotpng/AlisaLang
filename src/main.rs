mod lex;
#[allow(unused_imports)]
use lex::*;


mod parse;
use parse::Parser;
#[allow(unused_imports)]
use parse::stream::TokenStream;

mod ast;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    use std::env;
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUSTFLAGS", "-Awarnings");
    let mut file = File::open("foo.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    /*
    let mut lex = lexer::Lexer::new(&contents);
    println!("{:#?}", lex);
    */

    // println!("{:#?}", TokenStream::new(&contents));
    // println!("{}", contents);
    let t = Instant::now();
    let tree = Parser::parse(&contents);
    let time = Instant::now() - t;
    println!("{:?}", time);
    println!("{:#?}", tree);
    Ok(())
}
