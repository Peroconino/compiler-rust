use std::collections::HashMap;

use compiler::{AstNode, ParseTable, Parser};

// Função principal de exemplo
fn main() {
    let mut symbol_table = HashMap::new();
    let parse_table = ParseTable::create_parse_table();
    let mut parser = Parser::new(parse_table, &mut symbol_table);

    match parser.parse() {
        Ok(ast) => {
            println!("✓ Análise sintática bem-sucedida!");
            println!("\nÁrvore Sintática Abstrata:");
            AstNode::print_ast(&ast, 0);
        }
        Err(error) => {
            println!("✗ {}", error);
        }
    };
}
