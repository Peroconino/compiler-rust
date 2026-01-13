#[macro_use]
extern crate enum_display_derive;

use std::fs;
use std::io;

use lexer::Lexer;

use token::Token;

mod lexer;
mod token;

fn main() -> io::Result<()> {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut lexer = Lexer::new(contents);

    println!("Análise Léxica:");

    let mut token;
    loop {
        token = lexer.get_next_token();
        println!("{}", token);

        match token {
            Token::Eof {} => {
                break;
            }
            Token::Error {
                value: _,
                kind: _,
                line: _,
                column: _,
            } => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
