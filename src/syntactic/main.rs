use std::{collections::HashMap, fs};

use compiler::{ParseTable, Parser};

// Função principal de exemplo
fn main() {
    let mut symbol_table = HashMap::new();
    let parse_table = ParseTable::create_parse_table("inicio");
    let contents = fs::read_to_string("data.txt").expect("Failed to open the file entry.");
    let mut parser = Parser::new(contents, parse_table, &mut symbol_table);

    match parser.parse() {
        Ok(ast) => {
            println!("✓ Análise sintática bem-sucedida!");
            println!("\nÁrvore Sintática Abstrata:\n{:#?}", ast);
        }
        Err(error) => {
            println!("✗ {}", error);
        }
    };
}
