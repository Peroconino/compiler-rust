use crate::OperatorKind;

// Nó da AST
#[derive(Debug)]
pub enum AstNode {
    BinaryOp {
        op: OperatorKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: OperatorKind,
        expr: Box<AstNode>,
    },
    Number {
        value: String,
    },
    Identifier {
        name: String,
    },
    Assignment {
        id: String,
        expr: Box<AstNode>,
    },
    OperatorMarker(OperatorKind),
    Compound {
        statements: Vec<AstNode>,
    },
    Empty,
}

impl AstNode {
    // Função para imprimir AST de forma bonita
    pub fn print_ast(ast: &AstNode, indent: usize) {
        match ast {
            AstNode::BinaryOp { op, left, right } => {
                println!("{}├─ BinaryOp: {:?}", " ".repeat(indent * 2), op);
                AstNode::print_ast(left, indent + 1);
                AstNode::print_ast(right, indent + 1);
            }
            AstNode::Number { value } => {
                println!("{}├─ Number: {}", " ".repeat(indent * 2), value);
            }
            AstNode::Identifier { name } => {
                println!("{}├─ Identifier: {}", " ".repeat(indent * 2), name);
            }
            AstNode::Empty => {
                println!("{}├─ Empty", " ".repeat(indent * 2));
            }
            _ => {
                println!("{}├─ {:?}", " ".repeat(indent * 2), ast);
            }
        }
    }
}
