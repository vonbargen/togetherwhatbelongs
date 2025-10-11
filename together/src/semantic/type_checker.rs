use crate::parser::ast::*;
use super::symbol_table::*;
use std::collections::HashMap;

pub struct TypeChecker {
    symbol_table: SymbolTable,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut checker = TypeChecker {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
        };
        
        // Built-in Prozeduren registrieren
        checker.register_builtin_procedures();
        
        checker
    }
    
    fn register_builtin_procedures(&mut self) {
        // WriteLn: keine Parameter
        self.symbol_table.define(Symbol {
            name: "WriteLn".to_string(),
            kind: SymbolKind::Procedure {
                params: Vec::new(),
                return_type: None,
            },
            exported: ExportMark::ReadOnly,
            defined_at: None,
        }).ok();
        
        // WriteInt: ein INTEGER-Parameter
        self.symbol_table.define(Symbol {
            name: "WriteInt".to_string(),
            kind: SymbolKind::Procedure {
                params: vec![Parameter {
                    name: "n".to_string(),
                    param_type: ResolvedType::Integer,
                    is_var: false,
                }],
                return_type: None,
            },
            exported: ExportMark::ReadOnly,
            defined_at: None,
        }).ok();
    }

    pub fn check_module(&mut self, module: &Module) -> Result<(), Vec<String>> {
        // Module-Symbol hinzufügen
        self.symbol_table.define(Symbol {
            name: module.name.clone(),
            kind: SymbolKind::Module,
            exported: ExportMark::None,
            defined_at: None,
        }).ok();

        // Imports (vereinfacht - keine echte Modul-Auflösung)
        for import in &module.imports {
            self.symbol_table.define(Symbol {
                name: import.alias.clone().unwrap_or(import.module_name.clone()),
                kind: SymbolKind::Module,
                exported: ExportMark::None,
                defined_at: None,
            }).ok();
        }

        // Deklarationen prüfen
        self.check_declarations(&module.declarations)?;

        // Body prüfen
        if let Some(body) = &module.body {
            self.check_statement_sequence(body)?;
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    // ========================================================================
    // Konstanten-Auswertung
    // ========================================================================

    fn eval_const_expression(&self, expr: &Expression) -> Option<i64> {
        match expr {
            Expression::IntegerLiteral(val) => Some(*val),
            Expression::Designator(designator) => {
                // Konstante nachschlagen
                if designator.selectors.is_empty() {
                    if let Some(symbol) = self.symbol_table.lookup(&designator.base.name) {
                        if let SymbolKind::Constant { value, .. } = &symbol.kind {
                            return self.eval_const_expression(value);
                        }
                    }
                }
                None
            }
            Expression::Unary { op, expr } => {
                let val = self.eval_const_expression(expr)?;
                match op {
                    UnaryOp::Plus => Some(val),
                    UnaryOp::Minus => Some(-val),
                    UnaryOp::Not => None,
                }
            }
            Expression::Binary { left, op, right } => {
                let left_val = self.eval_const_expression(left)?;
                let right_val = self.eval_const_expression(right)?;
                match op {
                    BinaryOp::Add => Some(left_val + right_val),
                    BinaryOp::Sub => Some(left_val - right_val),
                    BinaryOp::Mul => Some(left_val * right_val),
                    BinaryOp::IntDiv => Some(left_val / right_val),
                    BinaryOp::Mod => Some(left_val % right_val),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    // ========================================================================
    // Deklarationen
    // ========================================================================

    fn check_declarations(&mut self, decls: &DeclSequence) -> Result<(), Vec<String>> {
        // Constants
        for const_decl in &decls.constants {
            let expr_type = self.infer_expression_type(&const_decl.value)?;

            self.symbol_table.define(Symbol {
                name: const_decl.name.name.clone(),
                kind: SymbolKind::Constant {
                    value: const_decl.value.clone(),
                    resolved_type: expr_type,
                },
                exported: const_decl.name.exported.clone(),
                defined_at: None,
            }).map_err(|e| {
                self.errors.push(e.clone());
                vec![e]
            })?;
        }

        // Types
        for type_decl in &decls.types {
            let resolved_type = self.resolve_type(&type_decl.type_def)?;

            self.symbol_table.define(Symbol {
                name: type_decl.name.name.clone(),
                kind: SymbolKind::Type {
                    type_def: resolved_type,
                },
                exported: type_decl.name.exported.clone(),
                defined_at: None,
            }).map_err(|e| {
                self.errors.push(e.clone());
                vec![e]
            })?;
        }

        // Variables
        for var_decl in &decls.variables {
            let resolved_type = self.resolve_type(&var_decl.var_type)?;

            for name in &var_decl.names {
                self.symbol_table.define(Symbol {
                    name: name.name.clone(),
                    kind: SymbolKind::Variable {
                        var_type: resolved_type.clone(),
                        is_parameter: false,
                        is_var_param: false,
                    },
                    exported: name.exported.clone(),
                    defined_at: None,
                }).map_err(|e| {
                    self.errors.push(e.clone());
                    vec![e]
                })?;
            }
        }

        // Procedures - NICHT MEHR Built-ins überschreiben!
        for proc_decl in &decls.procedures {
            // Built-ins komplett überspringen - sie sind bereits registriert
            let is_builtin = matches!(proc_decl.name.name.as_str(), "WriteInt" | "WriteLn");
            if !is_builtin {
                self.check_procedure(proc_decl)?;
            }
        }

        Ok(())
    }

    fn check_procedure(&mut self, proc: &ProcedureDeclaration) -> Result<(), Vec<String>> {
        // Prüfen, ob bereits als Built-in registriert
        let is_builtin = matches!(proc.name.name.as_str(), "WriteInt" | "WriteLn");

        if is_builtin {
            // Built-in-Funktionen: Body komplett überspringen, da sie im Code-Generator implementiert werden
            return Ok(());
        }

        let mut params = Vec::new();
        let mut return_type = None;

        if let Some(formal_params) = &proc.params {
            for section in &formal_params.sections {
                let param_type = self.resolve_type(&section.param_type)?;
                for param_name in &section.names {
                    params.push(Parameter {
                        name: param_name.clone(),
                        param_type: param_type.clone(),
                        is_var: section.is_var,
                    });
                }
            }

            if let Some(ret_type) = &formal_params.return_type {
                return_type = Some(self.resolve_qualident_type(ret_type)?);
            }
        }

        // Prozedur-Symbol definieren
        self.symbol_table.define(Symbol {
            name: proc.name.name.clone(),
            kind: SymbolKind::Procedure {
                params: params.clone(),
                return_type: return_type.clone(),
            },
            exported: proc.name.exported.clone(),
            defined_at: None,
        }).map_err(|e| {
            self.errors.push(e.clone());
            vec![e]
        })?;

        if proc.is_forward {
            return Ok(());
        }

        // Neuer Scope für Prozedur-Body
        self.symbol_table.enter_scope();
        self.symbol_table.set_current_procedure(Some(proc.name.name.clone()));

        // Parameter als lokale Variablen hinzufügen
        for param in &params {
            self.symbol_table.define(Symbol {
                name: param.name.clone(),
                kind: SymbolKind::Variable {
                    var_type: param.param_type.clone(),
                    is_parameter: true,
                    is_var_param: param.is_var,
                },
                exported: ExportMark::None,
                defined_at: None,
            }).ok();
        }

        // Lokale Deklarationen prüfen
        self.check_declarations(&proc.declarations)?;

        // Body prüfen
        if let Some(body) = &proc.body {
            self.check_statement_sequence(body)?;
        }

        // Return-Ausdruck prüfen
        if let Some(ret_expr) = &proc.return_expr {
            let expr_type = self.infer_expression_type(ret_expr)?;

            if let Some(expected_type) = &return_type {
                if !expr_type.is_assignable_to(expected_type) {
                    let err = format!(
                        "RETURN-Typ {:?} passt nicht zu deklariertem Typ {:?} in Prozedur '{}'",
                        expr_type, expected_type, proc.name.name
                    );
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }
            } else {
                let err = format!(
                    "Prozedur '{}' hat keinen Return-Typ, aber RETURN-Statement",
                    proc.name.name
                );
                self.errors.push(err.clone());
                return Err(vec![err]);
            }
        } else if return_type.is_some() {
            let err = format!(
                "Prozedur '{}' muss einen Wert zurückgeben",
                proc.name.name
            );
            self.errors.push(err.clone());
            return Err(vec![err]);
        }

        self.symbol_table.set_current_procedure(None);
        self.symbol_table.exit_scope();

        Ok(())
    }

    // ========================================================================
    // Typ-Auflösung
    // ========================================================================

    fn resolve_type(&mut self, type_def: &Type) -> Result<ResolvedType, Vec<String>> {
        match type_def {
            Type::Qualident(qualident) => self.resolve_qualident_type(qualident),
            Type::Array { lengths, element_type } => {
                let mut dims = Vec::new();
                for length_expr in lengths {
                    // Konstanten-Auswertung
                    if let Some(val) = self.eval_const_expression(length_expr) {
                        dims.push(val as usize);
                    } else {
                        let err = format!("Array-Länge muss ein konstanter Ausdruck sein: {:?}", length_expr);
                        self.errors.push(err.clone());
                        return Err(vec![err]);
                    }
                }
                let elem_type = Box::new(self.resolve_type(element_type)?);
                Ok(ResolvedType::Array {
                    dimensions: dims,
                    element_type: elem_type,
                })
            }
            Type::Record { base_type, fields } => {
                let mut field_map = HashMap::new();
                for field_list in fields {
                    let field_type = self.resolve_type(&field_list.field_type)?;
                    for name in &field_list.names {
                        if field_map.contains_key(&name.name) {
                            let err = format!("Feld '{}' bereits definiert", name.name);
                            self.errors.push(err.clone());
                            return Err(vec![err]);
                        }
                        field_map.insert(name.name.clone(), field_type.clone());
                    }
                }

                let base = if let Some(base_qualident) = base_type {
                    Some(Box::new(self.resolve_qualident_type(base_qualident)?))
                } else {
                    None
                };

                Ok(ResolvedType::Record {
                    fields: field_map,
                    base_type: base,
                })
            }
            Type::Pointer { target_type } => {
                let target = Box::new(self.resolve_type(target_type)?);
                Ok(ResolvedType::Pointer { target_type: target })
            }
            Type::Procedure { params } => {
                if let Some(formal_params) = params {
                    let mut proc_params = Vec::new();
                    for section in &formal_params.sections {
                        let param_type = self.resolve_type(&section.param_type)?;
                        for param_name in &section.names {
                            proc_params.push(Parameter {
                                name: param_name.clone(),
                                param_type: param_type.clone(),
                                is_var: section.is_var,
                            });
                        }
                    }

                    let return_type = if let Some(ret_type) = &formal_params.return_type {
                        Some(Box::new(self.resolve_qualident_type(ret_type)?))
                    } else {
                        None
                    };

                    Ok(ResolvedType::Procedure {
                        params: proc_params,
                        return_type,
                    })
                } else {
                    Ok(ResolvedType::Procedure {
                        params: Vec::new(),
                        return_type: None,
                    })
                }
            }
        }
    }

    fn resolve_qualident_type(&self, qualident: &Qualident) -> Result<ResolvedType, Vec<String>> {
        if qualident.module.is_some() {
            // Module-qualifizierte Typen (vereinfacht)
            return Ok(ResolvedType::Named(format!("{}", qualident)));
        }

        if let Some(symbol) = self.symbol_table.lookup(&qualident.name) {
            match &symbol.kind {
                SymbolKind::Type { type_def } => Ok(type_def.clone()),
                _ => Err(vec![format!("'{}' ist kein Typ", qualident.name)]),
            }
        } else {
            Err(vec![format!("Unbekannter Typ: {}", qualident.name)])
        }
    }

    // ========================================================================
    // Statements (unchanged, keeping existing implementation)
    // ========================================================================

    fn check_statement_sequence(&mut self, statements: &[Statement]) -> Result<(), Vec<String>> {
        for stmt in statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), Vec<String>> {
        match stmt {
            Statement::Empty => Ok(()),
            Statement::Assignment { target, value } => {
                let target_type = self.infer_designator_type(target)?;
                let value_type = self.infer_expression_type(value)?;

                if !value_type.is_assignable_to(&target_type) {
                    let err = format!(
                        "Typ-Fehler bei Zuweisung: {:?} kann nicht zu {:?} zugewiesen werden",
                        value_type, target_type
                    );
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }
                Ok(())
            }
            Statement::ProcedureCall { designator, arguments } => {
                let proc_type = self.infer_designator_type(designator)?;

                if let ResolvedType::Procedure { params, .. } = proc_type {
                    if arguments.len() != params.len() {
                        let err = format!(
                            "Falsche Anzahl an Argumenten: erwartet {}, gefunden {}",
                            params.len(),
                            arguments.len()
                        );
                        self.errors.push(err.clone());
                        return Err(vec![err]);
                    }

                    for (arg, param) in arguments.iter().zip(params.iter()) {
                        let arg_type = self.infer_expression_type(arg)?;
                        if !arg_type.is_assignable_to(&param.param_type) {
                            let err = format!(
                                "Argument-Typ {:?} passt nicht zu Parameter-Typ {:?}",
                                arg_type, param.param_type
                            );
                            self.errors.push(err.clone());
                            return Err(vec![err]);
                        }
                    }
                }
                Ok(())
            }
            Statement::If { condition, then_body, elsif_parts, else_body } => {
                let cond_type = self.infer_expression_type(condition)?;
                if !cond_type.is_boolean() {
                    let err = format!("IF-Bedingung muss BOOLEAN sein, ist {:?}", cond_type);
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }

                self.check_statement_sequence(then_body)?;

                for (elsif_cond, elsif_body) in elsif_parts {
                    let elsif_type = self.infer_expression_type(elsif_cond)?;
                    if !elsif_type.is_boolean() {
                        let err = format!("ELSIF-Bedingung muss BOOLEAN sein, ist {:?}", elsif_type);
                        self.errors.push(err.clone());
                        return Err(vec![err]);
                    }
                    self.check_statement_sequence(elsif_body)?;
                }

                if let Some(else_stmts) = else_body {
                    self.check_statement_sequence(else_stmts)?;
                }
                Ok(())
            }
            Statement::Case { expr, cases, else_body } => {
                let expr_type = self.infer_expression_type(expr)?;
                if !expr_type.is_integer() && !matches!(expr_type, ResolvedType::Char) {
                    let err = format!("CASE-Ausdruck muss INTEGER oder CHAR sein, ist {:?}", expr_type);
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }

                for case in cases {
                    for label in &case.labels {
                        let label_type = self.infer_expression_type(&label.start)?;
                        if !label_type.is_assignable_to(&expr_type) {
                            let err = format!("CASE-Label-Typ {:?} passt nicht zu {:?}", label_type, expr_type);
                            self.errors.push(err.clone());
                            return Err(vec![err]);
                        }
                    }
                    self.check_statement_sequence(&case.body)?;
                }

                if let Some(else_stmts) = else_body {
                    self.check_statement_sequence(else_stmts)?;
                }
                Ok(())
            }
            Statement::While { condition, body, elsif_parts } => {
                let cond_type = self.infer_expression_type(condition)?;
                if !cond_type.is_boolean() {
                    let err = format!("WHILE-Bedingung muss BOOLEAN sein, ist {:?}", cond_type);
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }

                self.check_statement_sequence(body)?;

                for (elsif_cond, elsif_body) in elsif_parts {
                    let elsif_type = self.infer_expression_type(elsif_cond)?;
                    if !elsif_type.is_boolean() {
                        let err = format!("ELSIF-Bedingung muss BOOLEAN sein, ist {:?}", elsif_type);
                        self.errors.push(err.clone());
                        return Err(vec![err]);
                    }
                    self.check_statement_sequence(elsif_body)?;
                }
                Ok(())
            }
            Statement::Repeat { body, condition } => {
                self.check_statement_sequence(body)?;
                let cond_type = self.infer_expression_type(condition)?;
                if !cond_type.is_boolean() {
                    let err = format!("REPEAT-Bedingung muss BOOLEAN sein, ist {:?}", cond_type);
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }
                Ok(())
            }
            Statement::For { variable, start, end, step, body } => {
                if let Some(symbol) = self.symbol_table.lookup(variable) {
                    if let SymbolKind::Variable { var_type, .. } = &symbol.kind {
                        if !var_type.is_integer() {
                            let err = "FOR-Variable muss INTEGER sein".to_string();
                            self.errors.push(err.clone());
                            return Err(vec![err]);
                        }
                    } else {
                        let err = format!("'{}' ist keine Variable", variable);
                        self.errors.push(err.clone());
                        return Err(vec![err]);
                    }
                } else {
                    let err = format!("Unbekannte Variable: {}", variable);
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }

                let start_type = self.infer_expression_type(start)?;
                let end_type = self.infer_expression_type(end)?;

                if !start_type.is_integer() || !end_type.is_integer() {
                    let err = "FOR-Grenzen müssen INTEGER sein".to_string();
                    self.errors.push(err.clone());
                    return Err(vec![err]);
                }

                if let Some(step_expr) = step {
                    let step_type = self.infer_expression_type(step_expr)?;
                    if !step_type.is_integer() {
                        let err = "FOR-Schritt muss INTEGER sein".to_string();
                        self.errors.push(err.clone());
                        return Err(vec![err]);
                    }
                }

                self.check_statement_sequence(body)?;
                Ok(())
            }
        }
    }

    // ========================================================================
    // Ausdrücke
    // ========================================================================

    fn infer_expression_type(&self, expr: &Expression) -> Result<ResolvedType, Vec<String>> {
        match expr {
            Expression::IntegerLiteral(_) => Ok(ResolvedType::Integer),
            Expression::RealLiteral(_) => Ok(ResolvedType::Real),
            Expression::StringLiteral(_) => Ok(ResolvedType::String),
            Expression::BooleanLiteral(_) => Ok(ResolvedType::Boolean),
            Expression::Nil => Ok(ResolvedType::Nil),
            Expression::Set(_) => Ok(ResolvedType::Set),
            Expression::Designator(designator) => self.infer_designator_type(designator),
            Expression::FunctionCall { designator, arguments: _ } => {
                let func_type = self.infer_designator_type(designator)?;
                if let ResolvedType::Procedure { return_type, .. } = func_type {
                    return_type.map(|t| *t).ok_or_else(|| {
                        vec!["Prozedur hat keinen Rückgabewert".to_string()]
                    })
                } else {
                    Err(vec![format!("'{:?}' ist keine Prozedur", designator)])
                }
            }
            Expression::Unary { op, expr } => {
                let expr_type = self.infer_expression_type(expr)?;
                match op {
                    UnaryOp::Plus | UnaryOp::Minus => {
                        if expr_type.is_numeric() {
                            Ok(expr_type)
                        } else {
                            Err(vec![format!("Unärer Operator +/- erfordert numerischen Typ, ist {:?}", expr_type)])
                        }
                    }
                    UnaryOp::Not => {
                        if expr_type.is_boolean() {
                            Ok(ResolvedType::Boolean)
                        } else {
                            Err(vec![format!("NOT erfordert BOOLEAN, ist {:?}", expr_type)])
                        }
                    }
                }
            }
            Expression::Binary { left, op, right } => {
                let left_type = self.infer_expression_type(left)?;
                let right_type = self.infer_expression_type(right)?;

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                        if left_type.is_numeric() && right_type.is_numeric() {
                            if matches!(left_type, ResolvedType::Real) || matches!(right_type, ResolvedType::Real) {
                                Ok(ResolvedType::Real)
                            } else {
                                Ok(ResolvedType::Integer)
                            }
                        } else {
                            Err(vec![format!("Arithmetische Operation erfordert numerische Typen: {:?} und {:?}", left_type, right_type)])
                        }
                    }
                    BinaryOp::Div => {
                        if left_type.is_numeric() && right_type.is_numeric() {
                            Ok(ResolvedType::Real)
                        } else {
                            Err(vec!["Division erfordert numerische Typen".to_string()])
                        }
                    }
                    BinaryOp::IntDiv | BinaryOp::Mod => {
                        if left_type.is_integer() && right_type.is_integer() {
                            Ok(ResolvedType::Integer)
                        } else {
                            Err(vec!["DIV/MOD erfordert INTEGER-Typen".to_string()])
                        }
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type.is_boolean() && right_type.is_boolean() {
                            Ok(ResolvedType::Boolean)
                        } else {
                            Err(vec!["Logische Operation erfordert BOOLEAN-Typen".to_string()])
                        }
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual => {
                        if left_type.is_comparable() && right_type.is_comparable() {
                            Ok(ResolvedType::Boolean)
                        } else {
                            Err(vec![format!("Vergleich nicht möglich für Typen {:?} und {:?}", left_type, right_type)])
                        }
                    }
                    BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                        if (left_type.is_numeric() && right_type.is_numeric())
                            || (matches!(left_type, ResolvedType::Char) && matches!(right_type, ResolvedType::Char))
                        {
                            Ok(ResolvedType::Boolean)
                        } else {
                            Err(vec![format!("Vergleichsoperator nicht anwendbar auf {:?} und {:?}", left_type, right_type)])
                        }
                    }
                    BinaryOp::In => {
                        if left_type.is_integer() && matches!(right_type, ResolvedType::Set) {
                            Ok(ResolvedType::Boolean)
                        } else {
                            Err(vec!["IN erfordert INTEGER und SET".to_string()])
                        }
                    }
                    BinaryOp::Is => {
                        Ok(ResolvedType::Boolean)
                    }
                }
            }
        }
    }

    fn infer_designator_type(&self, designator: &Designator) -> Result<ResolvedType, Vec<String>> {
        let base_name = &designator.base.name;

        let mut current_type = if let Some(symbol) = self.symbol_table.lookup(base_name) {
            match &symbol.kind {
                SymbolKind::Variable { var_type, .. } => var_type.clone(),
                SymbolKind::Constant { resolved_type, .. } => resolved_type.clone(),
                SymbolKind::Procedure { params, return_type } => {
                    ResolvedType::Procedure {
                        params: params.clone(),
                        return_type: return_type.clone().map(Box::new),
                    }
                }
                SymbolKind::Type { type_def } => type_def.clone(),
                SymbolKind::Module => ResolvedType::Named(base_name.clone()),
            }
        } else {
            return Err(vec![format!("Unbekannter Bezeichner: {}", base_name)]);
        };

        // Selektoren anwenden
        for selector in &designator.selectors {
            current_type = match selector {
                Selector::Field(field_name) => {
                    if let ResolvedType::Record { fields, .. } = current_type {
                        fields.get(field_name).cloned().ok_or_else(|| {
                            vec![format!("Unbekanntes Feld: {}", field_name)]
                        })?
                    } else {
                        return Err(vec![format!("Feld-Zugriff auf Nicht-Record-Typ: {:?}", current_type)]);
                    }
                }
                Selector::Index(_) => {
                    if let ResolvedType::Array { element_type, .. } = current_type {
                        *element_type
                    } else {
                        return Err(vec![format!("Index-Zugriff auf Nicht-Array-Typ: {:?}", current_type)]);
                    }
                }
                Selector::Dereference => {
                    if let ResolvedType::Pointer { target_type } = current_type {
                        *target_type
                    } else {
                        return Err(vec![format!("Dereferenzierung auf Nicht-Pointer-Typ: {:?}", current_type)]);
                    }
                }
                Selector::TypeGuard(_) => {
                    current_type
                }
            };
        }

        Ok(current_type)
    }

    #[allow(dead_code)]
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}