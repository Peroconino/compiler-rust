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

        match token {
            Ok(token) => {
                println!("{}", token);
                if token == Token::Eof {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}
