use std::fmt::{Display, Error, Formatter};

#[derive(Display)]
pub enum NumberKind {
    Integer,
    Float,
}

#[derive(Display)]
pub enum RelopKind {
    GT,
    LT,
    LE,
    EQ,
    NE,
    GE,
}

#[derive(Display)]
pub enum OperatorKind {
    Sum,
    Sub,
    Mult,
    Div,
    Exp,
    Paresq,
    Pardir,
}

#[derive(Display)]
pub enum PunctuationKind {
    Assigment,
    Comma,
    EndExp,
    BeginBlock,
    EndBlock,
}

#[derive(Display)]
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

#[derive(Display)]
pub enum ErrorKind {
    UnclosedChar,
    InvalidTokenAfterExclamation,
    FractionEndedWithADot,
    EndedWithEExpoent,
    EndedAfterExpoentSign,
    MissingEqual,
    UnknownToken,
}

pub enum Token {
    Id {
        value: String,
        line: usize,
        column: usize,
    },
    Char {
        value: char,
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
    Eof {},
    Error {
        value: Option<String>,
        kind: ErrorKind,
        line: usize,
        column: usize,
    },
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
            Self::Char {
                value,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Char, value='{}', line={}, column={}>",
                    value, line, column
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
            Self::Error {
                value,
                kind,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Error, value='{}', kind={}, line={}, column={}>",
                    value.clone().unwrap_or("".to_owned()),
                    kind,
                    line,
                    column,
                );
            }
            Self::Eof {} => {
                let _ = write!(f, "End of file");
            }
        }
        Ok(())
    }
}
