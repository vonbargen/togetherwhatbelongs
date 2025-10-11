use crate::parser::ast::*;
use std::fmt::Write;

pub struct CGenerator {
    output: String,
    indent_level: usize,
    label_counter: usize,
    temp_counter: usize,
}

impl CGenerator {
    pub fn new() -> Self {
        CGenerator {
            output: String::new(),
            indent_level: 0,
            label_counter: 0,
            temp_counter: 0,
        }
    }

    pub fn generate(&mut self, module: &Module) -> String {
        self.output.clear();

        // C Header
        self.emit_line("#include <stdio.h>");
        self.emit_line("#include <stdlib.h>");
        self.emit_line("#include <stdbool.h>");
        self.emit_line("#include <string.h>");
        self.emit_line("#include <stdint.h>");
        self.emit_line("");

        // Forward-Deklarationen für Prozeduren
        self.emit_forward_declarations(&module.declarations);
        self.emit_line("");

        // Globale Konstanten
        self.generate_constants(&module.declarations.constants);

        // Type Definitions
        self.generate_types(&module.declarations.types);

        // Globale Variablen
        self.generate_global_variables(&module.declarations.variables);

        // Prozeduren
        for proc in &module.declarations.procedures {
            self.generate_procedure(proc);
            self.emit_line("");
        }

        // Main-Funktion (Modul-Body)
        self.emit_line("int main(void) {");
        self.indent_level += 1;

        if let Some(body) = &module.body {
            self.generate_statement_sequence(body);
        }

        self.emit_line("return 0;");
        self.indent_level -= 1;
        self.emit_line("}");

        self.output.clone()
    }

    // ========================================================================
    // Forward-Deklarationen
    // ========================================================================

    fn emit_forward_declarations(&mut self, decls: &DeclSequence) {
        self.emit_line("// Forward declarations");
        for proc in &decls.procedures {
            if proc.is_forward {
                continue;
            }

            let return_type = if let Some(params) = &proc.params {
                if let Some(ret) = &params.return_type {
                    self.map_type_name(&ret.name)
                } else {
                    "void".to_string()
                }
            } else {
                "void".to_string()
            };

            let mut signature = format!("{} {}(", return_type, self.mangle_name(&proc.name.name));

            if let Some(params) = &proc.params {
                let param_strs: Vec<String> = params.sections.iter().flat_map(|section| {
                    section.names.iter().map(|name| {
                        let (type_str, array_suffix) = self.type_to_c_with_array(&section.param_type);
                        let ptr = if section.is_var { "*" } else { "" };
                        format!("{}{} {}{}", type_str, ptr, self.mangle_name(name), array_suffix)
                    }).collect::<Vec<_>>()
                }).collect();

                if param_strs.is_empty() {
                    signature.push_str("void");
                } else {
                    signature.push_str(&param_strs.join(", "));
                }
            } else {
                signature.push_str("void");
            }

            signature.push_str(");");
            self.emit_line(&signature);
        }
    }

    // ========================================================================
    // Konstanten
    // ========================================================================

    fn generate_constants(&mut self, constants: &[ConstDeclaration]) {
        if constants.is_empty() {
            return;
        }

        self.emit_line("// Constants");
        for const_decl in constants {
            let value = self.expression_to_c(&const_decl.value);
            self.emit_line(&format!(
                "#define {} {}",
                self.mangle_name(&const_decl.name.name),
                value
            ));
        }
        self.emit_line("");
    }

    // ========================================================================
    // Typen
    // ========================================================================

