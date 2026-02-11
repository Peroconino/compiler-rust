use crate::{OperatorKind, RelopKind, Type};

// Nó da AST
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program {
        kind: Type,
        body: Box<AstNode>,
    },
    Block {
        decls: Vec<AstNode>,
        stmts: Vec<AstNode>,
    },
    VarDecl {
        kind: Type,
        names: Vec<String>,
    },
    Assignment {
        id: String,
        expr: Box<AstNode>,
    },
    If {
        cond: Box<AstNode>,
        then_block: Box<AstNode>,
        else_block: Option<Box<AstNode>>,
    },
    While {
        cond: Box<AstNode>,
        body: Box<AstNode>,
    },
    DoWhile {
        body: Box<AstNode>,
        cond: Box<AstNode>,
    },
    For {
        id: String,
        start: u32,
        end: u32,
        step: Box<AstNode>,
        body: Box<AstNode>,
    },
    UnaryOp {
        // aceita apenas subtração como unário então não precisa de campo 'op: OperatorKind'
        expr: Box<AstNode>,
    },
    BinaryOp {
        op: OperatorKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    BinaryComp {
        relop: RelopKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Number {
        value: String,
    },
    Identifier {
        name: String,
    },
    Literal {
        value: char,
    },
    TypeWrapper(Type),
    CondWrapper(RelopKind),
    List(Vec<AstNode>),
}

impl AstNode {
    // TODO melhorar essa função para aceitar os novos nós
    pub fn print_ast(ast: &AstNode, indent: usize) {
        todo!();
        // match ast {
        //     AstNode::BinaryOp { op, left, right } => {
        //         println!("{}├─ BinaryOp: {:?}", " ".repeat(indent * 2), op);
        //         AstNode::print_ast(left, indent + 1);
        //         AstNode::print_ast(right, indent + 1);
        //     }
        //     AstNode::Number { value } => {
        //         println!("{}├─ Number: {}", " ".repeat(indent * 2), value);
        //     }
        //     AstNode::Identifier { name } => {
        //         println!("{}├─ Identifier: {}", " ".repeat(indent * 2), name);
        //     }
        //     _ => {
        //         println!("{}├─ {:?}", " ".repeat(indent * 2), ast);
        //     }
        // }
    }
}
