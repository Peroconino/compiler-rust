use crate::token::{OperatorKind, Token};

pub struct Lexer {
    pub file_content: Vec<char>,
    pub lookahead: char,
    pub has_lookahead: bool,
    pub eof: bool,
    pub ini: usize,
    pub prox: usize,
    pub line: u32,
    pub column: u32,
}

impl Lexer {
    pub fn new(file_content: String) -> Self {
        Self {
            file_content: file_content.chars().collect(),
            lookahead: char::default(),
            has_lookahead: false,
            eof: false,
            ini: 0,
            prox: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn prox_char(&mut self) -> char {
        if self.has_lookahead {
            self.has_lookahead = false;
            return self.lookahead;
        }

        let ch = self.file_content[self.prox];
        self.prox += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
    }

    pub fn peek_char(&mut self) -> char {
        if self.has_lookahead {
            return self.lookahead;
        }

        self.has_lookahead = true;
        self.lookahead = self.prox_char();
        self.lookahead
    }

    pub fn get_next_token(&mut self) -> Token {
        let mut c: char;
        let mut state: u16 = 0;
        let mut token: Token;
        let lexeme_len =  (self.prox - self.ini);
        loop {
            match state {
                0 => {
                    c = self.prox_char();
                    if c == ')' {
                        state = 1;
                    }
                }
                1 => {
                    return Token::Operator {
                        value: self.file_content[self.ini..self.prox].iter().collect(),
                        kind: OperatorKind::Pardir,
                        line: self.line,
                        column: self.column,
                    };
                }
                2 => {
                    c = self.prox_char();
                    if c == '(' {
                        state = 3;
                    }
                }
                3 => {
                    return Token::Operator {
                        value: self.file_content[self.ini..self.prox].iter().collect(),
                        kind: OperatorKind::Paresq,
                        line: self.line,
                        column: self.column,
                    };
                }
                _ => {
                    break;
                }
            };
        }

        todo!()
    }
}
