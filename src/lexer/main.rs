use std::collections::HashMap;
use std::io;

use compiler::{Lexer, Token};

mod token;

fn main() -> io::Result<()> {
    let mut symbol_table = HashMap::new();

    let mut lexer = Lexer::new("data.txt", &mut symbol_table);

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
