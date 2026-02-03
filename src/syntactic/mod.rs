use std::collections::HashMap;

pub use crate::{
    Lexer, OperatorKind, Token, TokenType,
    syntactic::{parse_table::ParseTable, symbol::Symbol, tree::AstNode},
};

mod parse_table;
mod symbol;
mod tree;

// Estrutura do Analisador sintático
pub struct Parser<'a> {
    stack: Vec<Symbol>,
    parse_table: ParseTable,
    lexer: Lexer<'a>,
}

// Funções para o analisador sintático
impl<'a> Parser<'a> {
    pub fn new(parse_table: ParseTable, symbol_table: &'a mut HashMap<String, Token>) -> Self {
        Parser {
            lexer: Lexer::new("data.txt", symbol_table),
            stack: Vec::new(),
            parse_table,
        }
    }

    // Função principal do sintático
    pub fn parse(&mut self) -> Result<AstNode, String> {
        // Insere simbolo de parada
        self.stack.push(Symbol::End);
        // Insere simbolo inicial
        self.stack
            .push(Symbol::NonTerminal(self.parse_table.start_symbol.clone()));

        let mut ast_stack: Vec<AstNode> = Vec::new();

        // Obtém primeiro token
        let mut current_token = self.lexer.get_next_token()?;

        while !self.stack.is_empty() {
            let x = self
                .stack
                .last()
                .expect("Era esperado uma transição válida")
                .clone();

            match x {
                Symbol::Terminal(_) | Symbol::End => {
                    self.handle_terminal(&x, &mut current_token, &mut ast_stack)?;
                }
                Symbol::NonTerminal(ref nt) => {
                    self.handle_non_terminal(nt, &current_token)?;
                }
                Symbol::Action(op_kind) => {
                    self.stack.pop(); // Remove a ação da pilha de parsing

                    // Precisamos de pelo menos 2 operandos na pilha AST (Esquerda e Direita)
                    if ast_stack.len() >= 2 {
                        let right = ast_stack.pop().unwrap();
                        let left = ast_stack.pop().unwrap();

                        let new_node = AstNode::BinaryOp {
                            op: op_kind,
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                        ast_stack.push(new_node);
                    } else {
                        return Err(
                            "Erro semântico: operandos insuficientes para operação".to_string()
                        );
                    }
                }
                Symbol::Epsilon => {
                    self.stack.pop();
                }
            }
        }
        // O resultado final deve estar no topo da pilha AST
        if let Some(ast) = ast_stack.pop() {
            Ok(ast)
        } else {
            Err("Falha ao construir AST".to_string())
        }
    }

    fn handle_terminal(
        &mut self,
        symbol: &Symbol,
        current_token: &mut Token,
        ast_stack: &mut Vec<AstNode>,
    ) -> Result<(), String> {
        if symbol.is_end() {
            if *current_token == Token::Eof {
                self.stack.pop();
                return Ok(());
            } else {
                return Err("Esperado fim de arquivo".into());
            }
        }

        if let Some(terminal_type) = symbol.as_terminal() {
            // Verifica compatibilidade entre Token esperado e atual
            if terminal_type == current_token.clone().into() {
                // Se for número ou ID, empilha na AST Stack
                match current_token {
                    Token::Id { value: name, .. } => {
                        ast_stack.push(AstNode::Identifier { name: name.clone() });
                    }
                    Token::Number { value, .. } => {
                        ast_stack.push(AstNode::Number {
                            value: value.clone(),
                        });
                    }
                    _ => {} // Parênteses e outros tokens não geram nós folhas diretos
                };

                self.stack.pop();
                *current_token = self.lexer.get_next_token()?;
                Ok(())
            } else {
                Err(format!(
                    "Erro de sintaxe. Esperado {:?}, encontrado {}",
                    terminal_type, current_token
                ))
            }
        } else {
            Ok(())
        }
    }

    fn handle_non_terminal(
        &mut self,
        non_terminal: &str,
        current_token: &Token,
    ) -> Result<(), String> {
        if let Some(production) = self
            .parse_table
            .get_entry(non_terminal, &current_token.clone().into())
        {
            self.stack.pop();

            // Empilha símbolos na ordem inversa
            for symbol in production.iter().rev() {
                if !symbol.is_epsilon() {
                    self.stack.push(symbol.clone());
                }
            }

            Ok(())
        } else {
            Err(format!("Erro de sintaxe. Não esperado {}", current_token))
        }
    }
}
