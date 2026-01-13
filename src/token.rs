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
    MulVars,
    EndExp,
    Declaration,
    Apostrophe,
    Dot,
}

#[derive(Display)]
pub enum KeywordKind {
    If,
    Then,
    Else,
    While,
    Do,
    Main,
    Begin,
    End,
    Repeat,
    Until,
}

#[derive(Display)]
pub enum ErrorKind {
    Undefined,
    UnclosedComment,
    UnclosedChar,
    InvalidTokenAfterExclamation,
    FractionEndedWithADot,
    EndedWithEExpoent,
    EndedAfterExpoentSign,
    UnknownToken,
}

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
        value: String,
        kind: RelopKind,
        line: usize,
        column: usize,
    },
    Operator {
        value: String,
        kind: OperatorKind,
        line: usize,
        column: usize,
    },
    Punctuation {
        value: String,
        kind: PunctuationKind,
        line: usize,
        column: usize,
    },
    Keyword {
        value: String,
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
                let _ = write!(f, "<Id, value={}, line={}, column={}>", value, line, column,);
            }
            Self::Number {
                value,
                kind,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Number, value={}, kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Relop {
                value,
                kind,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Relop, value={}, kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Operator {
                value,
                kind,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Operator, value={}, kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Punctuation {
                value,
                kind,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Punctuation, value={}, kind={}, line={}, column={}>",
                    value, kind, line, column,
                );
            }
            Self::Keyword {
                value,
                kind,
                line,
                column,
            } => {
                let _ = write!(
                    f,
                    "<Keyword, value={}, kind={}, line={}, column={}>",
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
                    "<Error, value={}, kind={}, line={}, column={}>",
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
