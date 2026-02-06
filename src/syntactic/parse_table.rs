use std::{collections::HashMap, vec};

use crate::{ActionKind, OperatorKind, TokenType, syntactic::symbol::Symbol};

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

    pub fn create_parse_table(start_symbol: &str) -> ParseTable {
        let mut table = ParseTable::new(start_symbol);
        table = ParseTable::create_expression_parse_table(table);
        table = ParseTable::create_programa_table(table);
        table = ParseTable::create_tipo_table(table);
        table = ParseTable::create_bloco_table(table);
        table = ParseTable::create_decls_table(table);
        table = ParseTable::create_decl_table(table);
        table = ParseTable::create_lista_ids_table(table);
        table = ParseTable::create_lista_ids_prime_table(table);
        table = ParseTable::create_comandos_table(table);
        table = ParseTable::create_comando_table(table);
        table = ParseTable::create_cmd_atrib_table(table);
        table = ParseTable::create_cmd_if_table(table);
        table = ParseTable::create_cond_table(table);
        table = ParseTable::create_op_rel_table(table);
        table = ParseTable::create_cmd_ou_bloco_table(table);
        table = ParseTable::create_cmd_if_prime_table(table);
        table = ParseTable::create_cmd_while_table(table);
        table = ParseTable::create_cmd_do_table(table);
        table = ParseTable::create_cmd_for_table(table);
        table
    }

    /**
     <inicio> -> <tipo> main ( ) <bloco>
    */
    fn create_programa_table(mut table: ParseTable) -> ParseTable {
        let tipo = || Symbol::NonTerminal("tipo".to_string());
        let main = || Symbol::Terminal(TokenType::MainKeyword);
        let lparen = || Symbol::Terminal(TokenType::LParenOperator);
        let rparen = || Symbol::Terminal(TokenType::RParenOperator);
        let bloco = || Symbol::NonTerminal("bloco".to_string());
        let create_program_action = || Symbol::Action(ActionKind::CreateProgram);

        table.set_entry(
            "inicio",
            TokenType::CharKeyword,
            vec![
                tipo(),
                main(),
                lparen(),
                rparen(),
                bloco(),
                create_program_action(),
            ],
        );
        table.set_entry(
            "inicio",
            TokenType::IntKeyword,
            vec![
                tipo(),
                main(),
                lparen(),
                rparen(),
                bloco(),
                create_program_action(),
            ],
        );
        table.set_entry(
            "inicio",
            TokenType::FloatKeyword,
            vec![
                tipo(),
                main(),
                lparen(),
                rparen(),
                bloco(),
                create_program_action(),
            ],
        );
        table.set_entry(
            "inicio",
            TokenType::VoidKeyword,
            vec![
                tipo(),
                main(),
                lparen(),
                rparen(),
                bloco(),
                create_program_action(),
            ],
        );

        table
    }

    /**
     <tipo> -> float | int | char | void
    */
    fn create_tipo_table(mut table: ParseTable) -> ParseTable {
        let float = || Symbol::Terminal(TokenType::FloatKeyword);
        let int = || Symbol::Terminal(TokenType::IntKeyword);
        let char = || Symbol::Terminal(TokenType::CharKeyword);
        let void = || Symbol::Terminal(TokenType::VoidKeyword);

        // Não sei ainda se tem Action aqui, acho que não
        table.set_entry("tipo", TokenType::FloatKeyword, vec![float()]);
        table.set_entry("tipo", TokenType::IntKeyword, vec![int()]);
        table.set_entry("tipo", TokenType::CharKeyword, vec![char()]);
        table.set_entry("tipo", TokenType::VoidKeyword, vec![void()]);

        table
    }

    /**
     <bloco> -> [ <decls> <comandos> ]
    */
    fn create_bloco_table(mut table: ParseTable) -> ParseTable {
        let lbracket = Symbol::Terminal(TokenType::BeginBlockPunctuation);
        let decls = Symbol::NonTerminal("decls".to_string());
        let comandos = Symbol::NonTerminal("comandos".to_string());
        let rbracket = Symbol::Terminal(TokenType::EndBlockPunctuation);
        let create_block = Symbol::Action(ActionKind::CreateBlock);

        table.set_entry(
            "bloco",
            TokenType::BeginBlockPunctuation,
            vec![lbracket, decls, comandos, rbracket, create_block],
        );

        table
    }

    /**
     <decls> -> <decl> <decls> | ε
    */
    fn create_decls_table(mut table: ParseTable) -> ParseTable {
        let decl = || Symbol::NonTerminal("decl".to_string());
        let decls = || Symbol::NonTerminal("decls".to_string());
        let epsilon = || Symbol::Epsilon;
        let action_make_list = || Symbol::Action(ActionKind::MakeList);
        let action_append_list = || Symbol::Action(ActionKind::AppendList);

        // <decls> -> <decl> <decls>
        table.set_entry(
            "decls",
            TokenType::CharKeyword,
            vec![decl(), decls(), action_append_list()],
        );
        table.set_entry(
            "decls",
            TokenType::IntKeyword,
            vec![decl(), decls(), action_append_list()],
        );
        table.set_entry(
            "decls",
            TokenType::FloatKeyword,
            vec![decl(), decls(), action_append_list()],
        );
        table.set_entry(
            "decls",
            TokenType::VoidKeyword,
            vec![decl(), decls(), action_append_list()],
        );

        // <delcs> -> ε
        table.set_entry("decls", TokenType::Id, vec![epsilon(), action_make_list()]);
        table.set_entry(
            "decls",
            TokenType::IfKeyword,
            vec![epsilon(), action_make_list()],
        );
        table.set_entry(
            "decls",
            TokenType::WhileKeyword,
            vec![epsilon(), action_make_list()],
        );
        table.set_entry(
            "decls",
            TokenType::DoKeyword,
            vec![epsilon(), action_make_list()],
        );
        table.set_entry(
            "decls",
            TokenType::ForKeyword,
            vec![epsilon(), action_make_list()],
        );
        table.set_entry(
            "decls",
            TokenType::EndBlockPunctuation,
            vec![epsilon(), action_make_list()],
        );

        table
    }

    /**
     <decl> -> <tipo> <lista_ids> ;
    */
    fn create_decl_table(mut table: ParseTable) -> ParseTable {
        let tipo = || Symbol::NonTerminal("tipo".to_string());
        let lista_ids = || Symbol::NonTerminal("lista_ids".to_string());
        let end_exp = || Symbol::Terminal(TokenType::SemiColonPunctuation);
        let action_create_decl = || Symbol::Action(ActionKind::CreateDecl);

        table.set_entry(
            "decl",
            TokenType::CharKeyword,
            vec![tipo(), lista_ids(), end_exp(), action_create_decl()],
        );
        table.set_entry(
            "decl",
            TokenType::IntKeyword,
            vec![tipo(), lista_ids(), end_exp(), action_create_decl()],
        );
        table.set_entry(
            "decl",
            TokenType::FloatKeyword,
            vec![tipo(), lista_ids(), end_exp(), action_create_decl()],
        );
        table.set_entry(
            "decl",
            TokenType::VoidKeyword,
            vec![tipo(), lista_ids(), end_exp(), action_create_decl()],
        );

        table
    }

    /**
     <lista_ids> -> id <lista_ids'>
    */
    fn create_lista_ids_table(mut table: ParseTable) -> ParseTable {
        let id = Symbol::Terminal(TokenType::Id);
        let lista_ids_prime = Symbol::NonTerminal("lista_ids'".to_string());
        let action_append_list = Symbol::Action(ActionKind::AppendList);

        table.set_entry(
            "lista_ids",
            TokenType::Id,
            vec![id, lista_ids_prime, action_append_list],
        );

        table
    }

    /**
     <lista_ids'> -> , id <lista_ids'> | ε
    */
    fn create_lista_ids_prime_table(mut table: ParseTable) -> ParseTable {
        let comma = Symbol::Terminal(TokenType::CommaPunctuation);
        let id = Symbol::Terminal(TokenType::Id);
        let lista_ids_prime = Symbol::NonTerminal("lista_ids'".to_string());
        let epsilon = Symbol::Epsilon;
        let action_append_list = Symbol::Action(ActionKind::AppendList);
        let action_make_list = Symbol::Action(ActionKind::MakeList);

        table.set_entry(
            "lista_ids'",
            TokenType::CommaPunctuation,
            vec![comma, id, lista_ids_prime, action_append_list],
        );

        table.set_entry(
            "lista_ids'",
            TokenType::SemiColonPunctuation,
            vec![epsilon, action_make_list],
        );

        table
    }

    /**
     <comandos> -> <comando> <comandos> | ε
    */
    fn create_comandos_table(mut table: ParseTable) -> ParseTable {
        let comando = || Symbol::NonTerminal("comando".to_string());
        let comandos = || Symbol::NonTerminal("comandos".to_string());
        let epsilon = || Symbol::Epsilon;
        let action_make_list = || Symbol::Action(ActionKind::MakeList);
        let action_append_list = || Symbol::Action(ActionKind::AppendList);

        table.set_entry(
            "comandos",
            TokenType::Id,
            vec![comando(), comandos(), action_append_list()],
        );
        table.set_entry(
            "comandos",
            TokenType::IfKeyword,
            vec![comando(), comandos(), action_append_list()],
        );
        table.set_entry(
            "comandos",
            TokenType::WhileKeyword,
            vec![comando(), comandos(), action_append_list()],
        );
        table.set_entry(
            "comandos",
            TokenType::DoKeyword,
            vec![comando(), comandos(), action_append_list()],
        );
        table.set_entry(
            "comandos",
            TokenType::ForKeyword,
            vec![comando(), comandos(), action_append_list()],
        );
        table.set_entry(
            "comandos",
            TokenType::BeginBlockPunctuation,
            vec![comando(), comandos(), action_append_list()],
        );

        table.set_entry(
            "comandos",
            TokenType::EndBlockPunctuation,
            vec![epsilon(), action_make_list()],
        );

        table
    }

    /**
     <comando> -> <cmd_atrib> | <cmd_if> | <cmd_while> | <cmd_do> | <cmd_for> | <bloco>
    */
    fn create_comando_table(mut table: ParseTable) -> ParseTable {
        let cmd_atrib = || Symbol::NonTerminal("cmd_atrib".to_string());
        let cmd_if = || Symbol::NonTerminal("cmd_if".to_string());
        let cmd_while = || Symbol::NonTerminal("cmd_while".to_string());
        let cmd_do = || Symbol::NonTerminal("cmd_do".to_string());
        let cmd_for = || Symbol::NonTerminal("cmd_for".to_string());
        let bloco = || Symbol::NonTerminal("bloco".to_string());

        table.set_entry("comando", TokenType::Id, vec![cmd_atrib()]);

        table.set_entry("comando", TokenType::IfKeyword, vec![cmd_if()]);

        table.set_entry("comando", TokenType::WhileKeyword, vec![cmd_while()]);

        table.set_entry("comando", TokenType::DoKeyword, vec![cmd_do()]);

        table.set_entry("comando", TokenType::ForKeyword, vec![cmd_for()]);

        table.set_entry("comando", TokenType::BeginBlockPunctuation, vec![bloco()]);

        table
    }

    /**
     <cmd_atrib> -> id := <E> ;
    */
    fn create_cmd_atrib_table(mut table: ParseTable) -> ParseTable {
        let id = Symbol::Terminal(TokenType::Id);
        let atrib = Symbol::Terminal(TokenType::AssignPunctuation);
        let expr = Symbol::NonTerminal("E".to_string());
        let end_expr = Symbol::Terminal(TokenType::SemiColonPunctuation);
        let action_assign = Symbol::Action(ActionKind::Assign);

        table.set_entry(
            "cmd_atrib",
            TokenType::Id,
            vec![id, atrib, expr, end_expr, action_assign],
        );

        table
    }

    /**
     E  → T E'
     E' → + T E' | - T E' | ε
     T  → F T'
     T' → * F T' | / F T' | ε
     F  → U F'
     F' → ** U F' | ε
     U  → id | num | ( E )
    */
    fn create_expression_parse_table(mut table: ParseTable) -> ParseTable {
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
        let action_add = || Symbol::Action(ActionKind::Math(OperatorKind::Sum));
        let action_sub = || Symbol::Action(ActionKind::Math(OperatorKind::Sub));
        let action_mult = || Symbol::Action(ActionKind::Math(OperatorKind::Mult));
        let action_div = || Symbol::Action(ActionKind::Math(OperatorKind::Div));
        let action_exp = || Symbol::Action(ActionKind::Math(OperatorKind::Exp));

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
        table.set_entry("E'", TokenType::SemiColonPunctuation, vec![epsilon()]);
        table.set_entry("E'", TokenType::GTOperator, vec![epsilon()]);
        table.set_entry("E'", TokenType::LTOperator, vec![epsilon()]);
        table.set_entry("E'", TokenType::GEOperator, vec![epsilon()]);
        table.set_entry("E'", TokenType::LEOperator, vec![epsilon()]);
        table.set_entry("E'", TokenType::NEOperator, vec![epsilon()]);
        table.set_entry("E'", TokenType::EQOperator, vec![epsilon()]);
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
        table.set_entry("T'", TokenType::SemiColonPunctuation, vec![epsilon()]);
        table.set_entry("T'", TokenType::GTOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::LTOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::GEOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::LEOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::NEOperator, vec![epsilon()]);
        table.set_entry("T'", TokenType::EQOperator, vec![epsilon()]);
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
        table.set_entry("F'", TokenType::SemiColonPunctuation, vec![epsilon()]);
        table.set_entry("F'", TokenType::GTOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::LTOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::GEOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::LEOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::NEOperator, vec![epsilon()]);
        table.set_entry("F'", TokenType::EQOperator, vec![epsilon()]);
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

    /**
     <cmd_if> -> if ( <cond> ) then <cmd_ou_bloco> <cmd_if'>
    */
    fn create_cmd_if_table(mut table: ParseTable) -> ParseTable {
        let se = || Symbol::Terminal(TokenType::IfKeyword);
        let lparen = || Symbol::Terminal(TokenType::LParenOperator);
        let cond = || Symbol::NonTerminal("cond".to_string());
        let rparen = || Symbol::Terminal(TokenType::RParenOperator);
        let then = || Symbol::Terminal(TokenType::ThenKeyword);
        let cmd_ou_bloco = || Symbol::NonTerminal("cmd_ou_bloco".to_string());
        let cmd_if_prime = || Symbol::NonTerminal("cmd_if'".to_string());

        table.set_entry(
            "cmd_if",
            TokenType::IfKeyword,
            vec![
                se(),
                lparen(),
                cond(),
                rparen(),
                then(),
                cmd_ou_bloco(),
                cmd_if_prime(),
            ],
        );

        table
    }

    /**
     <cond> -> <E> <op_rel> <E>
    */
    fn create_cond_table(mut table: ParseTable) -> ParseTable {
        let expr = || Symbol::NonTerminal("E".to_string());
        let op_rel = || Symbol::NonTerminal("op_rel".to_string());
        let action_create_cond = || Symbol::Action(ActionKind::CreateCond);

        table.set_entry(
            "cond",
            TokenType::Id,
            vec![expr(), op_rel(), expr(), action_create_cond()],
        );
        table.set_entry(
            "cond",
            TokenType::Number,
            vec![expr(), op_rel(), expr(), action_create_cond()],
        );
        table.set_entry(
            "cond",
            TokenType::LParenOperator,
            vec![expr(), op_rel(), expr(), action_create_cond()],
        );

        table
    }

    /**
     <op_rel> -> == | != | < | > | <= | >=
    */
    fn create_op_rel_table(mut table: ParseTable) -> ParseTable {
        let eq = Symbol::Terminal(TokenType::EQOperator);
        let ne = Symbol::Terminal(TokenType::NEOperator);
        let le = Symbol::Terminal(TokenType::LEOperator);
        let ge = Symbol::Terminal(TokenType::GEOperator);
        let lt = Symbol::Terminal(TokenType::LTOperator);
        let gt = Symbol::Terminal(TokenType::GTOperator);

        table.set_entry("op_rel", TokenType::EQOperator, vec![eq]);
        table.set_entry("op_rel", TokenType::NEOperator, vec![ne]);
        table.set_entry("op_rel", TokenType::LEOperator, vec![le]);
        table.set_entry("op_rel", TokenType::GEOperator, vec![ge]);
        table.set_entry("op_rel", TokenType::LTOperator, vec![lt]);
        table.set_entry("op_rel", TokenType::GTOperator, vec![gt]);

        table
    }

    /**
     <cmd_ou_bloco> -> <comando> | <bloco>
    */
    fn create_cmd_ou_bloco_table(mut table: ParseTable) -> ParseTable {
        let comando = || Symbol::NonTerminal("comando".to_string());
        let bloco = Symbol::NonTerminal("bloco".to_string());

        table.set_entry("cmd_ou_bloco", TokenType::Id, vec![comando()]);
        table.set_entry("cmd_ou_bloco", TokenType::IfKeyword, vec![comando()]);
        table.set_entry("cmd_ou_bloco", TokenType::WhileKeyword, vec![comando()]);
        table.set_entry("cmd_ou_bloco", TokenType::DoKeyword, vec![comando()]);
        table.set_entry("cmd_ou_bloco", TokenType::ForKeyword, vec![comando()]);
        table.set_entry(
            "cmd_ou_bloco",
            TokenType::BeginBlockPunctuation,
            vec![comando()],
        );

        table.set_entry(
            "cmd_ou_bloco",
            TokenType::BeginBlockPunctuation,
            vec![bloco],
        );

        table
    }

    /**
     <cmd_if'> -> elsif ( <cond> ) then <cmd_ou_bloco> <cmd_if'>
     <cmd_if'> -> else <cmd_ou_bloco>
     <cmd_if'> -> ε
    */
    fn create_cmd_if_prime_table(mut table: ParseTable) -> ParseTable {
        let elsif = Symbol::Terminal(TokenType::ElsifKeyword);
        let lparen = Symbol::Terminal(TokenType::LParenOperator);
        let cond = Symbol::NonTerminal("cond".to_string());
        let rparen = Symbol::Terminal(TokenType::RParenOperator);
        let then = Symbol::Terminal(TokenType::ThenKeyword);
        let cmd_ou_bloco = || Symbol::NonTerminal("cmd_ou_bloco".to_string());
        let cmd_if_prime = Symbol::NonTerminal("cmd_if'".to_string());
        let senao = Symbol::Terminal(TokenType::ElseKeyword);
        let epsilon = || Symbol::Epsilon;
        let action_create_if = || Symbol::Action(ActionKind::CreateIf);
        let action_create_if_else = || Symbol::Action(ActionKind::CreateIfElse);

        table.set_entry(
            "cmd_if'",
            TokenType::ElsifKeyword,
            vec![
                elsif,
                lparen,
                cond,
                rparen,
                then,
                cmd_ou_bloco(),
                cmd_if_prime,
                action_create_if_else(),
            ],
        );

        table.set_entry(
            "cmd_if'",
            TokenType::ElseKeyword,
            vec![senao, cmd_ou_bloco(), action_create_if_else()],
        );

        table.set_entry(
            "cmd_if'",
            TokenType::Id,
            vec![epsilon(), action_create_if()],
        );
        table.set_entry(
            "cmd_if'",
            TokenType::IfKeyword,
            vec![epsilon(), action_create_if()],
        );
        table.set_entry(
            "cmd_if'",
            TokenType::WhileKeyword,
            vec![epsilon(), action_create_if()],
        );
        table.set_entry(
            "cmd_if'",
            TokenType::DoKeyword,
            vec![epsilon(), action_create_if()],
        );
        table.set_entry(
            "cmd_if'",
            TokenType::ForKeyword,
            vec![epsilon(), action_create_if()],
        );
        table.set_entry(
            "cmd_if'",
            TokenType::BeginBlockPunctuation,
            vec![epsilon(), action_create_if()],
        );
        table.set_entry(
            "cmd_if'",
            TokenType::Eof,
            vec![epsilon(), action_create_if()],
        );

        table
    }

    /**
     <cmd_while> -> while ( <cond> ) do <cmd_ou_bloco>
    */
    fn create_cmd_while_table(mut table: ParseTable) -> ParseTable {
        let enquanto = Symbol::Terminal(TokenType::WhileKeyword);
        let lparen = Symbol::Terminal(TokenType::LParenOperator);
        let cond = Symbol::NonTerminal("cond".to_string());
        let rparen = Symbol::Terminal(TokenType::RParenOperator);
        let faca = Symbol::Terminal(TokenType::DoKeyword);
        let cmd_ou_bloco = Symbol::NonTerminal("cmd_ou_bloco".to_string());
        let action_create_while = Symbol::Action(ActionKind::CreateWhile);

        table.set_entry(
            "cmd_while",
            TokenType::WhileKeyword,
            vec![
                enquanto,
                lparen,
                cond,
                rparen,
                faca,
                cmd_ou_bloco,
                action_create_while,
            ],
        );

        table
    }

    /**
     <cmd_do> -> do <cmd_ou_bloco> while ( <cond> ) ;
    */
    fn create_cmd_do_table(mut table: ParseTable) -> ParseTable {
        let faca = Symbol::Terminal(TokenType::DoKeyword);
        let cmd_ou_bloco = Symbol::NonTerminal("cmd_ou_bloco".to_string());
        let enquanto = Symbol::Terminal(TokenType::WhileKeyword);
        let lparen = Symbol::Terminal(TokenType::LParenOperator);
        let cond = Symbol::NonTerminal("cond".to_string());
        let rparen = Symbol::Terminal(TokenType::RParenOperator);
        let semicolon = Symbol::Terminal(TokenType::SemiColonPunctuation);
        let action_create_do = Symbol::Action(ActionKind::CreateDoWhile);

        table.set_entry(
            "cmd_do",
            TokenType::DoKeyword,
            vec![
                faca,
                cmd_ou_bloco,
                enquanto,
                lparen,
                cond,
                rparen,
                semicolon,
                action_create_do,
            ],
        );

        table
    }

    /**
     <cmd_for> -> for ( id ; num ; num ; <E> ) <cmd_ou_bloco>
    */
    fn create_cmd_for_table(mut table: ParseTable) -> ParseTable {
        let enquanto = Symbol::Terminal(TokenType::ForKeyword);
        let lparen = Symbol::Terminal(TokenType::LParenOperator);
        let id = Symbol::Terminal(TokenType::Id);
        let semicolon = || Symbol::Terminal(TokenType::SemiColonPunctuation);
        let num = || Symbol::Terminal(TokenType::Number);
        let expr = Symbol::NonTerminal("E".to_string());
        let rparen = Symbol::Terminal(TokenType::RParenOperator);
        let cmd_ou_bloco = Symbol::NonTerminal("cmd_ou_bloco".to_string());
        let action_create_for = Symbol::Action(ActionKind::CreateFor);

        table.set_entry(
            "cmd_for",
            TokenType::ForKeyword,
            vec![
                enquanto,
                lparen,
                id,
                semicolon(),
                num(),
                semicolon(),
                num(),
                semicolon(),
                expr,
                rparen,
                cmd_ou_bloco,
                action_create_for,
            ],
        );

        table
    }
}
