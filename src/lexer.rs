use crate::token::{ErrorKind, OperatorKind, Token};

pub struct Lexer {
    pub file_content: Vec<char>,
    pub lookahead: Option<char>,
    pub has_lookahead: bool,
    pub ini: usize,
    pub prox: usize,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(file_content: String) -> Self {
        Self {
            file_content: file_content.chars().collect(),
            lookahead: None,
            has_lookahead: false,
            ini: 0,
            prox: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn prox_char(&mut self) -> Option<char> {
        if self.prox < self.file_content.len() {
            if self.has_lookahead {
                self.has_lookahead = false;
                return self.lookahead;
            }

            let ch = self.file_content[self.prox];
            self.prox += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            Some(ch)
        } else {
            None
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        if self.has_lookahead {
            return self.lookahead;
        }

        self.has_lookahead = true;
        self.lookahead = self.prox_char();
        self.lookahead
    }

    pub fn get_next_token(&mut self) -> Token {
        let mut c: Option<char>;
        let mut state: u16 = 0;

        if self.has_lookahead {
            self.ini = self.prox - 1;
        } else {
            self.ini = self.prox;
        }

        let column = self.column - (self.prox - self.ini);

        loop {
            match state {
                0 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == ')'
                    {
                        state = 1; // x2
                    } else if let Some(ch) = c
                        && ch == '('
                    {
                        state = 2; // w2
                    } else if c.is_none() {
                        state = 3;
                    } else if let Some(ch) = c
                        && ch == '\''
                    {
                        state = 4; // v2
                    }
                }
                1 => {
                    return Token::Operator {
                        value: self.file_content[self.ini..self.prox].iter().collect(),
                        kind: OperatorKind::Pardir,
                        line: self.line,
                        column,
                    }; // x2
                }
                2 => {
                    return Token::Operator {
                        value: self.file_content[self.ini..self.prox].iter().collect(),
                        kind: OperatorKind::Paresq,
                        line: self.line,
                        column,
                    }; // w2
                }
                3 => {
                    return Token::Eof {};
                }
                4 => {
                    // TODO atualizar diagrama v, ele pula pro estado final caso ler outro '
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch != '\''
                    {
                        state = 5; // v3
                    } else {
                        state = 6; // v4
                    }
                }
                5 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == '\''
                    {
                        state = 6; //v4
                    } else {
                        return Token::Error {
                            value: Some(self.file_content[self.ini..self.prox].iter().collect()),
                            kind: ErrorKind::UnclosedChar,
                            line: self.line,
                            column,
                        };
                    }
                }
                6 => {
                    return Token::Id {
                        value: self.file_content[self.ini..self.prox].iter().collect(),
                        line: self.line,
                        column,
                    };
                }
                _ => {
                    break;
                }
            };
        }

        Token::Error {
            value: None,
            kind: ErrorKind::Undefined,
            line: self.line,
            column,
        }
    }
}
