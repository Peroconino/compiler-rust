use std::{collections::HashMap, fs};
mod token;

pub use token::{
    ErrorKind, KeywordKind, NumberKind, OperatorKind, PunctuationKind, RelopKind, Token, TokenType,
};

pub struct Lexer<'a> {
    pub file_content: Vec<char>,
    pub ini: usize,
    pub prox: usize,
    pub line: usize,
    pub column: usize,
    has_atleast_one_digit: bool,
    has_lookahead: bool,
    lookahead: Option<char>,
    prev_column: usize,
    prev_line: usize,
    symbol_table: &'a mut HashMap<String, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(file_path: &str, symbol_table: &'a mut HashMap<String, Token>) -> Self {
        let contents = fs::read_to_string(file_path).expect("Failed to open the file entry.");

        Self {
            file_content: contents.chars().collect(),
            ini: 0,
            prox: 0,
            line: 1,
            column: 1,
            has_atleast_one_digit: false,
            has_lookahead: false,
            lookahead: None,
            prev_column: 0,
            prev_line: 0,
            symbol_table,
        }
    }

    fn prox_char(&mut self) -> Option<char> {
        if self.prox < self.file_content.len() {
            if self.has_lookahead {
                self.has_lookahead = false;
                return self.lookahead;
            }

            let ch = self.file_content[self.prox];
            self.prox += 1;
            self.prev_column = self.column;
            self.prev_line = self.line;

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            Some(ch)
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.prox < self.file_content.len() {
            if self.has_lookahead {
                return self.lookahead;
            }

            self.lookahead = self.prox_char();
            self.has_lookahead = true;
            self.lookahead
        } else {
            None
        }
    }

    fn ignore_spaces_and_commentaries(&mut self) {
        loop {
            match self.peek_char() {
                Some(ch) if ch.is_ascii_whitespace() => {
                    self.prox_char();
                    self.ini = self.prox;
                }
                Some('{') => {
                    // Pega o próximo char para verificar se é '%'
                    let next = {
                        let _ = self.prox_char(); // Consome '{'
                        self.peek_char()
                    };

                    if next == Some('%') {
                        self.prox_char(); // Consome '%'
                        self.ini = self.prox;

                        // Procura por '%}'
                        loop {
                            match self.prox_char() {
                                Some('%') => {
                                    if self.peek_char() == Some('}') {
                                        self.prox_char(); // Consome '}'
                                        self.ini = self.prox;
                                        break; // Comentário terminado
                                    }
                                }
                                None => break, // EOF
                                _ => {}        // Continua procurando
                            }
                        }
                    } else {
                        // Não era um comentário, devolve o '{'
                        self.trata_lookahead();
                        break;
                    }
                }
                _ => break, // Qualquer outra coisa (inclusive None/EOF)
            }
        }
    }

    fn trata_lookahead(&mut self) {
        self.prox -= 1;
        self.column = self.prev_column;
        self.line = self.prev_line;
        self.has_lookahead = false;
        self.lookahead = None;
    }

    fn get_column(&self) -> usize {
        if self.prox - self.ini <= self.column {
            self.column - (self.prox - self.ini)
        } else {
            self.column
        }
    }

    fn get_value(&self) -> String {
        self.file_content[self.ini..self.prox].iter().collect()
    }

    fn is_letter_digit_or_underscore(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_'
    }

    fn insert_table(&mut self, token: Token) {
        match token.clone() {
            Token::Id { value, .. } => {
                self.symbol_table.entry(value).or_insert(token);
            }
            Token::Char { value, .. } => {
                self.symbol_table.entry(value.to_string()).or_insert(token);
            }
            Token::Number { value, .. } => {
                self.symbol_table.entry(value).or_insert(token);
            }
            _ => {
                panic!("Token is not insertable.");
            }
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        let mut c: Option<char>;
        let mut state: u16 = 0;

        self.ini = self.prox; // avançando ini após cada token
        self.has_atleast_one_digit = false; // resetando booleano

        self.ignore_spaces_and_commentaries();
        loop {
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
                        } else if ch == '*' {
                            state = 9; // t2
                        } else if ch == '/' {
                            state = 12; // s3
                        } else if ch == '-' {
                            state = 13; // r3
                        } else if ch == '+' {
                            state = 14; // r2
                        } else if ch == '!' {
                            state = 15; // q10
                        } else if ch == '<' {
                            state = 17; // q7
                        } else if ch == '>' {
                            state = 20; // q4
                        } else if ch == '=' {
                            state = 23; // q2
                        } else if ch == ',' {
                            state = 25; // y3
                        } else if ch == ';' {
                            state = 26; // y2
                        } else if ch == ':' {
                            state = 27; // z2
                        } else if ch.is_ascii_digit() {
                            state = 29; // b2
                        } else if ch == 'm' {
                            state = 36; // d2
                        } else if ch == 'v' {
                            state = 41; // e2
                        } else if ch == 'i' {
                            state = 46; // f2
                        } else if ch == 'c' {
                            state = 52; // g2
                        } else if ch == 't' {
                            state = 57; // j2
                        } else if ch == 'e' {
                            state = 66; // k2
                        } else if ch == 'w' {
                            state = 74; // m2
                        } else if ch == 'd' {
                            state = 80; // n2
                        } else if ch == 'f' {
                            state = 83; // o2
                        } else if ch == '[' {
                            state = 92; // p2
                        } else if ch == ']' {
                            state = 93; // y2
                        } else if ch.is_ascii_alphabetic() || ch == '_' {
                            state = 7; // u2
                        }
                    } else {
                        state = 3; // eof
                    }
                }
                // x2
                1 => {
                    return Ok(Token::Operator {
                        kind: OperatorKind::Pardir,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // w2
                2 => {
                    return Ok(Token::Operator {
                        kind: OperatorKind::Paresq,
                        line: self.line,
                        column: self.get_column(),
                    }); // w2
                }
                3 => {
                    return Ok(Token::Eof);
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
                        self.trata_lookahead();
                        return Err(format!(
                            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                            value = self.get_value(),
                            kind = ErrorKind::UnclosedChar,
                            line = self.line,
                            col = self.get_column()
                        ));
                    }
                }
                // v4
                6 => {
                    let token = Token::Char {
                        value: self
                            .get_value()
                            .strip_prefix("'")
                            .unwrap()
                            .strip_suffix("'")
                            .unwrap()
                            .chars()
                            .last()
                            .unwrap(),
                        line: self.line,
                        column: self.get_column(),
                    };

                    //TODO inserir na tabela
                    self.insert_table(token.clone());
                    return Ok(token);
                }
                // u2
                7 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if !(self.is_letter_digit_or_underscore(ch)) {
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
                    let token = Token::Id {
                        value: self.get_value(),
                        line: self.line,
                        column: self.get_column(),
                    };

                    self.insert_table(token.clone());

                    return Ok(token);
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
                    return Ok(Token::Operator {
                        kind: OperatorKind::Exp,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // s2
                11 => {
                    return Ok(Token::Operator {
                        kind: OperatorKind::Mult,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // s3
                12 => {
                    return Ok(Token::Operator {
                        kind: OperatorKind::Div,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // r3
                13 => {
                    return Ok(Token::Operator {
                        kind: OperatorKind::Sub,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // r2
                14 => {
                    return Ok(Token::Operator {
                        kind: OperatorKind::Sum,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // q10
                15 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == '=' {
                            state = 16;
                        } else {
                            return Err(format!(
                                "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                                value = self.get_value(),
                                kind = ErrorKind::InvalidTokenAfterExclamation,
                                line = self.line,
                                col = self.get_column()
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                            value = self.get_value(),
                            kind = ErrorKind::InvalidTokenAfterExclamation,
                            line = self.line,
                            col = self.get_column()
                        ));
                    }
                }
                // q11
                16 => {
                    return Ok(Token::Relop {
                        kind: RelopKind::NE,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // q7
                17 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == '='
                    {
                        state = 18; // q8
                    } else {
                        state = 19; // q9
                        self.trata_lookahead();
                    }
                }
                // q8
                18 => {
                    return Ok(Token::Relop {
                        kind: RelopKind::LE,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // q9
                19 => {
                    return Ok(Token::Relop {
                        kind: RelopKind::LT,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // q4
                20 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == '='
                    {
                        state = 21; // q5
                    } else {
                        state = 22; // q6
                        self.trata_lookahead();
                    }
                }
                // q5
                21 => {
                    return Ok(Token::Relop {
                        kind: RelopKind::GE,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // q6
                22 => {
                    return Ok(Token::Relop {
                        kind: RelopKind::GT,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // q2
                23 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == '='
                    {
                        state = 24; // q3
                    } else {
                        self.trata_lookahead();
                        return Err(format!(
                            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                            value = self.get_value(),
                            kind = ErrorKind::MissingEqual,
                            line = self.line,
                            col = self.get_column()
                        ));
                    }
                }
                // q3
                24 => {
                    return Ok(Token::Relop {
                        kind: RelopKind::EQ,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // y3
                25 => {
                    return Ok(Token::Punctuation {
                        kind: PunctuationKind::Comma,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // y2
                26 => {
                    return Ok(Token::Punctuation {
                        kind: PunctuationKind::EndExp,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // z2
                27 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch == '='
                    {
                        state = 28; // z3
                    } else {
                        self.trata_lookahead();
                        return Err(format!(
                            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                            value = self.get_value(),
                            kind = ErrorKind::MissingEqual,
                            line = self.line,
                            col = self.get_column()
                        ));
                    }
                }
                // z3
                28 => {
                    return Ok(Token::Punctuation {
                        kind: PunctuationKind::Assigment,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // b2
                29 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == '.' {
                            state = 30; // b4
                        } else if ch == 'E' {
                            state = 32; // b5
                        } else if ch.is_ascii_digit() {
                        } else {
                            self.trata_lookahead();
                            state = 31; // b3
                        }
                    } else {
                        self.trata_lookahead();
                        state = 31;
                    }
                }
                // b4
                30 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'E' {
                            state = 32; // b5
                        } else if ch.is_ascii_digit() {
                            self.has_atleast_one_digit = true;
                        } else if self.has_atleast_one_digit {
                            self.trata_lookahead();
                            state = 33; // b8
                        } else {
                            self.trata_lookahead();
                            return Err(format!(
                                "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                                value = self.get_value(),
                                kind = ErrorKind::FractionEndedWithADot,
                                line = self.line,
                                col = self.get_column()
                            ));
                        }
                    } else if self.has_atleast_one_digit {
                        self.trata_lookahead();
                        state = 33; // b8
                    } else {
                        self.trata_lookahead();
                        return Err(format!(
                            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                            value = self.get_value(),
                            kind = ErrorKind::FractionEndedWithADot,
                            line = self.line,
                            col = self.get_column()
                        ));
                    }
                }
                // b3
                31 => {
                    let token = Token::Number {
                        value: self.get_value(),
                        kind: NumberKind::Integer,
                        line: self.line,
                        column: self.get_column(),
                    };

                    self.insert_table(token.clone());

                    return Ok(token);
                }
                // b5
                32 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == '+' || ch == '-' {
                            state = 34; // b6
                        } else if ch.is_ascii_digit() {
                            state = 35; // b7
                        } else {
                            self.trata_lookahead();
                            return Err(format!(
                                "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                                value = self.get_value(),
                                kind = ErrorKind::EndedWithEExpoent,
                                line = self.line,
                                col = self.get_column()
                            ));
                        }
                    } else {
                        self.trata_lookahead();
                        return Err(format!(
                            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                            value = self.get_value(),
                            kind = ErrorKind::EndedWithEExpoent,
                            line = self.line,
                            col = self.get_column()
                        ));
                    }
                }
                // b8
                33 => {
                    let token = Token::Number {
                        value: self.get_value(),
                        kind: NumberKind::Float,
                        line: self.line,
                        column: self.get_column(),
                    };

                    self.insert_table(token.clone());

                    return Ok(token);
                }
                // b6
                34 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch.is_ascii_digit()
                    {
                        state = 35; // b7 
                    } else {
                        self.trata_lookahead();
                        return Err(format!(
                            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
                            value = self.get_value(),
                            kind = ErrorKind::EndedAfterExpoentSign,
                            line = self.line,
                            col = self.get_column()
                        ));
                    }
                }
                // b7
                35 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && ch.is_ascii_digit()
                    {
                    } else {
                        self.trata_lookahead();
                        state = 33;
                    }
                }
                // d2
                36 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'a' {
                            state = 37; // d3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // d3
                37 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'i' {
                            state = 38; // d4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // d4
                38 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'n' {
                            state = 39; // d5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // d5
                39 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7 // u2
                    } else {
                        self.trata_lookahead();
                        state = 40; // d6
                    }
                }
                // d6
                40 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Main,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // e2
                41 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'o' {
                            state = 42; // e3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // e3
                42 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'i' {
                            state = 43; // e4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // e4
                43 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'd' {
                            state = 44; // e5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // e5
                44 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7 // u2
                    } else {
                        self.trata_lookahead();
                        state = 45; // e6
                    }
                }
                // e6
                45 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Void,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // f2
                46 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'f' {
                            state = 47; // i1
                        } else if ch == 'n' {
                            state = 48; // f3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // i1
                47 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7 // u2
                    } else {
                        self.trata_lookahead();
                        state = 49; // i2
                    }
                }
                // f3
                48 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 't' {
                            state = 50; // f4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // i2
                49 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::If,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // f4
                50 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7 // u2
                    } else {
                        self.trata_lookahead();
                        state = 51; // f5
                    }
                }
                // f5
                51 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Int,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // g2
                52 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'h' {
                            state = 53; // g3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // g3
                53 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'a' {
                            state = 54; // g4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // g4
                54 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'r' {
                            state = 55; // g5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // g5
                55 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; //u2
                    } else {
                        self.trata_lookahead();
                        state = 56; // g6
                    }
                }
                // g6
                56 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Char,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // j2
                57 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'i' {
                            state = 58; // c2
                        } else if ch == 'h' {
                            state = 59; // // j3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // c2
                58 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'p' {
                            state = 60; // c4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // j3
                59 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'e' {
                            state = 61; // j4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // c4
                60 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'o' {
                            state = 62; // c5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // j4
                61 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'n' {
                            state = 63; // j5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // c5
                62 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 64; // c6
                    }
                }
                // j5
                63 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 65; // j6
                    }
                }
                // c6
                64 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Type,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // j6
                65 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Then,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // k2
                66 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'l' {
                            state = 67; // k3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // k3
                67 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 's' {
                            state = 68; // k4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // k4
                68 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'e' {
                            state = 69; // l1
                        } else if ch == 'i' {
                            state = 70; // k5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // l1
                69 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 71; // l2
                    }
                }
                // k5
                70 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'f' {
                            state = 72; // k6
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // l2
                71 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Else,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // k6
                72 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 73; // k7
                    }
                }
                // k7
                73 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Elsif,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // m2
                74 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'h' {
                            state = 75; // m3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // m3
                75 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'i' {
                            state = 76; // m4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // m4
                76 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'l' {
                            state = 77; // m5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // m5
                77 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'e' {
                            state = 78; // m6
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // m6
                78 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 79; // m7
                    }
                }
                // m7
                79 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::While,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // n2
                80 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'o' {
                            state = 81; // n3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // n3
                81 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 82; // n4
                    }
                }
                // n4
                82 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Do,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // o2
                83 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'l' {
                            state = 84; // h3
                        } else if ch == 'o' {
                            state = 85; // o3
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // h3
                84 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'o' {
                            state = 86; // h4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // o3
                85 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'r' {
                            state = 87; // o4
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // h4
                86 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 'a' {
                            state = 88; // h5
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // o4
                87 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 89; // o5
                    }
                }
                // h5
                88 => {
                    c = self.prox_char();
                    if let Some(ch) = c {
                        if ch == 't' {
                            state = 90; // h6
                        } else if !self.is_letter_digit_or_underscore(ch) {
                            self.trata_lookahead();
                            state = 8; // u3
                        } else {
                            state = 7; // u2
                        }
                    } else {
                        self.trata_lookahead();
                        state = 8; // u3
                    }
                }
                // o5
                89 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::For,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // h6
                90 => {
                    c = self.prox_char();
                    if let Some(ch) = c
                        && self.is_letter_digit_or_underscore(ch)
                    {
                        state = 7; // u2
                    } else {
                        self.trata_lookahead();
                        state = 91; // h7
                    }
                }
                // h7
                91 => {
                    return Ok(Token::Keyword {
                        kind: KeywordKind::Float,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // p2
                92 => {
                    return Ok(Token::Punctuation {
                        kind: PunctuationKind::BeginBlock,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                // y2
                93 => {
                    return Ok(Token::Punctuation {
                        kind: PunctuationKind::EndBlock,
                        line: self.line,
                        column: self.get_column(),
                    });
                }
                _ => {
                    break;
                }
            };
        }

        Err(format!(
            "Erro léxico: {value}\n Tipo: {kind:?}\n linha: {line}\n coluna: {col}",
            value = self.get_value(),
            kind = ErrorKind::UnknownToken,
            line = self.line,
            col = self.get_column()
        ))
    }
}
