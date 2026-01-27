use std::{cell::RefCell, collections::HashMap, fs, io, rc::Rc};

use compiler::{Lexer, Token};

// Função principal de exemplo
fn main() -> io::Result<()> {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path)?;
    let symbol_table = Rc::new(RefCell::new(HashMap::new()));

    let mut lexer = Lexer::new(contents, Rc::clone(&symbol_table));

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
