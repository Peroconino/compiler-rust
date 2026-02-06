use crate::{OperatorKind, TokenType};

// Símbolos da gramática
#[derive(Debug, Clone)]
pub enum Symbol {
    Terminal(TokenType),
    NonTerminal(String),
    Epsilon, // ε
    End,     // $
    Action(ActionKind),
}

#[derive(Clone, Debug)]
pub enum ActionKind {
    Math(OperatorKind),

    // Comandos
    Assign,
    CreateBlock,
    CreateIf,
    CreateIfElse,
    CreateWhile,
    CreateDoWhile,
    CreateFor,

    MakeList,
    AppendList,
    CreateDecl,
    CreateCond,

    CreateProgram,
}
// Funções para os simbolos
impl Symbol {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Symbol::Terminal(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, Symbol::NonTerminal(_))
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(self, Symbol::Epsilon)
    }

    pub fn is_end(&self) -> bool {
        matches!(self, Symbol::End)
    }

    pub fn as_terminal(&self) -> Option<TokenType> {
        if let Symbol::Terminal(t) = self {
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn as_non_terminal(&self) -> Option<&String> {
        if let Symbol::NonTerminal(nt) = self {
            Some(nt)
        } else {
            None
        }
    }
}
