use crate::Token;

// Símbolos da gramática
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(Token),
    NonTerminal(String),
    Epsilon, // ε
    End,     // $
}

// Nó da AST
pub enum AstNode {
    BinaryOp {
        op: Token,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: Token,
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
    Compound {
        statements: Vec<AstNode>,
    },
    Empty,
}
