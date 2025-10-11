use super::ast::*;
use std::fmt::Write;

pub struct PrettyPrinter {
    indent_level: usize,
    indent_string: String,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {
            indent_level: 0,
            indent_string: "  ".to_string(), // 2 Leerzeichen pro Ebene
        }
    }

    pub fn print_module(&mut self, module: &Module) -> String {
        let mut output = String::new();

        // MODULE header
        writeln!(output, "MODULE {};", module.name).unwrap();
        writeln!(output).unwrap();

        // Imports
        if !module.imports.is_empty() {
            self.print_imports(&mut output, &module.imports);
            writeln!(output).unwrap();
        }

        // Declarations
        self.print_decl_sequence(&mut output, &module.declarations);

        // Body
        if let Some(body) = &module.body {
            if !body.is_empty() {
                writeln!(output, "BEGIN").unwrap();
                self.indent_level += 1;
                self.print_statement_sequence(&mut output, body);
                self.indent_level -= 1;
            }
        }

        // END
        writeln!(output, "END {}.", module.end_name).unwrap();

        output
    }

    // ========================================================================
    // Imports
    // ========================================================================

    fn print_imports(&mut self, output: &mut String, imports: &[Import]) {
        write!(output, "IMPORT").unwrap();
        for (i, import) in imports.iter().enumerate() {
            if i > 0 {
                write!(output, ",").unwrap();
            }
            write!(output, " ").unwrap();
            if let Some(alias) = &import.alias {
                write!(output, "{} := {}", alias, import.module_name).unwrap();
            } else {
                write!(output, "{}", import.module_name).unwrap();
            }
        }
        writeln!(output, ";").unwrap();
    }

    // ========================================================================
    // Deklarationen
    // ========================================================================

    fn print_decl_sequence(&mut self, output: &mut String, decls: &DeclSequence) {
        // Constants
        if !decls.constants.is_empty() {
            writeln!(output, "CONST").unwrap();
            self.indent_level += 1;
            for const_decl in &decls.constants {
                self.print_indent(output);
                write!(output, "{} = ", const_decl.name).unwrap();
                self.print_expression(output, &const_decl.value);
                writeln!(output, ";").unwrap();
            }
            self.indent_level -= 1;
            writeln!(output).unwrap();
        }

        // Types
        if !decls.types.is_empty() {
            writeln!(output, "TYPE").unwrap();
            self.indent_level += 1;
            for type_decl in &decls.types {
                self.print_indent(output);
                write!(output, "{} = ", type_decl.name).unwrap();
                self.print_type(output, &type_decl.type_def);
                writeln!(output, ";").unwrap();
            }
            self.indent_level -= 1;
            writeln!(output).unwrap();
        }

        // Variables
        if !decls.variables.is_empty() {
            writeln!(output, "VAR").unwrap();
            self.indent_level += 1;
            for var_decl in &decls.variables {
                self.print_indent(output);
                for (i, name) in var_decl.names.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ").unwrap();
                    }
                    write!(output, "{}", name).unwrap();
                }
                write!(output, ": ").unwrap();
                self.print_type(output, &var_decl.var_type);
                writeln!(output, ";").unwrap();
            }
            self.indent_level -= 1;
            writeln!(output).unwrap();
        }

        // Procedures
        for proc_decl in &decls.procedures {
            self.print_procedure(output, proc_decl);
            writeln!(output).unwrap();
        }
    }

    // ========================================================================
    // Typen
    // ========================================================================

    fn print_type(&mut self, output: &mut String, type_def: &Type) {
        match type_def {
            Type::Qualident(qualident) => {
                write!(output, "{}", qualident).unwrap();
            }
            Type::Array { lengths, element_type } => {
                write!(output, "ARRAY ").unwrap();
                for (i, length) in lengths.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ").unwrap();
                    }
                    self.print_expression(output, length);
                }
                write!(output, " OF ").unwrap();
                self.print_type(output, element_type);
            }
            Type::Record { base_type, fields } => {
                write!(output, "RECORD").unwrap();
                if let Some(base) = base_type {
                    write!(output, "({})", base).unwrap();
                }
                if !fields.is_empty() {
                    writeln!(output).unwrap();
                    self.indent_level += 1;
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            writeln!(output, ";").unwrap();
                        }
                        self.print_indent(output);
                        for (j, name) in field.names.iter().enumerate() {
                            if j > 0 {
                                write!(output, ", ").unwrap();
                            }
                            write!(output, "{}", name).unwrap();
                        }
                        write!(output, ": ").unwrap();
                        self.print_type(output, &field.field_type);
                    }
                    writeln!(output).unwrap();
                    self.indent_level -= 1;
                    self.print_indent(output);
                }
                write!(output, "END").unwrap();
            }
            Type::Pointer { target_type } => {
                write!(output, "POINTER TO ").unwrap();
                self.print_type(output, target_type);
            }
            Type::Procedure { params } => {
                write!(output, "PROCEDURE").unwrap();
                if let Some(p) = params {
                    self.print_formal_parameters(output, p);
                }
            }
        }
    }

    fn print_formal_parameters(&mut self, output: &mut String, params: &FormalParameters) {
        write!(output, "(").unwrap();
        for (i, section) in params.sections.iter().enumerate() {
            if i > 0 {
                write!(output, "; ").unwrap();
            }
            if section.is_var {
                write!(output, "VAR ").unwrap();
            }
            for (j, name) in section.names.iter().enumerate() {
                if j > 0 {
                    write!(output, ", ").unwrap();
                }
                write!(output, "{}", name).unwrap();
            }
            write!(output, ": ").unwrap();
            self.print_type(output, &section.param_type);
        }
        write!(output, ")").unwrap();
        if let Some(ret) = &params.return_type {
            write!(output, ": {}", ret).unwrap();
        }
    }

    // ========================================================================
    // Prozeduren
    // ========================================================================

    fn print_procedure(&mut self, output: &mut String, proc: &ProcedureDeclaration) {
        self.print_indent(output);
        write!(output, "PROCEDURE").unwrap();

        if proc.is_forward {
            write!(output, " ^").unwrap();
        }

        write!(output, " {}", proc.name).unwrap();

        if let Some(params) = &proc.params {
            self.print_formal_parameters(output, params);
        }

        writeln!(output, ";").unwrap();

        if proc.is_forward {
            return;
        }

        self.indent_level += 1;

        // Declarations
        if !proc.declarations.constants.is_empty()
            || !proc.declarations.types.is_empty()
            || !proc.declarations.variables.is_empty()
            || !proc.declarations.procedures.is_empty()
        {
            self.print_decl_sequence(output, &proc.declarations);
        }

        // Body
        if let Some(body) = &proc.body {
            if !body.is_empty() {
                self.print_indent(output);
                writeln!(output, "BEGIN").unwrap();
                self.indent_level += 1;
                self.print_statement_sequence(output, body);
                self.indent_level -= 1;
            }
        }

        // Return
        if let Some(ret_expr) = &proc.return_expr {
            self.print_indent(output);
            write!(output, "RETURN ").unwrap();
            self.print_expression(output, ret_expr);
            writeln!(output).unwrap();
        }

        self.indent_level -= 1;
        self.print_indent(output);
        writeln!(output, "END {};", proc.end_name).unwrap();
    }

    // ========================================================================
    // Statements
    // ========================================================================

    fn print_statement_sequence(&mut self, output: &mut String, statements: &[Statement]) {
        for (i, stmt) in statements.iter().enumerate() {
            if i > 0 && !matches!(stmt, Statement::Empty) {
                writeln!(output, ";").unwrap();
            }
            self.print_statement(output, stmt);
        }
        writeln!(output).unwrap();
    }

    fn print_statement(&mut self, output: &mut String, stmt: &Statement) {
        match stmt {
            Statement::Empty => {}
            Statement::Assignment { target, value } => {
                self.print_indent(output);
                self.print_designator(output, target);
                write!(output, " := ").unwrap();
                self.print_expression(output, value);
            }
            Statement::ProcedureCall { designator, arguments } => {
                self.print_indent(output);
                self.print_designator(output, designator);
                if !arguments.is_empty() {
                    write!(output, "(").unwrap();
                    for (i, arg) in arguments.iter().enumerate() {
                        if i > 0 {
                            write!(output, ", ").unwrap();
                        }
                        self.print_expression(output, arg);
                    }
                    write!(output, ")").unwrap();
                }
            }
            Statement::If { condition, then_body, elsif_parts, else_body } => {
                self.print_indent(output);
                write!(output, "IF ").unwrap();
                self.print_expression(output, condition);
                writeln!(output, " THEN").unwrap();
                self.indent_level += 1;
                self.print_statement_sequence(output, then_body);
                self.indent_level -= 1;

                for (elsif_cond, elsif_body) in elsif_parts {
                    self.print_indent(output);
                    write!(output, "ELSIF ").unwrap();
                    self.print_expression(output, elsif_cond);
                    writeln!(output, " THEN").unwrap();
                    self.indent_level += 1;
                    self.print_statement_sequence(output, elsif_body);
                    self.indent_level -= 1;
                }

                if let Some(else_stmts) = else_body {
                    self.print_indent(output);
                    writeln!(output, "ELSE").unwrap();
                    self.indent_level += 1;
                    self.print_statement_sequence(output, else_stmts);
                    self.indent_level -= 1;
                }

                self.print_indent(output);
                write!(output, "END").unwrap();
            }
            Statement::Case { expr, cases, else_body } => {
                self.print_indent(output);
                write!(output, "CASE ").unwrap();
                self.print_expression(output, expr);
                writeln!(output, " OF").unwrap();
                self.indent_level += 1;

                for (i, case) in cases.iter().enumerate() {
                    if i > 0 {
                        writeln!(output).unwrap();
                        self.print_indent(output);
                        writeln!(output, "|").unwrap();
                    }
                    self.print_indent(output);
                    for (j, label) in case.labels.iter().enumerate() {
                        if j > 0 {
                            write!(output, ", ").unwrap();
                        }
                        self.print_expression(output, &label.start);
                        if let Some(end) = &label.end {
                            write!(output, "..").unwrap();
                            self.print_expression(output, end);
                        }
                    }
                    writeln!(output, ":").unwrap();
                    self.indent_level += 1;
                    self.print_statement_sequence(output, &case.body);
                    self.indent_level -= 1;
                }

                if let Some(else_stmts) = else_body {
                    self.print_indent(output);
                    writeln!(output, "ELSE").unwrap();
                    self.indent_level += 1;
                    self.print_statement_sequence(output, else_stmts);
                    self.indent_level -= 1;
                }

                self.indent_level -= 1;
                self.print_indent(output);
                write!(output, "END").unwrap();
            }
            Statement::While { condition, body, elsif_parts } => {
                self.print_indent(output);
                write!(output, "WHILE ").unwrap();
                self.print_expression(output, condition);
                writeln!(output, " DO").unwrap();
                self.indent_level += 1;
                self.print_statement_sequence(output, body);
                self.indent_level -= 1;

                for (elsif_cond, elsif_body) in elsif_parts {
                    self.print_indent(output);
                    write!(output, "ELSIF ").unwrap();
                    self.print_expression(output, elsif_cond);
                    writeln!(output, " DO").unwrap();
                    self.indent_level += 1;
                    self.print_statement_sequence(output, elsif_body);
                    self.indent_level -= 1;
                }

                self.print_indent(output);
                write!(output, "END").unwrap();
            }
            Statement::Repeat { body, condition } => {
                self.print_indent(output);
                writeln!(output, "REPEAT").unwrap();
                self.indent_level += 1;
                self.print_statement_sequence(output, body);
                self.indent_level -= 1;
                self.print_indent(output);
                write!(output, "UNTIL ").unwrap();
                self.print_expression(output, condition);
            }
            Statement::For { variable, start, end, step, body } => {
                self.print_indent(output);
                write!(output, "FOR {} := ", variable).unwrap();
                self.print_expression(output, start);
                write!(output, " TO ").unwrap();
                self.print_expression(output, end);
                if let Some(step_expr) = step {
                    write!(output, " BY ").unwrap();
                    self.print_expression(output, step_expr);
                }
                writeln!(output, " DO").unwrap();
                self.indent_level += 1;
                self.print_statement_sequence(output, body);
                self.indent_level -= 1;
                self.print_indent(output);
                write!(output, "END").unwrap();
            }
        }
    }

    // ========================================================================
    // Ausdrücke
    // ========================================================================

    fn print_expression(&mut self, output: &mut String, expr: &Expression) {
        match expr {
            Expression::IntegerLiteral(val) => {
                write!(output, "{}", val).unwrap();
            }
            Expression::RealLiteral(val) => {
                write!(output, "{}", val).unwrap();
            }
            Expression::StringLiteral(val) => {
                write!(output, "\"{}\"", val).unwrap();
            }
            Expression::BooleanLiteral(val) => {
                write!(output, "{}", if *val { "TRUE" } else { "FALSE" }).unwrap();
            }
            Expression::Nil => {
                write!(output, "NIL").unwrap();
            }
            Expression::Set(elements) => {
                write!(output, "{{").unwrap();
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ").unwrap();
                    }
                    self.print_expression(output, &elem.start);
                    if let Some(end) = &elem.end {
                        write!(output, "..").unwrap();
                        self.print_expression(output, end);
                    }
                }
                write!(output, "}}").unwrap();
            }
            Expression::Designator(designator) => {
                self.print_designator(output, designator);
            }
            Expression::FunctionCall { designator, arguments } => {
                self.print_designator(output, designator);
                write!(output, "(").unwrap();
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ").unwrap();
                    }
                    self.print_expression(output, arg);
                }
                write!(output, ")").unwrap();
            }
            Expression::Unary { op, expr } => {
                match op {
                    UnaryOp::Plus => write!(output, "+").unwrap(),
                    UnaryOp::Minus => write!(output, "-").unwrap(),
                    UnaryOp::Not => write!(output, "~").unwrap(),
                }
                self.print_expression(output, expr);
            }
            Expression::Binary { left, op, right } => {
                let needs_parens = matches!(**left, Expression::Binary { .. });
                if needs_parens {
                    write!(output, "(").unwrap();
                }
                self.print_expression(output, left);
                if needs_parens {
                    write!(output, ")").unwrap();
                }

                write!(output, " ").unwrap();
                match op {
                    BinaryOp::Add => write!(output, "+").unwrap(),
                    BinaryOp::Sub => write!(output, "-").unwrap(),
                    BinaryOp::Mul => write!(output, "*").unwrap(),
                    BinaryOp::Div => write!(output, "/").unwrap(),
                    BinaryOp::IntDiv => write!(output, "DIV").unwrap(),
                    BinaryOp::Mod => write!(output, "MOD").unwrap(),
                    BinaryOp::And => write!(output, "&").unwrap(),
                    BinaryOp::Or => write!(output, "OR").unwrap(),
                    BinaryOp::Equal => write!(output, "=").unwrap(),
                    BinaryOp::NotEqual => write!(output, "#").unwrap(),
                    BinaryOp::Less => write!(output, "<").unwrap(),
                    BinaryOp::LessEqual => write!(output, "<=").unwrap(),
                    BinaryOp::Greater => write!(output, ">").unwrap(),
                    BinaryOp::GreaterEqual => write!(output, ">=").unwrap(),
                    BinaryOp::In => write!(output, "IN").unwrap(),
                    BinaryOp::Is => write!(output, "IS").unwrap(),
                }
                write!(output, " ").unwrap();

                let needs_parens = matches!(**right, Expression::Binary { .. });
                if needs_parens {
                    write!(output, "(").unwrap();
                }
                self.print_expression(output, right);
                if needs_parens {
                    write!(output, ")").unwrap();
                }
            }
        }
    }

    // ========================================================================
    // Designator
    // ========================================================================

    fn print_designator(&mut self, output: &mut String, designator: &Designator) {
        write!(output, "{}", designator.base).unwrap();
        for selector in &designator.selectors {
            match selector {
                Selector::Field(name) => {
                    write!(output, ".{}", name).unwrap();
                }
                Selector::Index(indices) => {
                    write!(output, "[").unwrap();
                    for (i, idx) in indices.iter().enumerate() {
                        if i > 0 {
                            write!(output, ", ").unwrap();
                        }
                        self.print_expression(output, idx);
                    }
                    write!(output, "]").unwrap();
                }
                Selector::Dereference => {
                    write!(output, "^").unwrap();
                }
                Selector::TypeGuard(type_name) => {
                    write!(output, "({})", type_name).unwrap();
                }
            }
        }
    }

    // ========================================================================
    // Hilfsfunktionen
    // ========================================================================

    fn print_indent(&self, output: &mut String) {
        for _ in 0..self.indent_level {
            write!(output, "{}", self.indent_string).unwrap();
        }
    }
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}