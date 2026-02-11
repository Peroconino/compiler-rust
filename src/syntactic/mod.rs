pub use crate::{
    lexer::{KeywordKind, Lexer, OperatorKind, RelopKind, SymbolTable, Token, TokenType, Type},
    syntactic::{
        parse_table::ParseTable,
        symbol::{ActionKind, Symbol},
        tree::AstNode,
    },
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
    pub fn new(
        content: String,
        parse_table: ParseTable,
        symbol_table: &'a mut SymbolTable,
    ) -> Self {
        Parser {
            lexer: Lexer::new(content, symbol_table),
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
                // Ações semânticas
                Symbol::Action(kind) => {
                    match kind {
                        ActionKind::CreateProgram => {
                            self.stack.pop(); // Remove a ação

                            if ast_stack.len() >= 2 {
                                let body = Box::new(ast_stack.pop().unwrap());
                                let kind = ast_stack.pop().unwrap();

                                let node = AstNode::Program {
                                    kind: if let AstNode::TypeWrapper(value) = kind {
                                        value
                                    } else {
                                        return Err("Era esperado um tipo.".into());
                                    },
                                    body,
                                };

                                ast_stack.push(node);
                            } else {
                                return Err(format!(
                                    "Erro semântico: não foi possivel criar nó raiz 'programa' com {:#?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::Math(op_kind) => {
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
                                    "Erro semântico: operandos insuficientes para operação"
                                        .to_string(),
                                );
                            }
                        }
                        ActionKind::CreateBlock => {
                            self.stack.pop(); // Remove ação

                            if ast_stack.len() >= 2 {
                                let stmts_node = ast_stack.pop().unwrap();
                                let stmts = match stmts_node {
                                    AstNode::List(vec) => vec,
                                    _ => {
                                        return Err(format!(
                                            "Erro: Esperava-se uma lista de comandos, recebeu {:?}",
                                            stmts_node
                                        ));
                                    }
                                };

                                let decls_node = ast_stack.pop().unwrap();
                                let decls = match decls_node {
                                    AstNode::List(vec) => vec,
                                    _ => {
                                        return Err(format!(
                                            "Erro: Esperava-se uma lista de declarações, recebeu {:?}",
                                            decls_node
                                        ));
                                    }
                                };

                                let node = AstNode::Block { decls, stmts };
                                ast_stack.push(node);
                            } else {
                                return Err(format!(
                                    "Erro: era esperado ao menos 2 nodos para criar um bloco, temos {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::CreateDecl => {
                            self.stack.pop();

                            if ast_stack.len() >= 2 {
                                let ids_nodes = ast_stack.pop().unwrap();

                                let type_node = ast_stack.pop().unwrap();

                                let var_type = match type_node {
                                    AstNode::TypeWrapper(t) => t,
                                    _ => panic!("Esperava-se um nó tipo, recebeu {:?}", type_node),
                                };

                                let names_vec: Vec<String> = match ids_nodes {
                                    AstNode::List(nodes) => nodes
                                        .into_iter()
                                        .map(|node| match node {
                                            AstNode::Identifier { name } => name,
                                            _ => panic!("Item da lista de declaração não é um ID"),
                                        })
                                        .collect(),
                                    _ => panic!(
                                        "Esperava uma lista de IDs. Recebeu: {:?}",
                                        ids_nodes
                                    ),
                                };

                                let decl_node = AstNode::VarDecl {
                                    kind: var_type,
                                    names: names_vec,
                                };

                                ast_stack.push(decl_node);
                            } else {
                                return Err(format!(
                                    "Era esperado ao menos 2 nodos, recebido: {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::MakeList => {
                            self.stack.pop();

                            ast_stack.push(AstNode::List(vec![]));
                        }
                        ActionKind::AppendList => {
                            self.stack.pop();

                            if ast_stack.len() >= 2 {
                                let mut list_node = ast_stack.pop().unwrap();
                                let item_node = ast_stack.pop().unwrap();

                                if let AstNode::List(ref mut vec) = list_node {
                                    vec.insert(0, item_node);
                                } else {
                                    return Err(format!(
                                        "Esperava-se uma lista no topo da lista. Recebido: {:?}",
                                        list_node
                                    ));
                                }

                                ast_stack.push(list_node);
                            } else {
                                return Err(format!(
                                    "Era esperado ao menos 2 nodos, recebido: {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::Assign => {
                            self.stack.pop();

                            if ast_stack.len() >= 2 {
                                let expr = ast_stack.pop().unwrap();
                                let id = ast_stack.pop().unwrap();

                                let id_name = match id {
                                    AstNode::Identifier { name } => name,
                                    _ => {
                                        return Err(format!(
                                            "Era esperado um identificador. Recebido {:?}",
                                            id
                                        ));
                                    }
                                };

                                ast_stack.push(AstNode::Assignment {
                                    id: id_name,
                                    expr: Box::new(expr),
                                });
                            } else {
                                return Err(format!(
                                    "Era esperado ao menos dois nodos. Recebido: {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::CreateIf => {
                            self.stack.pop();

                            if ast_stack.len() >= 2 {
                                let then_block = Box::new(ast_stack.pop().unwrap());
                                let cond = Box::new(ast_stack.pop().unwrap());

                                ast_stack.push(AstNode::If {
                                    cond,
                                    then_block,
                                    else_block: None,
                                });
                            } else {
                                return Err(format!(
                                    "Era esperado ao menos 2 nodos. Recebido: {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::CreateIfElse => {
                            self.stack.pop();
                            if ast_stack.len() >= 3 {
                                let else_block = Some(Box::new(ast_stack.pop().unwrap()));
                                let then_block = Box::new(ast_stack.pop().unwrap());
                                let cond = Box::new(ast_stack.pop().unwrap());

                                ast_stack.push(AstNode::If {
                                    cond,
                                    then_block,
                                    else_block,
                                });
                            } else {
                                return Err(format!(
                                    "Era esperado ao menos 3 nodos. Recebido {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::CreateWhile => {
                            self.stack.pop();
                            if ast_stack.len() >= 2 {
                                let body = Box::new(ast_stack.pop().unwrap());
                                let cond = Box::new(ast_stack.pop().unwrap());

                                ast_stack.push(AstNode::While { cond, body });
                            } else {
                                return Err(format!(
                                    "Era esperado ao menos 2 nodos. Recebido: {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::CreateDoWhile => {
                            self.stack.pop();
                            if ast_stack.len() >= 2 {
                                let cond = Box::new(ast_stack.pop().unwrap());
                                let body = Box::new(ast_stack.pop().unwrap());

                                ast_stack.push(AstNode::DoWhile { body, cond });
                            } else {
                                return Err(format!(
                                    "Era esperado ao menos 2 nodos. Recebido: {:?}",
                                    ast_stack
                                ));
                            }
                        }
                        ActionKind::CreateFor => {
                            self.stack.pop();
                            if ast_stack.len() >= 5 {
                                let body = Box::new(ast_stack.pop().unwrap());
                                let step = Box::new(ast_stack.pop().unwrap());
                                let end_node = ast_stack.pop().unwrap();
                                let start_node = ast_stack.pop().unwrap();
                                let id_node = ast_stack.pop().unwrap();

                                let end = match end_node {
                                    AstNode::Number { value } => {
                                        value.parse().expect("Esperado um número inteiro.")
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Era esperado um número. Recebido: {:?}",
                                            end_node
                                        ));
                                    }
                                };

                                let start = match start_node {
                                    AstNode::Number { value } => {
                                        value.parse().expect("Esperado um número inteiro.")
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Era esperado um número. Recebido: {:?}",
                                            start_node
                                        ));
                                    }
                                };

                                let id = match id_node {
                                    AstNode::Identifier { name } => name,
                                    _ => {
                                        return Err(format!(
                                            "Era esperado um identificador. Recebido: {:?}",
                                            id_node
                                        ));
                                    }
                                };

                                ast_stack.push(AstNode::For {
                                    id,
                                    start,
                                    end,
                                    step,
                                    body,
                                });
                            }
                        }
                        ActionKind::CreateCond => {
                            self.stack.pop();
                            if ast_stack.len() >= 3 {
                                let right = Box::new(ast_stack.pop().unwrap());
                                let relop_term = ast_stack.pop().unwrap();
                                let left = Box::new(ast_stack.pop().unwrap());

                                let relop = match relop_term {
                                    AstNode::CondWrapper(r) => r,
                                    _ => {
                                        panic!("Era esperado um relop. Recebido: {:?}", relop_term)
                                    }
                                };

                                ast_stack.push(AstNode::BinaryComp { relop, left, right });
                            } else {
                                return Err(
                                    "Era esperado ao menos 3 nodos, para criar uma condição"
                                        .to_string(),
                                );
                            }
                        }
                        ActionKind::CreateUnaryOp => {
                            self.stack.pop();
                            if !ast_stack.is_empty() {
                                let expr = Box::new(ast_stack.pop().unwrap());

                                ast_stack.push(AstNode::UnaryOp { expr });
                            } else {
                                return Err(
                                    "Era esperado ao menos 1 nodo, para criar um operador unário"
                                        .to_string(),
                                );
                            }
                        }
                    }
                }
                Symbol::Epsilon => {
                    self.stack.pop();
                }
            }
        }

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
                // Se for número ou ID ou tipo ou relop, empilha na AST Stack
                match current_token {
                    Token::Id { value: name, .. } => {
                        ast_stack.push(AstNode::Identifier { name: name.clone() });
                    }
                    Token::Number { value, .. } => {
                        ast_stack.push(AstNode::Number {
                            value: value.clone(),
                        });
                    }
                    Token::Char { value, .. } => {
                        ast_stack.push(AstNode::Literal { value: *value });
                    }
                    Token::Keyword {
                        kind: KeywordKind::Char,
                        ..
                    } => {
                        ast_stack.push(AstNode::TypeWrapper(Type::Char));
                    }
                    Token::Keyword {
                        kind: KeywordKind::Float,
                        ..
                    } => {
                        ast_stack.push(AstNode::TypeWrapper(Type::Float));
                    }
                    Token::Keyword {
                        kind: KeywordKind::Int,
                        ..
                    } => {
                        ast_stack.push(AstNode::TypeWrapper(Type::Int));
                    }
                    Token::Keyword {
                        kind: KeywordKind::Void,
                        ..
                    } => {
                        ast_stack.push(AstNode::TypeWrapper(Type::Void));
                    }
                    Token::Relop {
                        kind: RelopKind::GT,
                        ..
                    } => {
                        ast_stack.push(AstNode::CondWrapper(RelopKind::GT));
                    }
                    Token::Relop {
                        kind: RelopKind::LT,
                        ..
                    } => {
                        ast_stack.push(AstNode::CondWrapper(RelopKind::LT));
                    }
                    Token::Relop {
                        kind: RelopKind::LE,
                        ..
                    } => {
                        ast_stack.push(AstNode::CondWrapper(RelopKind::LE));
                    }
                    Token::Relop {
                        kind: RelopKind::EQ,
                        ..
                    } => {
                        ast_stack.push(AstNode::CondWrapper(RelopKind::EQ));
                    }
                    Token::Relop {
                        kind: RelopKind::NE,
                        ..
                    } => {
                        ast_stack.push(AstNode::CondWrapper(RelopKind::NE));
                    }
                    Token::Relop {
                        kind: RelopKind::GE,
                        ..
                    } => {
                        ast_stack.push(AstNode::CondWrapper(RelopKind::GE));
                    }
                    _ => {} //Parênteses e outros tokens não geram nós folhas diretos
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
            Err(format!(
                "Erro de sintaxe. Não esperado {}, com não terminal {:?}",
                current_token, non_terminal
            ))
        }
    }
}
