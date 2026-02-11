pub mod lexer;
pub mod syntactic;

pub use lexer::*;
pub use syntactic::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn create_instance<'a>(
        contents: String,
        start_symbol: &str,
        symbol_table: &'a mut SymbolTable,
    ) -> Parser<'a> {
        let parse_table = ParseTable::create_parse_table(start_symbol);
        Parser::new(contents, parse_table, symbol_table)
    }

    #[test]
    fn test_unary_ast_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance("-6 ".into(), "E", &mut symbol_table);

        let result = parser.parse();

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        let expected_ast = AstNode::UnaryOp {
            expr: Box::new(AstNode::Number { value: "6".into() }),
        };

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A árvore gerada não corresponde a esperada"
        );

        let mut parser = create_instance("-(x * 2) ".into(), "E", &mut symbol_table);

        let result = parser.parse();

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        let expected_ast = AstNode::UnaryOp {
            expr: Box::new(AstNode::BinaryOp {
                op: OperatorKind::Mult,
                left: Box::new(AstNode::Identifier { name: "x".into() }),
                right: Box::new(AstNode::Number { value: "2".into() }),
            }),
        };

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A árvore gerada não corresponde a esperada"
        );
    }

    #[test]
    fn test_assigment_ast_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance("x := 5 - 6; ".into(), "cmd_atrib", &mut symbol_table);

        let result = parser.parse();

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        let expected_ast = AstNode::Assignment {
            id: "x".to_string(),
            expr: Box::new(AstNode::BinaryOp {
                op: OperatorKind::Sub, // A raiz da expressão é a subtração
                left: Box::new(AstNode::Number {
                    value: "5".to_string(),
                }),
                right: Box::new(AstNode::Number {
                    value: "6".to_string(),
                }),
            }),
        };

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A árvore gerada não corresponde a esperada"
        );
    }

    #[test]
    fn test_if_ast_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance(
            "if(x>y)then[y := y * 2;] ".into(),
            "cmd_if",
            &mut symbol_table,
        );

        let result = parser.parse();

        let expected_ast = AstNode::If {
            cond: Box::new(AstNode::BinaryComp {
                relop: RelopKind::GT,
                left: Box::new(AstNode::Identifier {
                    name: "x".to_string(),
                }),
                right: Box::new(AstNode::Identifier {
                    name: "y".to_string(),
                }),
            }),
            then_block: Box::new(AstNode::Block {
                decls: vec![],
                stmts: vec![AstNode::Assignment {
                    id: "y".to_string(),
                    expr: Box::new(AstNode::BinaryOp {
                        op: OperatorKind::Mult,
                        left: Box::new(AstNode::Identifier {
                            name: "y".to_string(),
                        }),
                        right: Box::new(AstNode::Number {
                            value: "2".to_string(),
                        }),
                    }),
                }],
            }),
            else_block: None,
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_if_else_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance(
            "if(x>y)then[y := y * 2;]else[x := 2**4;] ".into(),
            "cmd_if",
            &mut symbol_table,
        );

        let result = parser.parse();

        let expected_ast = AstNode::If {
            cond: Box::new(AstNode::BinaryComp {
                relop: RelopKind::GT,
                left: Box::new(AstNode::Identifier {
                    name: "x".to_string(),
                }),
                right: Box::new(AstNode::Identifier {
                    name: "y".to_string(),
                }),
            }),
            then_block: Box::new(AstNode::Block {
                decls: vec![],
                stmts: vec![AstNode::Assignment {
                    id: "y".to_string(),
                    expr: Box::new(AstNode::BinaryOp {
                        op: OperatorKind::Mult,
                        left: Box::new(AstNode::Identifier {
                            name: "y".to_string(),
                        }),
                        right: Box::new(AstNode::Number {
                            value: "2".to_string(),
                        }),
                    }),
                }],
            }),
            else_block: Some(Box::new(AstNode::Block {
                decls: vec![],
                stmts: vec![AstNode::Assignment {
                    id: "x".to_string(),
                    expr: Box::new(AstNode::BinaryOp {
                        op: OperatorKind::Exp,
                        left: Box::new(AstNode::Number {
                            value: "2".to_string(),
                        }),
                        right: Box::new(AstNode::Number {
                            value: "4".to_string(),
                        }),
                    }),
                }],
            })),
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_cond_ast_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance("x > y ".into(), "cond", &mut symbol_table);

        let result = parser.parse();

        let expected_tree = AstNode::BinaryComp {
            relop: RelopKind::GT,
            left: Box::new(AstNode::Identifier {
                name: "x".to_string(),
            }),
            right: Box::new(AstNode::Identifier {
                name: "y".to_string(),
            }),
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());
        assert_eq!(
            result.unwrap(),
            expected_tree,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_block_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser =
            create_instance("[int a,b; y := y * 2;] ".into(), "bloco", &mut symbol_table);

        let result = parser.parse();

        let expected_ast = AstNode::Block {
            decls: vec![AstNode::VarDecl {
                kind: Type::Int,
                names: vec!["a".to_string(), "b".to_string()],
            }],
            stmts: vec![AstNode::Assignment {
                id: "y".to_string(),
                expr: Box::new(AstNode::BinaryOp {
                    op: OperatorKind::Mult,
                    left: Box::new(AstNode::Identifier {
                        name: "y".to_string(),
                    }),
                    right: Box::new(AstNode::Number {
                        value: "2".to_string(),
                    }),
                }),
            }],
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_if_elsif_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance(
            "if(x>y)then[y := y * 2;]elsif(c=='a')then[x := 2**4;]else[int a, b;] ".into(),
            "cmd_if",
            &mut symbol_table,
        );

        let result = parser.parse();

        let expected_ast = AstNode::If {
            cond: Box::new(AstNode::BinaryComp {
                relop: RelopKind::GT,
                left: Box::new(AstNode::Identifier {
                    name: "x".to_string(),
                }),
                right: Box::new(AstNode::Identifier {
                    name: "y".to_string(),
                }),
            }),
            then_block: Box::new(AstNode::Block {
                decls: vec![],
                stmts: vec![AstNode::Assignment {
                    id: "y".to_string(),
                    expr: Box::new(AstNode::BinaryOp {
                        op: OperatorKind::Mult,
                        left: Box::new(AstNode::Identifier {
                            name: "y".to_string(),
                        }),
                        right: Box::new(AstNode::Number {
                            value: "2".to_string(),
                        }),
                    }),
                }],
            }),
            else_block: Some(Box::new(AstNode::If {
                cond: Box::new(AstNode::BinaryComp {
                    relop: RelopKind::EQ,
                    left: Box::new(AstNode::Identifier {
                        name: "c".to_string(),
                    }),
                    right: Box::new(AstNode::Literal { value: 'a' }),
                }),
                then_block: Box::new(AstNode::Block {
                    decls: vec![],
                    stmts: vec![AstNode::Assignment {
                        id: "x".to_string(),
                        expr: Box::new(AstNode::BinaryOp {
                            op: OperatorKind::Exp,
                            left: Box::new(AstNode::Number {
                                value: "2".to_string(),
                            }),
                            right: Box::new(AstNode::Number {
                                value: "4".to_string(),
                            }),
                        }),
                    }],
                }),
                else_block: Some(Box::new(AstNode::Block {
                    decls: vec![AstNode::VarDecl {
                        kind: Type::Int,
                        names: vec!["a".to_string(), "b".to_string()],
                    }],
                    stmts: vec![],
                })),
            })),
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_while_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance(
            "while(x>y)do y := y * 2; ".into(),
            "cmd_while",
            &mut symbol_table,
        );

        let result = parser.parse();

        let expected_ast = AstNode::While {
            cond: Box::new(AstNode::BinaryComp {
                relop: RelopKind::GT,
                left: Box::new(AstNode::Identifier {
                    name: "x".to_string(),
                }),
                right: Box::new(AstNode::Identifier {
                    name: "y".to_string(),
                }),
            }),
            body: Box::new(AstNode::Assignment {
                id: "y".to_string(),
                expr: Box::new(AstNode::BinaryOp {
                    op: OperatorKind::Mult,
                    left: Box::new(AstNode::Identifier {
                        name: "y".to_string(),
                    }),
                    right: Box::new(AstNode::Number {
                        value: "2".to_string(),
                    }),
                }),
            }),
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_do_while_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance(
            "do y := y * 2; while(x>y); ".into(),
            "cmd_do",
            &mut symbol_table,
        );

        let result = parser.parse();

        let expected_ast = AstNode::DoWhile {
            body: Box::new(AstNode::Assignment {
                id: "y".to_string(),
                expr: Box::new(AstNode::BinaryOp {
                    op: OperatorKind::Mult,
                    left: Box::new(AstNode::Identifier {
                        name: "y".to_string(),
                    }),
                    right: Box::new(AstNode::Number {
                        value: "2".to_string(),
                    }),
                }),
            }),
            cond: Box::new(AstNode::BinaryComp {
                relop: RelopKind::GT,
                left: Box::new(AstNode::Identifier {
                    name: "x".to_string(),
                }),
                right: Box::new(AstNode::Identifier {
                    name: "y".to_string(),
                }),
            }),
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_for_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance(
            "for(x;0;10;x+1) y := y / 2; ".into(),
            "cmd_for",
            &mut symbol_table,
        );

        let result = parser.parse();

        let expected_ast = AstNode::For {
            id: "x".to_string(),
            start: "0".parse().unwrap(),
            end: "10".parse().unwrap(),
            step: Box::new(AstNode::BinaryOp {
                op: OperatorKind::Sum,
                left: Box::new(AstNode::Identifier {
                    name: "x".to_string(),
                }),
                right: Box::new(AstNode::Number {
                    value: "1".to_string(),
                }),
            }),
            body: Box::new(AstNode::Assignment {
                id: "y".to_string(),
                expr: Box::new(AstNode::BinaryOp {
                    op: OperatorKind::Div,
                    left: Box::new(AstNode::Identifier {
                        name: "y".to_string(),
                    }),
                    right: Box::new(AstNode::Number {
                        value: "2".to_string(),
                    }),
                }),
            }),
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }

    #[test]
    fn test_program_tree() {
        let mut symbol_table = HashMap::new();
        let mut parser = create_instance(
            "int main() [
                char c;
                int x;
                float y;

                c := 'a';
                x := 67;
                y := 54.90E-22;

                if(x>y)then[
                    y := y * 2;
                ]elsif(c=='a')then[
                    x := x + 2;
                ]else[
                    c := 'b';
                ]

                {%comentario%}
                {%
                    comentario
                    de
                    multiplas
                    linhas
                %}

                do [
                    x := x + 1;
                ]while (x < 100);

            ] "
            .into(),
            "inicio",
            &mut symbol_table,
        );

        let result = parser.parse();

        let expected_ast = AstNode::Program {
            kind: Type::Int,
            body: Box::new(AstNode::Block {
                decls: vec![
                    AstNode::VarDecl {
                        kind: Type::Char,
                        names: vec!["c".to_string()],
                    },
                    AstNode::VarDecl {
                        kind: Type::Int,
                        names: vec!["x".to_string()],
                    },
                    AstNode::VarDecl {
                        kind: Type::Float,
                        names: vec!["y".to_string()],
                    },
                ],
                stmts: vec![
                    AstNode::Assignment {
                        id: "c".to_string(),
                        expr: Box::new(AstNode::Literal { value: 'a' }),
                    },
                    AstNode::Assignment {
                        id: "x".to_string(),
                        expr: Box::new(AstNode::Number {
                            value: "67".to_string(),
                        }),
                    },
                    AstNode::Assignment {
                        id: "y".to_string(),
                        expr: Box::new(AstNode::Number {
                            value: "54.90E-22".to_string(),
                        }),
                    },
                    AstNode::If {
                        cond: Box::new(AstNode::BinaryComp {
                            relop: RelopKind::GT,
                            left: Box::new(AstNode::Identifier {
                                name: "x".to_string(),
                            }),
                            right: Box::new(AstNode::Identifier {
                                name: "y".to_string(),
                            }),
                        }),
                        then_block: Box::new(AstNode::Block {
                            decls: vec![],
                            stmts: vec![AstNode::Assignment {
                                id: "y".to_string(),
                                expr: Box::new(AstNode::BinaryOp {
                                    op: OperatorKind::Mult,
                                    left: Box::new(AstNode::Identifier {
                                        name: "y".to_string(),
                                    }),
                                    right: Box::new(AstNode::Number {
                                        value: "2".to_string(),
                                    }),
                                }),
                            }],
                        }),
                        else_block: Some(Box::new(AstNode::If {
                            cond: Box::new(AstNode::BinaryComp {
                                relop: RelopKind::EQ,
                                left: Box::new(AstNode::Identifier {
                                    name: "c".to_string(),
                                }),
                                right: Box::new(AstNode::Literal { value: 'a' }),
                            }),
                            then_block: Box::new(AstNode::Block {
                                decls: vec![],
                                stmts: vec![AstNode::Assignment {
                                    id: "x".to_string(),
                                    expr: Box::new(AstNode::BinaryOp {
                                        op: OperatorKind::Sum,
                                        left: Box::new(AstNode::Identifier {
                                            name: "x".to_string(),
                                        }),
                                        right: Box::new(AstNode::Number {
                                            value: "2".to_string(),
                                        }),
                                    }),
                                }],
                            }),
                            else_block: Some(Box::new(AstNode::Block {
                                decls: vec![],
                                stmts: vec![AstNode::Assignment {
                                    id: "c".to_string(),
                                    expr: Box::new(AstNode::Literal { value: 'b' }),
                                }],
                            })),
                        })),
                    },
                    AstNode::DoWhile {
                        body: Box::new(AstNode::Block {
                            decls: vec![],
                            stmts: vec![AstNode::Assignment {
                                id: "x".to_string(),
                                expr: Box::new(AstNode::BinaryOp {
                                    op: OperatorKind::Sum,
                                    left: Box::new(AstNode::Identifier {
                                        name: "x".to_string(),
                                    }),
                                    right: Box::new(AstNode::Number {
                                        value: "1".to_string(),
                                    }),
                                }),
                            }],
                        }),
                        cond: Box::new(AstNode::BinaryComp {
                            relop: RelopKind::LT,
                            left: Box::new(AstNode::Identifier {
                                name: "x".to_string(),
                            }),
                            right: Box::new(AstNode::Number {
                                value: "100".to_string(),
                            }),
                        }),
                    },
                ],
            }),
        };

        assert!(result.is_ok(), "O parser retornou erro: {:?}", result.err());

        assert_eq!(
            result.unwrap(),
            expected_ast,
            "A arvore gerada não corresponde"
        );
    }
}