    fn generate_types(&mut self, types: &[TypeDeclaration]) {
        if types.is_empty() {
            return;
        }

        self.emit_line("// Type definitions");
        for type_decl in types {
            match &type_decl.type_def {
                Type::Record { fields, .. } => {
                    self.emit_line(&format!("typedef struct {{"));
                    self.indent_level += 1;
                    for field_list in fields {
                        let (type_str, array_suffix) = self.type_to_c_with_array(&field_list.field_type);
                        for name in &field_list.names {
                            self.emit_line(&format!("{} {}{};", type_str, self.mangle_name(&name.name), array_suffix));
                        }
                    }
                    self.indent_level -= 1;
                    self.emit_line(&format!("}} {};", self.mangle_name(&type_decl.name.name)));
                }
                Type::Array { lengths, element_type } => {
                    // Für Array-Typen verwenden wir jetzt eine korrekte C-Syntax
                    let elem_type = self.type_to_c_base(element_type);
                    self.emit_line(&format!(
                        "typedef {} {}",
                        elem_type,
                        self.mangle_name(&type_decl.name.name)
                    ));

                    let dims = lengths.iter()
                        .map(|len| {
                            if let Expression::IntegerLiteral(val) = len {
                                format!("[{}]", val)
                            } else if let Expression::Designator(d) = len {
                                // Konstanten-Referenz auflösen
                                format!("[{}]", self.mangle_name(&d.base.name))
                            } else {
                                "[100]".to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("");

                    self.emit_line(&format!("{};", dims));
                }
                Type::Pointer { target_type } => {
                    let target = self.type_to_c_base(target_type);
                    self.emit_line(&format!(
                        "typedef {}* {};",
                        target,
                        self.mangle_name(&type_decl.name.name)
                    ));
                }
                _ => {
                    let (type_str, _) = self.type_to_c_with_array(&type_decl.type_def);
                    self.emit_line(&format!(
                        "typedef {} {};",
                        type_str,
                        self.mangle_name(&type_decl.name.name)
                    ));
                }
            }
        }
        self.emit_line("");
    }

    fn type_to_c_base(&self, type_def: &Type) -> String {
        match type_def {
            Type::Qualident(q) => self.map_type_name(&q.name),
            Type::Array { element_type, .. } => {
                self.type_to_c_base(element_type)
            }
            Type::Pointer { target_type } => {
                format!("{}*", self.type_to_c_base(target_type))
            }
            _ => "void".to_string(),
        }
    }

    fn type_to_c_with_array(&self, type_def: &Type) -> (String, String) {
        match type_def {
            Type::Array { lengths, element_type } => {
                let base_type = self.type_to_c_base(element_type);
                let dims = lengths.iter()
                    .map(|len| {
                        if let Expression::IntegerLiteral(val) = len {
                            format!("[{}]", val)
                        } else if let Expression::Designator(d) = len {
                            format!("[{}]", self.mangle_name(&d.base.name))
                        } else {
                            "[100]".to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("");
                (base_type, dims)
            }
            _ => (self.type_to_c_base(type_def), String::new()),
        }
    }

    fn map_type_name(&self, name: &str) -> String {
        match name {
            "INTEGER" => "int64_t".to_string(),
            "REAL" => "double".to_string(),
            "BOOLEAN" => "bool".to_string(),
            "CHAR" => "char".to_string(),
            "SET" => "uint32_t".to_string(),
            _ => self.mangle_name(name),
        }
    }

    // ========================================================================
    // Variablen
    // ========================================================================

    fn generate_global_variables(&mut self, variables: &[VariableDeclaration]) {
        if variables.is_empty() {
            return;
        }

        self.emit_line("// Global variables");
        for var_decl in variables {
            let (type_str, array_suffix) = self.type_to_c_with_array(&var_decl.var_type);
            for name in &var_decl.names {
                self.emit_line(&format!("{} {}{};", type_str, self.mangle_name(&name.name), array_suffix));
            }
        }
        self.emit_line("");
    }

    // ========================================================================
    // Prozeduren
    // ========================================================================

    fn generate_procedure(&mut self, proc: &ProcedureDeclaration) {
        if proc.is_forward {
            return;
        }

        let return_type = if let Some(params) = &proc.params {
            if let Some(ret) = &params.return_type {
                self.map_type_name(&ret.name)
            } else {
                "void".to_string()
            }
        } else {
            "void".to_string()
        };

        let mut signature = format!("{} {}(", return_type, self.mangle_name(&proc.name.name));

        if let Some(params) = &proc.params {
            let param_strs: Vec<String> = params.sections.iter().flat_map(|section| {
                section.names.iter().map(|name| {
                    let (type_str, array_suffix) = self.type_to_c_with_array(&section.param_type);
                    let ptr = if section.is_var { "*" } else { "" };
                    format!("{}{} {}{}", type_str, ptr, self.mangle_name(name), array_suffix)
                }).collect::<Vec<_>>()
            }).collect();

            if param_strs.is_empty() {
                signature.push_str("void");
            } else {
                signature.push_str(&param_strs.join(", "));
            }
        } else {
            signature.push_str("void");
        }

        signature.push_str(") {");
        self.emit_line(&signature);
        self.indent_level += 1;

        // Lokale Variablen
        for var_decl in &proc.declarations.variables {
            let (type_str, array_suffix) = self.type_to_c_with_array(&var_decl.var_type);
            for name in &var_decl.names {
                self.emit_line(&format!("{} {}{};", type_str, self.mangle_name(&name.name), array_suffix));
            }
        }

        // Body
        if let Some(body) = &proc.body {
            self.generate_statement_sequence(body);
        }

        // Return
        if let Some(ret_expr) = &proc.return_expr {
            self.emit_line(&format!("return {};", self.expression_to_c(ret_expr)));
        }

        self.indent_level -= 1;
        self.emit_line("}");
    }

    // ========================================================================
    // Statements
    // ========================================================================

    fn generate_statement_sequence(&mut self, statements: &[Statement]) {
        for stmt in statements {
            self.generate_statement(stmt);
        }
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Empty => {}
            Statement::Assignment { target, value } => {
                let target_str = self.designator_to_c(target);
                let value_str = self.expression_to_c(value);
                self.emit_line(&format!("{} = {};", target_str, value_str));
            }
            Statement::ProcedureCall { designator, arguments } => {
                let proc_name = self.designator_to_c(designator);
                let args: Vec<String> = arguments.iter()
                    .map(|arg| self.expression_to_c(arg))
                    .collect();
                self.emit_line(&format!("{}({});", proc_name, args.join(", ")));
            }
            Statement::If { condition, then_body, elsif_parts, else_body } => {
                let cond_str = self.expression_to_c(condition);
                self.emit_line(&format!("if ({}) {{", cond_str));
                self.indent_level += 1;
                self.generate_statement_sequence(then_body);
                self.indent_level -= 1;
                self.emit_line("}");

                for (elsif_cond, elsif_body) in elsif_parts {
                    let elsif_str = self.expression_to_c(elsif_cond);
                    self.emit_line(&format!("else if ({}) {{", elsif_str));
                    self.indent_level += 1;
                    self.generate_statement_sequence(elsif_body);
                    self.indent_level -= 1;
                    self.emit_line("}");
                }

                if let Some(else_stmts) = else_body {
                    self.emit_line("else {");
                    self.indent_level += 1;
                    self.generate_statement_sequence(else_stmts);
                    self.indent_level -= 1;
                    self.emit_line("}");
                }
            }
            Statement::Case { expr, cases, else_body } => {
                let expr_str = self.expression_to_c(expr);
                self.emit_line(&format!("switch ({}) {{", expr_str));
                self.indent_level += 1;

                for case in cases {
                    for label in &case.labels {
                        let label_str = self.expression_to_c(&label.start);
                        self.emit_line(&format!("case {}:", label_str));
                    }
                    self.indent_level += 1;
                    self.generate_statement_sequence(&case.body);
                    self.emit_line("break;");
                    self.indent_level -= 1;
                }

                if let Some(else_stmts) = else_body {
                    self.emit_line("default:");
                    self.indent_level += 1;
                    self.generate_statement_sequence(else_stmts);
                    self.emit_line("break;");
                    self.indent_level -= 1;
                }

                self.indent_level -= 1;
                self.emit_line("}");
            }
            Statement::While { condition, body, elsif_parts } => {
                if elsif_parts.is_empty() {
                    let cond_str = self.expression_to_c(condition);
                    self.emit_line(&format!("while ({}) {{", cond_str));
                    self.indent_level += 1;
                    self.generate_statement_sequence(body);
                    self.indent_level -= 1;
                    self.emit_line("}");
                } else {
                    self.emit_line("while (1) {");
                    self.indent_level += 1;

                    let cond_str = self.expression_to_c(condition);
                    self.emit_line(&format!("if ({}) {{", cond_str));
                    self.indent_level += 1;
                    self.generate_statement_sequence(body);
                    self.indent_level -= 1;
                    self.emit_line("}");

                    for (elsif_cond, elsif_body) in elsif_parts {
                        let elsif_str = self.expression_to_c(elsif_cond);
                        self.emit_line(&format!("else if ({}) {{", elsif_str));
                        self.indent_level += 1;
                        self.generate_statement_sequence(elsif_body);
                        self.indent_level -= 1;
                        self.emit_line("}");
                    }

                    self.emit_line("else { break; }");
                    self.indent_level -= 1;
                    self.emit_line("}");
                }
            }
            Statement::Repeat { body, condition } => {
                self.emit_line("do {");
                self.indent_level += 1;
                self.generate_statement_sequence(body);
                self.indent_level -= 1;
                let cond_str = self.expression_to_c(condition);
                self.emit_line(&format!("}} while (!({}) );", cond_str));
            }
            Statement::For { variable, start, end, step, body } => {
                let var_name = self.mangle_name(variable);
                let start_str = self.expression_to_c(start);
                let end_str = self.expression_to_c(end);

                let step_str = if let Some(step_expr) = step {
                    self.expression_to_c(step_expr)
                } else {
                    "1".to_string()
                };

                self.emit_line(&format!(
                    "for ({} = {}; {} <= {}; {} += {}) {{",
                    var_name, start_str, var_name, end_str, var_name, step_str
                ));
                self.indent_level += 1;
                self.generate_statement_sequence(body);
                self.indent_level -= 1;
                self.emit_line("}");
            }
        }
    }

    // ========================================================================
    // Ausdrücke
    // ========================================================================

    fn expression_to_c(&self, expr: &Expression) -> String {
        match expr {
            Expression::IntegerLiteral(val) => format!("{}LL", val),
            Expression::RealLiteral(val) => format!("{}", val),
            Expression::StringLiteral(val) => format!("\"{}\"", val),
            Expression::BooleanLiteral(val) => format!("{}", val),
            Expression::Nil => "NULL".to_string(),
            Expression::Set(_) => "0".to_string(),
            Expression::Designator(designator) => self.designator_to_c(designator),
            Expression::FunctionCall { designator, arguments } => {
                let func_name = self.designator_to_c(designator);
                let args: Vec<String> = arguments.iter()
                    .map(|arg| self.expression_to_c(arg))
                    .collect();
                format!("{}({})", func_name, args.join(", "))
            }
            Expression::Unary { op, expr } => {
                let expr_str = self.expression_to_c(expr);
                match op {
                    UnaryOp::Plus => format!("+{}", expr_str),
                    UnaryOp::Minus => format!("-{}", expr_str),
                    UnaryOp::Not => format!("!{}", expr_str),
                }
            }
            Expression::Binary { left, op, right } => {
                let left_str = self.expression_to_c(left);
                let right_str = self.expression_to_c(right);
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::IntDiv => "/",
                    BinaryOp::Mod => "%",
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                    BinaryOp::Equal => "==",
                    BinaryOp::NotEqual => "!=",
                    BinaryOp::Less => "<",
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::Greater => ">",
                    BinaryOp::GreaterEqual => ">=",
                    BinaryOp::In => "&",
                    BinaryOp::Is => "==",
                };
                format!("({} {} {})", left_str, op_str, right_str)
            }
        }
    }

    fn designator_to_c(&self, designator: &Designator) -> String {
        let mut result = self.mangle_name(&designator.base.name);

        for selector in &designator.selectors {
            match selector {
                Selector::Field(name) => {
                    result.push('.');
                    result.push_str(&self.mangle_name(name));
                }
                Selector::Index(indices) => {
                    for index in indices {
                        result.push('[');
                        result.push_str(&self.expression_to_c(index));
                        result.push(']');
                    }
                }
                Selector::Dereference => {
                    result = format!("(*{})", result);
                }
                Selector::TypeGuard(_) => {}
            }
        }

        result
    }

    // ========================================================================
    // Hilfsfunktionen
    // ========================================================================

    fn mangle_name(&self, name: &str) -> String {
        format!("oberon_{}", name)
    }

    fn emit_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            write!(self.output, "    ").unwrap();
        }
        writeln!(self.output, "{}", line).unwrap();
    }

    #[allow(dead_code)]
    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    #[allow(dead_code)]
    fn new_temp(&mut self) -> String {
        let temp = format!("_t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }
}

impl Default for CGenerator {
    fn default() -> Self {
        Self::new()
    }
}