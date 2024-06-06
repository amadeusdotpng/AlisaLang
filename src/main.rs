mod lex;
#[allow(unused_imports)]
use lex::*;


mod parse;
use parse::parser::Parser;
use parse::stream::TokenStream;

mod ast;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("{:#?}", TokenStream::new(&contents));
    println!("{:#?}", Parser::parse(&contents));
    Ok(())
}
