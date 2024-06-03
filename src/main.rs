mod lex;
#[allow(unused_imports)]
use lex::*;


mod parse;
use parse::parser::Parser;

mod ast;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", Parser::parse(&contents));
    Ok(())
}
