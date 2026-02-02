use std::{cell::RefCell, collections::HashMap, rc::Rc};

use compiler::{AstNode, ParseTable, Parser};

// Função principal de exemplo
fn main() {
    let symbol_table = Rc::new(RefCell::new(HashMap::new()));
    let parse_table = ParseTable::create_expression_parse_table();
    let mut parser = Parser::new(parse_table, Rc::clone(&symbol_table));

    match parser.parse() {
        Ok(ast) => {
            println!("✓ Análise sintática bem-sucedida!");
            println!("\nÁrvore Sintática Abstrata:");
            AstNode::print_ast(&ast, 0);
            println!("\nRepresentação Debug:");
            println!("{:#?}", ast);
        }
        Err(error) => {
            println!("✗ Erros encontrados: {}", error);
        }
    }
}
