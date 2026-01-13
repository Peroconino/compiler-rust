pub enum NumberKind {
    Integer,
    Float,
}

pub enum RelopKind {
    GT,
    LT,
    LE,
    EQ,
    NE,
    GE,
}

pub enum OperatorKind {
    Sum,
    Sub,
    Mult,
    Div,
    Exp,
    Paresq,
    Pardir,
}

pub enum PunctuationKind {
    Assigment,
    MulVars,
    EndExp,
    Declaration,
    Apostrophe,
    Dot,
}

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

pub enum ErrorKind {
    Undefined,
    UnclosedComment,
    InvalidTokenAfterExclamation,
    FractionEndedWithADot,
    EndedWithEExpoent,
    EndedAfterExpoentSign,
    UnknownToken,
}

pub enum Token {
    Id {
        value: String,
        line: u32,
        column: u32,
    },
    Number {
        value: String,
        kind: NumberKind,
        line: u32,
        column: u32,
    },
    Relop {
        value: String,
        kind: RelopKind,
        line: u32,
        column: u32,
    },
    Operator {
        value: String,
        kind: OperatorKind,
        line: u32,
        column: u32,
    },
    Punctuation {
        value: String,
        kind: PunctuationKind,
        line: u32,
        column: u32,
    },
    Keyword {
        value: String,
        kind: KeywordKind,
        line: u32,
        column: u32,
    },
    Eof {},
    Error {
        value: String,
        kind: ErrorKind,
        line: u32,
        column: u32,
    },
}

impl Token {
    pub fn default() -> Self {
        Self::Eof {}
    }
}
