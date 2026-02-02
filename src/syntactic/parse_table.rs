use std::collections::HashMap;

use crate::{OperatorKind, TokenType, syntactic::symbol::Symbol};

// Tabela de análise LL(1)
pub struct ParseTable {
    table: HashMap<(String, TokenType), Vec<Symbol>>,
    pub start_symbol: String,
}

// Funções da tabela de análise
impl ParseTable {
    pub fn new(start_symbol: &str) -> Self {
        ParseTable {
            table: HashMap::new(),
            start_symbol: start_symbol.to_string(),
        }
    }

    pub fn set_entry(&mut self, non_terminal: &str, terminal: TokenType, symbols: Vec<Symbol>) {
        self.table
            .insert((non_terminal.to_string(), terminal), symbols);
    }

    pub fn get_entry(&self, non_terminal: &str, terminal: &TokenType) -> Option<&Vec<Symbol>> {
        self.table
            .get(&(non_terminal.to_string(), terminal.clone()))
    }

    // Exemplo de gramática para expressões aritméticas simples
    // Gramática:
    // E  → T E'
    // E' → + T E' | - T E' | ε
    // T  → F T'
    // T' → * F T' | / F T' | ε
    // F  → U F'
    // F' → ** U F' | ε
    // U  → id | num | ( E )

    pub fn create_expression_parse_table() -> ParseTable {
        let mut table = ParseTable::new("E");

        // Define os símbolos
        let e = || Symbol::NonTerminal("E".to_string());
        let e_prime = || Symbol::NonTerminal("E'".to_string());
        let t = || Symbol::NonTerminal("T".to_string());
        let t_prime = || Symbol::NonTerminal("T'".to_string());
        let f = || Symbol::NonTerminal("F".to_string());
        let f_prime = || Symbol::NonTerminal("F'".to_string());
        let u = || Symbol::NonTerminal("U".to_string());

        let plus = || Symbol::Terminal(TokenType::PlusOperator);
        let minus = || Symbol::Terminal(TokenType::MinusOperator);
        let mult = || Symbol::Terminal(TokenType::MultOperator);
        let div = || Symbol::Terminal(TokenType::DivOperator);
        let exp = || Symbol::Terminal(TokenType::ExpOperator);
        let lparen = || Symbol::Terminal(TokenType::LParenOperator);
        let rparen = || Symbol::Terminal(TokenType::RParenOperator);
        let id = || Symbol::Terminal(TokenType::Id);
        let num = || Symbol::Terminal(TokenType::Number);
        let epsilon = || Symbol::Epsilon;

        // Adicione definições de Ação:
        let action_add = || Symbol::Action(OperatorKind::Sum);
        let action_sub = || Symbol::Action(OperatorKind::Sub);
        let action_mult = || Symbol::Action(OperatorKind::Mult);
        let action_div = || Symbol::Action(OperatorKind::Div);
        let action_exp = || Symbol::Action(OperatorKind::Exp);

        // Produção 1: E → T E'
        table.set_entry("E", TokenType::Id, vec![t(), e_prime()]);
        table.set_entry("E", TokenType::Number, vec![t(), e_prime()]);
        table.set_entry("E", TokenType::LParenOperator, vec![t(), e_prime()]);

        // Produção 2: E' → + T E'
        table.set_entry(
            "E'",
            TokenType::PlusOperator,
            vec![plus(), t(), action_add(), e_prime()],
        );

        // Produção 3: E' → - T E'
        table.set_entry(
            "E'",
            TokenType::MinusOperator,
            vec![minus(), t(), action_sub(), e_prime()],
        );

        // Produção 4: E' → ε
        table.set_entry("E'", TokenType::RParenOperator, vec![epsilon()]);
        table.set_entry("E'", TokenType::Eof, vec![epsilon()]);

        // Produção 5: T → F T'
        table.set_entry("T", TokenType::Id, vec![f(), t_prime()]);
        table.set_entry("T", TokenType::Number, vec![f(), t_prime()]);
        table.set_entry("T", TokenType::LParenOperator, vec![f(), t_prime()]);

        // Produção 6: T' → * F T'
        table.set_entry(
            "T'",
            TokenType::MultOperator,
            vec![mult(), f(), action_mult(), t_prime()],
        );

        // Produção 7: T' → / F T'
        table.set_entry(
            "T'",
            TokenType::DivOperator,
            vec![div(), f(), action_div(), t_prime()],
        );

        // Produção 8: T' → ε
        table.set_entry("T'", TokenType::PlusOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::MinusOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::RParenOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::Eof, vec![epsilon()]);

        // Produção 9: F → U F'
        table.set_entry("F", TokenType::Id, vec![u(), f_prime()]);
        table.set_entry("F", TokenType::Number, vec![u(), f_prime()]);
        table.set_entry("F", TokenType::LParenOperator, vec![u(), f_prime()]);

        // Produção 10: F' → ** U F'
        table.set_entry(
            "F'",
            TokenType::ExpOperator,
            vec![exp(), u(), action_exp(), f_prime()],
        );

        // Produção 11: F' → ε
        table.set_entry("F'", TokenType::PlusOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::MinusOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::MultOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::DivOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::RParenOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::Eof, vec![epsilon()]);

        // Produção 9: U → id
        table.set_entry("U", TokenType::Id, vec![id()]);

        // Produção 10: U → num
        table.set_entry("U", TokenType::Number, vec![num()]);

        // Produção 11: U → ( E )
        table.set_entry(
            "U",
            TokenType::LParenOperator,
            vec![lparen(), e(), rparen()],
        );

        table
    }
}
