use std::fmt::{Display, Error, Formatter};

use strum_macros::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum NumberKind {
    Integer,
    Float,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum RelopKind {
    GT,
    LT,
    LE,
    EQ,
    NE,
    GE,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum OperatorKind {
    Sum,
    Sub,
    Mult,
    Div,
    Exp,
    Paresq,
    Pardir,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum PunctuationKind {
    Assigment,
    Comma,
    EndExp,
    BeginBlock,
    EndBlock,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum KeywordKind {
    If,
    Int,
    Float,
    Char,
    Then,
    Type,
    Else,
    Elsif,
    While,
    For,
    Do,
    Main,
    Void,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    UnclosedChar,
    InvalidTokenAfterExclamation,
    FractionEndedWithADot,
    EndedWithEExpoent,
    EndedAfterExpoentSign,
    MissingEqual,
    UnknownToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Id {
        value: String,
        line: usize,
        column: usize,
    },
    Number {
        value: String,
        kind: NumberKind,
        line: usize,
        column: usize,
    },
    Relop {
        kind: RelopKind,
        line: usize,
        column: usize,
    },
    Operator {
        kind: OperatorKind,
        line: usize,
        column: usize,
    },
    Punctuation {
        kind: PunctuationKind,
        line: usize,
        column: usize,
    },
    Keyword {
        kind: KeywordKind,
        line: usize,
        column: usize,
    },
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Id {
                value,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Id, value='{}', line={}, column={}>",
                    value, line, column,
                );
            }
            Self::Number {
                value,
                kind,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Number, value='{}', kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Relop { kind, line, column } => {
                let value = match kind {
                    RelopKind::GT => ">".to_string(),
                    RelopKind::LT => "<".to_string(),
                    RelopKind::EQ => "==".to_string(),
                    RelopKind::NE => "!=".to_string(),
                    RelopKind::LE => "<=".to_string(),
                    RelopKind::GE => ">=".to_string(),
                };

                let _ = write!(
                    f,
                    "<Relop, value='{}', kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Operator { kind, line, column } => {
                let value = match kind {
                    OperatorKind::Sum => "+".to_string(),
                    OperatorKind::Sub => "-".to_string(),
                    OperatorKind::Mult => "*".to_string(),
                    OperatorKind::Div => "/".to_string(),
                    OperatorKind::Exp => "**".to_string(),
                    OperatorKind::Paresq => "(".to_string(),
                    OperatorKind::Pardir => ")".to_string(),
                };

                let _ = write!(
                    f,
                    "<Operator, value='{}', kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Punctuation { kind, line, column } => {
                let value = match kind {
                    PunctuationKind::Assigment => ":=".to_string(),
                    PunctuationKind::Comma => ",".to_string(),
                    PunctuationKind::EndExp => ";".to_string(),
                    PunctuationKind::BeginBlock => "[".to_string(),
                    PunctuationKind::EndBlock => "]".to_string(),
                };

                let _ = write!(
                    f,
                    "<Punctuation, value='{}', kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Keyword { kind, line, column } => {
                let value = match kind {
                    KeywordKind::If => "if".to_string(),
                    KeywordKind::Int => "int".to_string(),
                    KeywordKind::Float => "float".to_string(),
                    KeywordKind::Char => "char".to_string(),
                    KeywordKind::Then => "then".to_string(),
                    KeywordKind::Type => "tipo".to_string(),
                    KeywordKind::Else => "else".to_string(),
                    KeywordKind::Elsif => "elsif".to_string(),
                    KeywordKind::While => "while".to_string(),
                    KeywordKind::For => "for".to_string(),
                    KeywordKind::Do => "do".to_string(),
                    KeywordKind::Main => "main".to_string(),
                    KeywordKind::Void => "void".to_string(),
                };
                let _ = write!(
                    f,
                    "<Keyword, value='{}', kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Eof {} => {
                let _ = write!(f, "End of file");
            }
        }
        Ok(())
    }
}

// Tipos de tokens (simplificado)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Id, // identificador
    CharValue,
    Number,
    GTOperator,
    LTOperator,
    LEOperator,
    NEOperator,
    EQOperator,
    GEOperator,
    PlusOperator,
    MinusOperator,
    MultOperator,
    DivOperator,
    ExpOperator,
    LParenOperator,
    RParenOperator,
    AssignPunctuation,
    CommaPunctuation,
    SemiColonPunctuation,
    BeginBlockPunctuation,
    EndBlockPunctuation,
    IfKeyword,
    IntKeyword,
    FloatKeyword,
    CharKeyword,
    ThenKeyword,
    TypeKeyword,
    ElseKeyword,
    ElsifKeyword,
    WhileKeyword,
    ForKeyword,
    DoKeyword,
    MainKeyword,
    VoidKeyword,
    Eof,
    UnclosedCharErr,
    InvalidTokenAfterExclamationErr,
    FractionEndedWithADotErr,
    EndedWithEExpoentErr,
    EndedAfterExpoentSignErr,
    MissingEqualErr,
    UknownTokenErr,
}

impl From<Token> for TokenType {
    fn from(token: Token) -> Self {
        match token {
            Token::Id { .. } => Self::Id,
            Token::Number {
                kind: NumberKind::Integer,
                ..
            }
            | Token::Number {
                kind: NumberKind::Float,
                ..
            } => Self::Number,

            Token::Relop {
                kind: RelopKind::GT,
                ..
            } => Self::GTOperator,
            Token::Relop {
                kind: RelopKind::LT,
                ..
            } => Self::LTOperator,
            Token::Relop {
                kind: RelopKind::LE,
                ..
            } => Self::LEOperator,
            Token::Relop {
                kind: RelopKind::EQ,
                ..
            } => Self::EQOperator,
            Token::Relop {
                kind: RelopKind::NE,
                ..
            } => Self::NEOperator,
            Token::Relop {
                kind: RelopKind::GE,
                ..
            } => Self::GEOperator,
            Token::Operator {
                kind: OperatorKind::Sum,
                ..
            } => Self::PlusOperator,
            Token::Operator {
                kind: OperatorKind::Sub,
                ..
            } => Self::MinusOperator,
            Token::Operator {
                kind: OperatorKind::Mult,
                ..
            } => Self::MultOperator,
            Token::Operator {
                kind: OperatorKind::Div,
                ..
            } => Self::DivOperator,
            Token::Operator {
                kind: OperatorKind::Exp,
                ..
            } => Self::ExpOperator,
            Token::Operator {
                kind: OperatorKind::Paresq,
                ..
            } => Self::LParenOperator,
            Token::Operator {
                kind: OperatorKind::Pardir,
                ..
            } => Self::RParenOperator,
            Token::Punctuation {
                kind: PunctuationKind::Assigment,
                ..
            } => Self::AssignPunctuation,
            Token::Punctuation {
                kind: PunctuationKind::Comma,
                ..
            } => Self::CommaPunctuation,
            Token::Punctuation {
                kind: PunctuationKind::EndExp,
                ..
            } => Self::SemiColonPunctuation,
            Token::Punctuation {
                kind: PunctuationKind::BeginBlock,
                ..
            } => Self::BeginBlockPunctuation,
            Token::Punctuation {
                kind: PunctuationKind::EndBlock,
                ..
            } => Self::EndBlockPunctuation,
            Token::Keyword {
                kind: KeywordKind::If,
                ..
            } => Self::IfKeyword,
            Token::Keyword {
                kind: KeywordKind::Int,
                ..
            } => Self::IntKeyword,
            Token::Keyword {
                kind: KeywordKind::Float,
                ..
            } => Self::FloatKeyword,
            Token::Keyword {
                kind: KeywordKind::Char,
                ..
            } => Self::CharKeyword,
            Token::Keyword {
                kind: KeywordKind::Then,
                ..
            } => Self::ThenKeyword,
            Token::Keyword {
                kind: KeywordKind::Type,
                ..
            } => Self::TypeKeyword,
            Token::Keyword {
                kind: KeywordKind::Else,
                ..
            } => Self::ElseKeyword,
            Token::Keyword {
                kind: KeywordKind::Elsif,
                ..
            } => Self::ElsifKeyword,
            Token::Keyword {
                kind: KeywordKind::While,
                ..
            } => Self::WhileKeyword,
            Token::Keyword {
                kind: KeywordKind::For,
                ..
            } => Self::ForKeyword,
            Token::Keyword {
                kind: KeywordKind::Do,
                ..
            } => Self::DoKeyword,
            Token::Keyword {
                kind: KeywordKind::Main,
                ..
            } => Self::MainKeyword,
            Token::Keyword {
                kind: KeywordKind::Void,
                ..
            } => Self::VoidKeyword,
            Token::Eof => Self::Eof,
        }
    }
}
