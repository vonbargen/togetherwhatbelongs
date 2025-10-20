use crate::scanner::{Token, TokenType};
use super::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Module, String> {
        self.parse_module()
    }

    // ========================================================================
    // Module
    // ========================================================================

    fn parse_module(&mut self) -> Result<Module, String> {
        self.expect(TokenType::Module)?;
        let name = self.parse_identifier()?;
        self.expect(TokenType::Semicolon)?;

        let imports = if self.check(&TokenType::Import) {
            self.parse_import_list()?
        } else {
            Vec::new()
        };

        let declarations = self.parse_decl_sequence()?;

        let body = if self.check(&TokenType::Begin) {
            self.advance();
            Some(self.parse_statement_sequence()?)
        } else {
            None
        };

        self.expect(TokenType::End)?;
        let end_name = self.parse_identifier()?;
        self.expect(TokenType::Period)?;

        if name != end_name {
            return Err(format!(
                "Modulname stimmt nicht überein: '{}' != '{}'",
                name, end_name
            ));
        }

        Ok(Module {
            name,
            imports,
            declarations,
            body,
            end_name,
        })
    }

    fn parse_import_list(&mut self) -> Result<Vec<Import>, String> {
        self.expect(TokenType::Import)?;
        let mut imports = Vec::new();

        loop {
            let import = self.parse_import()?;
            imports.push(import);

            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }

        self.expect(TokenType::Semicolon)?;
        Ok(imports)
    }

    fn parse_import(&mut self) -> Result<Import, String> {
        let first_ident = self.parse_identifier()?;

        if self.match_token(&TokenType::Assign) {
            // alias := module
            let module_name = self.parse_identifier()?;
            Ok(Import {
                alias: Some(first_ident),
                module_name,
            })
        } else {
            Ok(Import {
                alias: None,
                module_name: first_ident,
            })
        }
    }

    // ========================================================================
    // Deklarationen
    // ========================================================================

    fn parse_decl_sequence(&mut self) -> Result<DeclSequence, String> {
        let mut decls = DeclSequence::default();

        loop {
            if self.check(&TokenType::Const) {
                self.advance();
                while !self.check(&TokenType::Type)
                    && !self.check(&TokenType::Var)
                    && !self.check(&TokenType::Procedure)
                    && !self.check(&TokenType::Begin)
                    && !self.check(&TokenType::End)
                {
                    decls.constants.push(self.parse_const_declaration()?);
                    self.expect(TokenType::Semicolon)?;
                }
            } else if self.check(&TokenType::Type) {
                self.advance();
                while !self.check(&TokenType::Const)
                    && !self.check(&TokenType::Var)
                    && !self.check(&TokenType::Procedure)
                    && !self.check(&TokenType::Begin)
                    && !self.check(&TokenType::End)
                {
                    decls.types.push(self.parse_type_declaration()?);
                    self.expect(TokenType::Semicolon)?;
                }
            } else if self.check(&TokenType::Var) {
                self.advance();
                while !self.check(&TokenType::Const)
                    && !self.check(&TokenType::Type)
                    && !self.check(&TokenType::Procedure)
                    && !self.check(&TokenType::Begin)
                    && !self.check(&TokenType::End)
                {
                    decls.variables.push(self.parse_variable_declaration()?);
                    self.expect(TokenType::Semicolon)?;
                }
            } else if self.check(&TokenType::Procedure) {
                decls.procedures.push(self.parse_procedure_declaration()?);
                self.expect(TokenType::Semicolon)?;
            } else {
                break;
            }
        }

        Ok(decls)
    }

    fn parse_const_declaration(&mut self) -> Result<ConstDeclaration, String> {
        let name = self.parse_ident_def()?;
        self.expect(TokenType::Equal)?;
        let value = self.parse_expression()?;

        Ok(ConstDeclaration { name, value })
    }

    fn parse_type_declaration(&mut self) -> Result<TypeDeclaration, String> {
        let name = self.parse_ident_def()?;
        self.expect(TokenType::Equal)?;
        let type_def = self.parse_type()?;

        Ok(TypeDeclaration { name, type_def })
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, String> {
        let names = self.parse_ident_list()?;
        self.expect(TokenType::Colon)?;
        let var_type = self.parse_type()?;

        Ok(VariableDeclaration { names, var_type })
    }

    // ========================================================================
    // Typen
    // ========================================================================

    fn parse_type(&mut self) -> Result<Type, String> {
        if self.check(&TokenType::Array) {
            self.parse_array_type()
        } else if self.check(&TokenType::Record) {
            self.parse_record_type()
        } else if self.check(&TokenType::Pointer) {
            self.parse_pointer_type()
        } else if self.check(&TokenType::Procedure) {
            self.parse_procedure_type()
        } else {
            let qualident = self.parse_qualident()?;
            Ok(Type::Qualident(qualident))
        }
    }

    fn parse_array_type(&mut self) -> Result<Type, String> {
        self.expect(TokenType::Array)?;
        let mut lengths = Vec::new();

        loop {
            lengths.push(self.parse_expression()?);
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }

        self.expect(TokenType::Of)?;
        let element_type = Box::new(self.parse_type()?);

        Ok(Type::Array {
            lengths,
            element_type,
        })
    }

    fn parse_record_type(&mut self) -> Result<Type, String> {
        self.expect(TokenType::Record)?;

        let base_type = if self.match_token(&TokenType::LParen) {
            let base = self.parse_qualident()?;
            self.expect(TokenType::RParen)?;
            Some(base)
        } else {
            None
        };

        let mut fields = Vec::new();
        if !self.check(&TokenType::End) {
            loop {
                fields.push(self.parse_field_list()?);
                if !self.match_token(&TokenType::Semicolon) {
                    break;
                }
                if self.check(&TokenType::End) {
                    break;
                }
            }
        }

        self.expect(TokenType::End)?;

        Ok(Type::Record { base_type, fields })
    }

    fn parse_field_list(&mut self) -> Result<FieldList, String> {
        let names = self.parse_ident_list()?;
        self.expect(TokenType::Colon)?;
        let field_type = self.parse_type()?;

        Ok(FieldList { names, field_type })
    }

    fn parse_pointer_type(&mut self) -> Result<Type, String> {
        self.expect(TokenType::Pointer)?;
        self.expect(TokenType::To)?;
        let target_type = Box::new(self.parse_type()?);

        Ok(Type::Pointer { target_type })
    }

    fn parse_procedure_type(&mut self) -> Result<Type, String> {
        self.expect(TokenType::Procedure)?;

        let params = if self.check(&TokenType::LParen) {
            Some(self.parse_formal_parameters()?)
        } else {
            None
        };

        Ok(Type::Procedure { params })
    }

    fn parse_formal_parameters(&mut self) -> Result<FormalParameters, String> {
        self.expect(TokenType::LParen)?;

        let mut sections = Vec::new();
        if !self.check(&TokenType::RParen) {
            loop {
                sections.push(self.parse_fp_section()?);
                if !self.match_token(&TokenType::Semicolon) {
                    break;
                }
            }
        }

        self.expect(TokenType::RParen)?;

        let return_type = if self.match_token(&TokenType::Colon) {
            Some(self.parse_qualident()?)
        } else {
            None
        };

        Ok(FormalParameters {
            sections,
            return_type,
        })
    }

    fn parse_fp_section(&mut self) -> Result<FPSection, String> {
        let is_var = self.match_token(&TokenType::Var);

        let mut names = Vec::new();
        loop {
            names.push(self.parse_identifier()?);
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }

        self.expect(TokenType::Colon)?;
        let param_type = self.parse_type()?;

        Ok(FPSection {
            is_var,
            names,
            param_type,
        })
    }

    // ========================================================================
    // Prozeduren
    // ========================================================================

    fn parse_procedure_declaration(&mut self) -> Result<ProcedureDeclaration, String> {
        self.expect(TokenType::Procedure)?;

        // Forward-Deklaration?
        let is_forward = self.match_token(&TokenType::Caret);

        let name = self.parse_ident_def()?;

        let params = if self.check(&TokenType::LParen) {
            Some(self.parse_formal_parameters()?)
        } else {
            None
        };

        self.expect(TokenType::Semicolon)?;

        if is_forward {
            return Ok(ProcedureDeclaration {
                name,
                params,
                declarations: DeclSequence::default(),
                body: None,
                return_expr: None,
                end_name: String::new(),
                is_forward: true,
            });
        }

        let declarations = self.parse_decl_sequence()?;

        let body = if self.check(&TokenType::Begin) {
            self.advance();
            Some(self.parse_statement_sequence()?)
        } else {
            None
        };

        let return_expr = if self.match_token(&TokenType::Return) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(TokenType::End)?;
        let end_name = self.parse_identifier()?;

        if name.name != end_name {
            return Err(format!(
                "Prozedurname stimmt nicht überein: '{}' != '{}'",
                name.name, end_name
            ));
        }

        Ok(ProcedureDeclaration {
            name,
            params,
            declarations,
            body,
            return_expr,
            end_name,
            is_forward: false,
        })
    }

    // ========================================================================
    // Statements
    // ========================================================================

    fn parse_statement_sequence(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        loop {
            if self.is_statement_start() {
                statements.push(self.parse_statement()?);
            } else {
                // Leeres Statement nur wenn Semicolon folgt
                if self.check(&TokenType::Semicolon) {
                    statements.push(Statement::Empty);
                } else {
                    break;
                }
            }

            if !self.match_token(&TokenType::Semicolon) {
                break;
            }
        }

        Ok(statements)
    }

    fn is_statement_start(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::Identifier(_)
                | TokenType::If
                | TokenType::Case
                | TokenType::While
                | TokenType::Repeat
                | TokenType::For
        )
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.peek().token_type {
            TokenType::If => self.parse_if_statement(),
            TokenType::Case => self.parse_case_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::Repeat => self.parse_repeat_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Identifier(_) => self.parse_assignment_or_call(),
            _ => Ok(Statement::Empty),
        }
    }

    fn parse_assignment_or_call(&mut self) -> Result<Statement, String> {
        let designator = self.parse_designator()?;

        if self.match_token(&TokenType::Assign) {
            let value = self.parse_expression()?;
            Ok(Statement::Assignment {
                target: designator,
                value,
            })
        } else if self.check(&TokenType::LParen) {
            let arguments = self.parse_actual_parameters()?;
            Ok(Statement::ProcedureCall {
                designator,
                arguments,
            })
        } else {
            // Prozeduraufruf ohne Parameter
            Ok(Statement::ProcedureCall {
                designator,
                arguments: Vec::new(),
            })
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::If)?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::Then)?;
        let then_body = self.parse_statement_sequence()?;

        let mut elsif_parts = Vec::new();
        while self.match_token(&TokenType::Elsif) {
            let elsif_cond = self.parse_expression()?;
            self.expect(TokenType::Then)?;
            let elsif_body = self.parse_statement_sequence()?;
            elsif_parts.push((elsif_cond, elsif_body));
        }

        let else_body = if self.match_token(&TokenType::Else) {
            Some(self.parse_statement_sequence()?)
        } else {
            None
        };

        self.expect(TokenType::End)?;

        Ok(Statement::If {
            condition,
            then_body,
            elsif_parts,
            else_body,
        })
    }

    fn parse_case_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::Case)?;
        let expr = self.parse_expression()?;
        self.expect(TokenType::Of)?;

        let mut cases = Vec::new();
        loop {
            if self.check(&TokenType::Else) || self.check(&TokenType::End) {
                break;
            }
            cases.push(self.parse_case_clause()?);
            if !self.match_token(&TokenType::Bar) {
                break;
            }
        }

        let else_body = if self.match_token(&TokenType::Else) {
            Some(self.parse_statement_sequence()?)
        } else {
            None
        };

        self.expect(TokenType::End)?;

        Ok(Statement::Case {
            expr,
            cases,
            else_body,
        })
    }

    fn parse_case_clause(&mut self) -> Result<CaseClause, String> {
        let mut labels = Vec::new();

        if !self.check(&TokenType::Bar) && !self.check(&TokenType::Else) && !self.check(&TokenType::End) {
            loop {
                labels.push(self.parse_case_label()?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }

            self.expect(TokenType::Colon)?;
        }

        let body = self.parse_statement_sequence()?;

        Ok(CaseClause { labels, body })
    }

    fn parse_case_label(&mut self) -> Result<CaseLabel, String> {
        let start = self.parse_expression()?;

        let end = if self.match_token(&TokenType::DotDot) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(CaseLabel { start, end })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::While)?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::Do)?;
        let body = self.parse_statement_sequence()?;

        let mut elsif_parts = Vec::new();
        while self.match_token(&TokenType::Elsif) {
            let elsif_cond = self.parse_expression()?;
            self.expect(TokenType::Do)?;
            let elsif_body = self.parse_statement_sequence()?;
            elsif_parts.push((elsif_cond, elsif_body));
        }

        self.expect(TokenType::End)?;

        Ok(Statement::While {
            condition,
            body,
            elsif_parts,
        })
    }

    fn parse_repeat_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::Repeat)?;
        let body = self.parse_statement_sequence()?;
        self.expect(TokenType::Until)?;
        let condition = self.parse_expression()?;

        Ok(Statement::Repeat { body, condition })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::For)?;
        let variable = self.parse_identifier()?;
        self.expect(TokenType::Assign)?;
        let start = self.parse_expression()?;
        self.expect(TokenType::To)?;
        let end = self.parse_expression()?;

        let step = if self.match_token(&TokenType::By) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(TokenType::Do)?;
        let body = self.parse_statement_sequence()?;
        self.expect(TokenType::End)?;

        Ok(Statement::For {
            variable,
            start,
            end,
            step,
            body,
        })
    }

    // ========================================================================
    // Ausdrücke
    // ========================================================================

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_simple_expression()?;

        if self.is_relation() {
            let op = self.parse_relation()?;
            let right = self.parse_simple_expression()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn is_relation(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::Equal
                | TokenType::NotEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::In
                | TokenType::Is
        )
    }

    fn parse_relation(&mut self) -> Result<BinaryOp, String> {
        let op = match &self.peek().token_type {
            TokenType::Equal => BinaryOp::Equal,
            TokenType::NotEqual => BinaryOp::NotEqual,
            TokenType::Less => BinaryOp::Less,
            TokenType::LessEqual => BinaryOp::LessEqual,
            TokenType::Greater => BinaryOp::Greater,
            TokenType::GreaterEqual => BinaryOp::GreaterEqual,
            TokenType::In => BinaryOp::In,
            TokenType::Is => BinaryOp::Is,
            _ => return Err(format!("Erwarte Vergleichsoperator, gefunden: {:?}", self.peek())),
        };
        self.advance();
        Ok(op)
    }

    fn parse_simple_expression(&mut self) -> Result<Expression, String> {
        let unary_op = if self.check(&TokenType::Plus) || self.check(&TokenType::Minus) {
            Some(if self.check(&TokenType::Plus) {
                UnaryOp::Plus
            } else {
                UnaryOp::Minus
            })
        } else {
            None
        };

        if unary_op.is_some() {
            self.advance();
        }

        let mut expr = self.parse_term()?;

        if let Some(op) = unary_op {
            expr = Expression::Unary {
                op,
                expr: Box::new(expr),
            };
        }

        while self.is_add_operator() {
            let op = self.parse_add_operator()?;
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn is_add_operator(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::Plus | TokenType::Minus | TokenType::Or
        )
    }

    fn parse_add_operator(&mut self) -> Result<BinaryOp, String> {
        let op = match &self.peek().token_type {
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Sub,
            TokenType::Or => BinaryOp::Or,
            _ => return Err(format!("Erwarte Additionsoperator, gefunden: {:?}", self.peek())),
        };
        self.advance();
        Ok(op)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_factor()?;

        while self.is_mul_operator() {
            let op = self.parse_mul_operator()?;
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn is_mul_operator(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::Times | TokenType::Slash | TokenType::Div | TokenType::Mod | TokenType::Ampersand
        )
    }

    fn parse_mul_operator(&mut self) -> Result<BinaryOp, String> {
        let op = match &self.peek().token_type {
            TokenType::Times => BinaryOp::Mul,
            TokenType::Slash => BinaryOp::Div,
            TokenType::Div => BinaryOp::IntDiv,
            TokenType::Mod => BinaryOp::Mod,
            TokenType::Ampersand => BinaryOp::And,
            _ => return Err(format!("Erwarte Multiplikationsoperator, gefunden: {:?}", self.peek())),
        };
        self.advance();
        Ok(op)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        match &self.peek().token_type.clone() {
            TokenType::IntegerLiteral(val) => {
                let v = *val;
                self.advance();
                Ok(Expression::IntegerLiteral(v))
            }
            TokenType::RealLiteral(val) => {
                let v = *val;
                self.advance();
                Ok(Expression::RealLiteral(v))
            }
            TokenType::StringLiteral(val) => {
                let v = val.clone();
                self.advance();
                Ok(Expression::StringLiteral(v))
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::BooleanLiteral(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expression::BooleanLiteral(false))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expression::Nil)
            }
            TokenType::LBrace => self.parse_set(),
            TokenType::Tilde => {
                self.advance();
                let expr = self.parse_factor()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            }
            TokenType::Identifier(_) => {
                let designator = self.parse_designator()?;
                if self.check(&TokenType::LParen) {
                    let arguments = self.parse_actual_parameters()?;
                    Ok(Expression::FunctionCall {
                        designator,
                        arguments,
                    })
                } else {
                    Ok(Expression::Designator(designator))
                }
            }
            _ => Err(format!("Unerwartetes Token in Ausdruck: {:?}", self.peek())),
        }
    }

    fn parse_set(&mut self) -> Result<Expression, String> {
        self.expect(TokenType::LBrace)?;
        let mut elements = Vec::new();

        if !self.check(&TokenType::RBrace) {
            loop {
                let start = Box::new(self.parse_expression()?);
                let end = if self.match_token(&TokenType::DotDot) {
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                };
                elements.push(SetElement { start, end });

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.expect(TokenType::RBrace)?;
        Ok(Expression::Set(elements))
    }

    fn parse_actual_parameters(&mut self) -> Result<Vec<Expression>, String> {
        self.expect(TokenType::LParen)?;
        let mut args = Vec::new();

        if !self.check(&TokenType::RParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.expect(TokenType::RParen)?;
        Ok(args)
    }

    // ========================================================================
    // Designator
    // ========================================================================

    fn parse_designator(&mut self) -> Result<Designator, String> {
        let base = self.parse_qualident()?;
        let mut selectors = Vec::new();

        loop {
            match &self.peek().token_type {
                TokenType::Period => {
                    self.advance();
                    let field = self.parse_identifier()?;
                    selectors.push(Selector::Field(field));
                }
                TokenType::LBracket => {
                    self.advance();
                    let mut indices = Vec::new();
                    loop {
                        indices.push(self.parse_expression()?);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                    self.expect(TokenType::RBracket)?;
                    selectors.push(Selector::Index(indices));
                }
                TokenType::Caret => {
                    self.advance();
                    selectors.push(Selector::Dereference);
                }
                // TypeGuard entfernt - wird später bei Bedarf hinzugefügt
                _ => break,
            }
        }

        Ok(Designator { base, selectors })
    }

    // ========================================================================
    // Hilfsfunktionen
    // ========================================================================

    fn parse_ident_def(&mut self) -> Result<IdentDef, String> {
        let name = self.parse_identifier()?;
        let exported = if self.match_token(&TokenType::Times) {
            ExportMark::ReadOnly
        } else if self.match_token(&TokenType::Minus) {
            ExportMark::ReadWrite
        } else {
            ExportMark::None
        };

        Ok(IdentDef { name, exported })
    }

    fn parse_qualident(&mut self) -> Result<Qualident, String> {
        let first = self.parse_identifier()?;

        if self.match_token(&TokenType::Period) {
            let second = self.parse_identifier()?;
            Ok(Qualident::with_module(first, second))
        } else {
            Ok(Qualident::new(first))
        }
    }

    fn parse_ident_list(&mut self) -> Result<Vec<IdentDef>, String> {
        let mut idents = Vec::new();
        loop {
            idents.push(self.parse_ident_def()?);
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        Ok(idents)
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        if let TokenType::Identifier(name) = &self.peek().token_type {
            let n = name.clone();
            self.advance();
            Ok(n)
        } else {
            Err(format!("Erwarte Identifier, gefunden: {:?}", self.peek()))
        }
    }

    // ========================================================================
    // Token-Verwaltung
    // ========================================================================

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        if self.current + offset < self.tokens.len() {
            Some(&self.tokens[self.current + offset])
        } else {
            None
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token_type: TokenType) -> Result<(), String> {
        if self.check(&token_type) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Erwarte {:?}, gefunden: {:?} in Zeile {}",
                token_type,
                self.peek().token_type,
                self.peek().line
            ))
        }
    }
}