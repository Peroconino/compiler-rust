use std::io;
use std::{collections::HashMap, fs};

use compiler::{Lexer, Token};

mod token;

fn main() -> io::Result<()> {
    let mut symbol_table = HashMap::new();

    let contents = fs::read_to_string("data.txt").expect("Failed to open the file entry.");
    let mut lexer = Lexer::new(contents, &mut symbol_table);

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
