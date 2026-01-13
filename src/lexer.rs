use crate::token::{ErrorKind, OperatorKind, Token};

pub struct Lexer {
    pub file_content: Vec<char>,
    pub ini: usize,
    pub prox: usize,
    pub line: usize,
    pub column: usize,
    prev_column: usize,
    prev_line: usize,
}

impl Lexer {
    pub fn new(file_content: String) -> Self {
        Self {
            file_content: file_content.chars().collect(),
            ini: 0,
            prox: 0,
            line: 1,
            column: 0,
            prev_column: 0,
            prev_line: 0,
        }
    }

    pub fn prox_char(&mut self) -> Option<char> {
        if self.prox < self.file_content.len() {
            let ch = self.file_content[self.prox];
            self.prox += 1;
            self.prev_column = self.column;
            self.prev_line = self.line;

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

    fn trata_lookahead(&mut self) {
        self.prox -= 1;
        self.column = self.prev_column;
        self.line = self.prev_line;
    }

    pub fn get_next_token(&mut self) -> Token {
        let mut c: Option<char>;
        let mut state: u16 = 0;

        self.ini = self.prox; // avançando ini após cada token

        // pra descobrir a coluna inicial do lexema
        let column = self.column - (self.prox - self.ini);
        let line = self.line;

        loop {
            let value: String = self.file_content[self.ini..self.prox].iter().collect();
            match state {
                // q0
                0 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == ')' {
                            state = 1; // x2
                        } else if ch == '(' {
                            state = 2; // w2
                        } else if ch == '\'' {
                            state = 4; // v2
                        } else if ch.is_ascii_alphabetic() || ch == '_' {
                            state = 7; // u2
                        } else if ch == '*' {
                            state = 9; // t2
                        }
                    } else {
                        state = 3; // eof
                    }
                }
                // x2
                1 => {
                    return Token::Operator {
                        value,
                        kind: OperatorKind::Pardir,
                        line,
                        column,
                    };
                }
                // w2
                2 => {
                    return Token::Operator {
                        value,
                        kind: OperatorKind::Paresq,
                        line,
                        column,
                    }; // w2
                }
                3 => {
                    return Token::Eof {};
                }
                // v2
                4 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch != '\'' {
                            state = 5; // v3
                        } else {
                            state = 6; // v4 char vazio
                        }
                    } else {
                        state = 3; // eof
                    }
                }
                // v3
                5 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == '\''
                    {
                        state = 6; //v4
                    } else {
                        return Token::Error {
                            value: Some(value),
                            kind: ErrorKind::UnclosedChar,
                            line,
                            column,
                        };
                    }
                }
                // v4
                6 => {
                    // TODO inserir na tabela de simbolos caso nao exista
                    return Token::Id {
                        value,
                        line,
                        column,
                    };
                }
                // u2
                7 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if !(ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_') {
                            state = 8; // u3
                            self.trata_lookahead();
                        }
                    } else {
                        state = 8; // eof
                        self.trata_lookahead();
                    }
                }
                // u3
                8 => {
                    //TODO inserir caso não exista na tabela de simbolos
                    return Token::Id {
                        value,
                        line,
                        column,
                    };
                }
                // t2
                9 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == '*'
                    {
                        state = 10; // t3
                    } else {
                        self.trata_lookahead();
                        state = 11;
                    }
                }
                // t3
                10 => {
                    return Token::Operator {
                        value,
                        kind: OperatorKind::Exp,
                        line,
                        column,
                    };
                }
                // s2
                11 => {
                    return Token::Operator {
                        value,
                        kind: OperatorKind::Mult,
                        line,
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
            kind: ErrorKind::UnknownToken,
            line,
            column,
        }
    }
}
