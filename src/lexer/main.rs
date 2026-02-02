use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use compiler::{Lexer, Token};

mod token;

fn main() -> io::Result<()> {
    let symbol_table = Rc::new(RefCell::new(HashMap::new()));

    let mut lexer = Lexer::new("data.txt", Rc::clone(&symbol_table));

    println!("Análise Léxica:");

    let mut token;
    loop {
        token = lexer.get_next_token();
        println!("{}", token);

        match token {
            Token::Eof => {
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
