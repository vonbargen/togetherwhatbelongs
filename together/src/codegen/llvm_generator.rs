use crate::parser::ast::*;
use crate::semantic::{ResolvedType, SymbolTable, Symbol, SymbolKind};
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module as LLVMModule;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate, FloatPredicate};
use std::collections::HashMap;

pub struct LLVMGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: LLVMModule<'ctx>,
    symbol_table: SymbolTable,
    variables: HashMap<String, PointerValue<'ctx>>,
    current_function: Option<FunctionValue<'ctx>>,
    type_table: HashMap<String, BasicTypeEnum<'ctx>>,
    variable_types: HashMap<String, BasicTypeEnum<'ctx>>,
}

impl<'ctx> LLVMGenerator<'ctx> {

    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        LLVMGenerator {
            context,
            builder: context.create_builder(),
            module: context.create_module(module_name),
            symbol_table: SymbolTable::new(),
            variables: HashMap::new(),
            current_function: None,
            type_table: HashMap::new(),
            variable_types: HashMap::new(),  // NEU  // NEU: Initialisierung
        }
    }

    pub fn generate(&mut self, module: &Module) -> Result<String, String> {
        // Externe Funktionen deklarieren (printf, puts)
        self.declare_external_functions();

        // Module-Symbol hinzufügen
        self.symbol_table.define(Symbol {
            name: module.name.clone(),
            kind: SymbolKind::Module,
            exported: ExportMark::None,
            defined_at: None,
        }).ok();

        // Konstanten registrieren
        for const_decl in &module.declarations.constants {
            self.symbol_table.define(Symbol {
                name: const_decl.name.name.clone(),
                kind: SymbolKind::Constant {
                    value: const_decl.value.clone(),
                    resolved_type: ResolvedType::Integer, // Vereinfacht
                },
                exported: const_decl.name.exported.clone(),
                defined_at: None,
            }).ok();
        }

        // Globale Typen deklarieren
        for type_decl in &module.declarations.types {
            self.declare_type(type_decl)?;
        }

        // Globale Variablen
        for var_decl in &module.declarations.variables {
            self.declare_global_variable(var_decl)?;
        }

        // Prozeduren deklarieren
        for proc_decl in &module.declarations.procedures {
            if !proc_decl.is_forward {
                self.declare_function(proc_decl)?;
            }
        }

        // Prozeduren implementieren
        for proc_decl in &module.declarations.procedures {
            if !proc_decl.is_forward {
                self.generate_function(proc_decl)?;
            }
        }

        // Main-Funktion generieren
        self.generate_main(module)?;

        // LLVM-IR als String zurückgeben
        Ok(self.module.print_to_string().to_string())
    }

    // ========================================================================
    // Typen
    // ========================================================================

    fn declare_type(&mut self, type_decl: &TypeDeclaration) -> Result<(), String> {
        // Typ generieren und in Tabelle speichern
        let llvm_type = self.resolve_llvm_type(&type_decl.type_def)?;
        self.type_table.insert(type_decl.name.name.clone(), llvm_type);
        Ok(())
    }

    // ========================================================================
    // Globale Variablen
    // ========================================================================

    fn resolve_llvm_type(&self, oberon_type: &Type) -> Result<BasicTypeEnum<'ctx>, String> {
        match oberon_type {
            Type::Qualident(q) => {
                match q.name.as_str() {
                    "INTEGER" => Ok(self.context.i64_type().into()),
                    "REAL" => Ok(self.context.f64_type().into()),
                    "BOOLEAN" => Ok(self.context.bool_type().into()),
                    "CHAR" => Ok(self.context.i8_type().into()),
                    _ => {
                        // Benutzerdefinierten Typ aus Tabelle suchen
                        self.type_table
                            .get(&q.name)
                            .copied()
                            .ok_or(format!("Unbekannter Typ: {}", q.name))
                    }
                }
            }
            Type::Array { lengths, element_type } => {
                let elem_type = self.resolve_llvm_type(element_type)?;
                let mut array_type = elem_type;

                // Arrays von rechts nach links aufbauen
                for length_expr in lengths.iter().rev() {
                    let size = if let Expression::IntegerLiteral(val) = length_expr {
                        *val as u32
                    } else {
                        100 // Fallback
                    };
                    array_type = array_type.array_type(size).into();
                }

                Ok(array_type)
            }
            Type::Record { fields, .. } => {
                let mut field_types = Vec::new();
                for field_list in fields {
                    let field_type = self.resolve_llvm_type(&field_list.field_type)?;
                    for _ in &field_list.names {
                        field_types.push(field_type);
                    }
                }
                Ok(self.context.struct_type(&field_types, false).into())
            }
            Type::Pointer { target_type } => {
                let target = self.resolve_llvm_type(target_type)?;
                Ok(target.ptr_type(AddressSpace::default()).into())
            }
            _ => Err("Nicht unterstützter Typ".to_string()),
        }
    }

    // ========================================================================
    // Externe Funktionen
    // ========================================================================

    fn declare_external_functions(&mut self) {
        // printf deklarieren: i32 printf(i8*, ...)
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let printf_type = self.context.i32_type().fn_type(&[i8_ptr_type.into()], true); // varargs
        self.module.add_function("printf", printf_type, None);

        // puts deklarieren: i32 puts(i8*)
        let puts_type = self.context.i32_type().fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("puts", puts_type, None);
    }

    // ========================================================================
    // Globale Variablen
    // ========================================================================

    fn declare_global_variable(&mut self, var_decl: &VariableDeclaration) -> Result<(), String> {
        let var_type = self.resolve_llvm_type(&var_decl.var_type)?;

        for name in &var_decl.names {
            let global = self.module.add_global(
                var_type,
                Some(AddressSpace::default()),
                &self.mangle_name(&name.name),
            );

            // Initialisierung mit Null
            global.set_initializer(&var_type.const_zero());

            self.variables.insert(name.name.clone(), global.as_pointer_value());
            self.variable_types.insert(name.name.clone(), var_type);  // NEU: Typ speichern
        }

        Ok(())
    }

    // ========================================================================
    // Funktionen
    // ========================================================================

    fn declare_function(&mut self, proc: &ProcedureDeclaration) -> Result<(), String> {
        let return_type = if let Some(params) = &proc.params {
            if let Some(ret_type) = &params.return_type {
                Some(self.resolve_llvm_type(&Type::Qualident(ret_type.clone()))?)
            } else {
                None
            }
        } else {
            None
        };

        let mut param_types = Vec::new();
        if let Some(params) = &proc.params {
            for section in &params.sections {
                let param_type = self.resolve_llvm_type(&section.param_type)?;
                for _ in &section.names {
                    if section.is_var {
                        // VAR-Parameter als Pointer
                        param_types.push(param_type.ptr_type(AddressSpace::default()).into());
                    } else {
                        param_types.push(param_type.into());
                    }
                }
            }
        }

        let fn_type = if let Some(ret) = return_type {
            ret.fn_type(&param_types, false)
        } else {
            self.context.void_type().fn_type(&param_types, false)
        };

        self.module.add_function(
            &self.mangle_name(&proc.name.name),
            fn_type,
            None,
        );

        Ok(())
    }

    fn generate_function(&mut self, proc: &ProcedureDeclaration) -> Result<(), String> {
        // Spezielle Built-in Funktionen
        if proc.name.name == "WriteInt" {
            return self.generate_write_int();
        }
        if proc.name.name == "WriteLn" {
            return self.generate_write_ln();
        }

        let function = self.module
            .get_function(&self.mangle_name(&proc.name.name))
            .ok_or("Funktion nicht gefunden")?;

        self.current_function = Some(function);

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Neuer Scope für lokale Variablen
        self.symbol_table.enter_scope();
        let old_vars = self.variables.clone();

        // Parameter als lokale Variablen
        if let Some(params) = &proc.params {
            let mut param_idx = 0;
            for section in &params.sections {
                let param_type = self.resolve_llvm_type(&section.param_type)?;
                for name in &section.names {
                    let param_value = function.get_nth_param(param_idx as u32)
                        .ok_or("Parameter nicht gefunden")?;

                    if section.is_var {
                        // VAR-Parameter ist bereits ein Pointer
                        self.variables.insert(name.clone(), param_value.into_pointer_value());
                        self.variable_types.insert(name.clone(), param_type);  // NEU: Typ speichern
                    } else {
                        // Normale Parameter: alloca + store
                        let alloca = self.builder.build_alloca(param_type, name).unwrap();
                        self.builder.build_store(alloca, param_value).unwrap();
                        self.variables.insert(name.clone(), alloca);
                        self.variable_types.insert(name.clone(), param_type);  // NEU: Typ speichern
                    }

                    param_idx += 1;
                }
            }
        }

        // Lokale Variablen
        for var_decl in &proc.declarations.variables {
            let var_type = self.resolve_llvm_type(&var_decl.var_type)?;
            for name in &var_decl.names {
                let alloca = self.builder.build_alloca(var_type, &name.name).unwrap();
                self.variables.insert(name.name.clone(), alloca);
                self.variable_types.insert(name.name.clone(), var_type);  // NEU: Typ speichern
            }
        }

        // Body
        if let Some(body) = &proc.body {
            self.generate_statement_sequence(body)?;
        }

        // Return
        if let Some(ret_expr) = &proc.return_expr {
            let ret_val = self.generate_expression(ret_expr)?;
            self.builder.build_return(Some(&ret_val)).unwrap();
        } else if proc.params.is_none() || proc.params.as_ref().unwrap().return_type.is_none() {
            self.builder.build_return(None).unwrap();
        }

        // Scope verlassen
        self.symbol_table.exit_scope();
        self.variables = old_vars;
        self.current_function = None;

        Ok(())
    }

    // ========================================================================
    // Main-Funktion
    // ========================================================================

    fn generate_main(&mut self, module: &Module) -> Result<(), String> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        if let Some(body) = &module.body {
            self.generate_statement_sequence(body)?;
        }

        self.builder.build_return(Some(&i32_type.const_zero())).unwrap();

        Ok(())
    }

    // ========================================================================
    // Statements
    // ========================================================================

    fn generate_statement_sequence(&mut self, statements: &[Statement]) -> Result<(), String> {
        for stmt in statements {
            self.generate_statement(stmt)?;
        }
        Ok(())
    }

    fn generate_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Empty => Ok(()),
            Statement::Assignment { target, value } => {
                let ptr = self.generate_designator_ptr(target)?;
                let val = self.generate_expression(value)?;
                self.builder.build_store(ptr, val).unwrap();
                Ok(())
            }
            Statement::ProcedureCall { designator, arguments } => {
                let func_name = self.mangle_name(&designator.base.name);
                let function = self.module
                    .get_function(&func_name)
                    .ok_or(format!("Funktion nicht gefunden: {}", func_name))?;

                let mut args: Vec<BasicMetadataValueEnum> = Vec::new();
                for arg in arguments {
                    let arg_val = self.generate_expression(arg)?;
                    args.push(arg_val.into());
                }

                self.builder.build_call(function, &args, "call").unwrap();
                Ok(())
            }
            Statement::If { condition, then_body, elsif_parts, else_body } => {
                let function = self.current_function.ok_or("Keine aktuelle Funktion")?;

                let cond_val = self.generate_expression(condition)?;
                let cond_bool = cond_val.into_int_value();

                let then_bb = self.context.append_basic_block(function, "then");
                let merge_bb = self.context.append_basic_block(function, "ifcont");
                let else_bb = if !elsif_parts.is_empty() || else_body.is_some() {
                    self.context.append_basic_block(function, "else")
                } else {
                    merge_bb
                };

                self.builder.build_conditional_branch(cond_bool, then_bb, else_bb).unwrap();

                // Then-Block
                self.builder.position_at_end(then_bb);
                self.generate_statement_sequence(then_body)?;
                self.builder.build_unconditional_branch(merge_bb).unwrap();

                // ELSIF/ELSE-Blocks (vereinfacht)
                if !elsif_parts.is_empty() || else_body.is_some() {
                    self.builder.position_at_end(else_bb);
                    if let Some(else_stmts) = else_body {
                        self.generate_statement_sequence(else_stmts)?;
                    }
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                }

                self.builder.position_at_end(merge_bb);
                Ok(())
            }
            Statement::While { condition, body, .. } => {
                let function = self.current_function.ok_or("Keine aktuelle Funktion")?;

                let cond_bb = self.context.append_basic_block(function, "whilecond");
                let body_bb = self.context.append_basic_block(function, "whilebody");
                let merge_bb = self.context.append_basic_block(function, "whilecont");

                self.builder.build_unconditional_branch(cond_bb).unwrap();

                // Condition
                self.builder.position_at_end(cond_bb);
                let cond_val = self.generate_expression(condition)?;
                let cond_bool = cond_val.into_int_value();
                self.builder.build_conditional_branch(cond_bool, body_bb, merge_bb).unwrap();

                // Body
                self.builder.position_at_end(body_bb);
                self.generate_statement_sequence(body)?;
                self.builder.build_unconditional_branch(cond_bb).unwrap();

                self.builder.position_at_end(merge_bb);
                Ok(())
            }
            Statement::For { variable, start, end, step, body } => {
                let function = self.current_function.ok_or("Keine aktuelle Funktion")?;

                // WICHTIG: Pointer KOPIEREN (dereferenzieren beim get)
                let var_ptr = *self.variables.get(variable)
                    .ok_or(format!("Variable nicht gefunden: {}", variable))?;

                // Initialisierung
                let start_val = self.generate_expression(start)?;
                self.builder.build_store(var_ptr, start_val).unwrap();

                let cond_bb = self.context.append_basic_block(function, "forcond");
                let body_bb = self.context.append_basic_block(function, "forbody");
                let incr_bb = self.context.append_basic_block(function, "forincr");
                let merge_bb = self.context.append_basic_block(function, "forcont");

                self.builder.build_unconditional_branch(cond_bb).unwrap();

                // Condition
                self.builder.position_at_end(cond_bb);
                let var_val = self.builder.build_load(self.context.i64_type(), var_ptr, variable).unwrap();
                let end_val = self.generate_expression(end)?;
                let cmp = self.builder.build_int_compare(
                    IntPredicate::SLE,
                    var_val.into_int_value(),
                    end_val.into_int_value(),
                    "forcmp"
                ).unwrap();
                self.builder.build_conditional_branch(cmp, body_bb, merge_bb).unwrap();

                // Body
                self.builder.position_at_end(body_bb);
                self.generate_statement_sequence(body)?;
                self.builder.build_unconditional_branch(incr_bb).unwrap();

                // Increment
                self.builder.position_at_end(incr_bb);
                let current = self.builder.build_load(self.context.i64_type(), var_ptr, "current").unwrap();
                let step_val = if let Some(s) = step {
                    self.generate_expression(s)?
                } else {
                    self.context.i64_type().const_int(1, false).into()
                };
                let next = self.builder.build_int_add(
                    current.into_int_value(),
                    step_val.into_int_value(),
                    "next"
                ).unwrap();
                self.builder.build_store(var_ptr, next).unwrap();
                self.builder.build_unconditional_branch(cond_bb).unwrap();

                self.builder.position_at_end(merge_bb);
                Ok(())
            }
            _ => Err("Statement nicht implementiert".to_string()),
        }
    }

    // ========================================================================
    // Ausdrücke
    // ========================================================================

    fn generate_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expression::IntegerLiteral(val) => {
                Ok(self.context.i64_type().const_int(*val as u64, false).into())
            }
            Expression::RealLiteral(val) => {
                Ok(self.context.f64_type().const_float(*val).into())
            }
            Expression::BooleanLiteral(val) => {
                Ok(self.context.bool_type().const_int(*val as u64, false).into())
            }
            Expression::Designator(designator) => {
                let ptr = self.generate_designator_ptr(designator)?;
                // Typ aus der Symbol-Tabelle oder aus dem Designator ableiten
                // Für INTEGER verwenden wir i64 als Standard
                let var_type = self.context.i64_type(); // Vereinfacht - sollte aus Type-Info kommen
                Ok(self.builder.build_load(var_type, ptr, "load").unwrap())
            }
            Expression::FunctionCall { designator, arguments } => {
                let func_name = self.mangle_name(&designator.base.name);
                let function = self.module
                    .get_function(&func_name)
                    .ok_or(format!("Funktion nicht gefunden: {}", func_name))?;

                let mut args: Vec<BasicMetadataValueEnum> = Vec::new();
                for arg in arguments {
                    let arg_val = self.generate_expression(arg)?;
                    args.push(arg_val.into());
                }

                let call = self.builder.build_call(function, &args, "call").unwrap();
                call.try_as_basic_value()
                    .left()
                    .ok_or("Funktion gibt keinen Wert zurück".to_string())
            }
            Expression::Binary { left, op, right } => {
                let lhs = self.generate_expression(left)?;
                let rhs = self.generate_expression(right)?;

                match op {
                    BinaryOp::Add => {
                        if lhs.is_int_value() {
                            Ok(self.builder.build_int_add(
                                lhs.into_int_value(),
                                rhs.into_int_value(),
                                "add"
                            ).unwrap().into())
                        } else {
                            Ok(self.builder.build_float_add(
                                lhs.into_float_value(),
                                rhs.into_float_value(),
                                "fadd"
                            ).unwrap().into())
                        }
                    }
                    BinaryOp::Sub => {
                        if lhs.is_int_value() {
                            Ok(self.builder.build_int_sub(
                                lhs.into_int_value(),
                                rhs.into_int_value(),
                                "sub"
                            ).unwrap().into())
                        } else {
                            Ok(self.builder.build_float_sub(
                                lhs.into_float_value(),
                                rhs.into_float_value(),
                                "fsub"
                            ).unwrap().into())
                        }
                    }
                    BinaryOp::Mul => {
                        if lhs.is_int_value() {
                            Ok(self.builder.build_int_mul(
                                lhs.into_int_value(),
                                rhs.into_int_value(),
                                "mul"
                            ).unwrap().into())
                        } else {
                            Ok(self.builder.build_float_mul(
                                lhs.into_float_value(),
                                rhs.into_float_value(),
                                "fmul"
                            ).unwrap().into())
                        }
                    }
                    BinaryOp::IntDiv => {
                        Ok(self.builder.build_int_signed_div(
                            lhs.into_int_value(),
                            rhs.into_int_value(),
                            "div"
                        ).unwrap().into())
                    }
                    BinaryOp::Equal => {
                        if lhs.is_int_value() {
                            Ok(self.builder.build_int_compare(
                                IntPredicate::EQ,
                                lhs.into_int_value(),
                                rhs.into_int_value(),
                                "eq"
                            ).unwrap().into())
                        } else {
                            Ok(self.builder.build_float_compare(
                                FloatPredicate::OEQ,
                                lhs.into_float_value(),
                                rhs.into_float_value(),
                                "feq"
                            ).unwrap().into())
                        }
                    }
                    BinaryOp::Less => {
                        if lhs.is_int_value() {
                            Ok(self.builder.build_int_compare(
                                IntPredicate::SLT,
                                lhs.into_int_value(),
                                rhs.into_int_value(),
                                "lt"
                            ).unwrap().into())
                        } else {
                            Ok(self.builder.build_float_compare(
                                FloatPredicate::OLT,
                                lhs.into_float_value(),
                                rhs.into_float_value(),
                                "flt"
                            ).unwrap().into())
                        }
                    }
                    BinaryOp::LessEqual => {
                        if lhs.is_int_value() {
                            Ok(self.builder.build_int_compare(
                                IntPredicate::SLE,
                                lhs.into_int_value(),
                                rhs.into_int_value(),
                                "le"
                            ).unwrap().into())
                        } else {
                            Ok(self.builder.build_float_compare(
                                FloatPredicate::OLE,
                                lhs.into_float_value(),
                                rhs.into_float_value(),
                                "fle"
                            ).unwrap().into())
                        }
                    }
                    _ => Err(format!("Operator {:?} nicht implementiert", op)),
                }
            }
            _ => Err("Ausdruck nicht implementiert".to_string()),
        }
    }

    fn generate_designator_ptr(&mut self, designator: &Designator) -> Result<PointerValue<'ctx>, String> {
        let base_name = &designator.base.name;
        let mut ptr = self.variables
            .get(base_name)
            .ok_or(format!("Variable nicht gefunden: {}", base_name))?
            .clone();
        
        // Typ der Variable holen
        let mut current_type = self.variable_types
            .get(base_name)
            .ok_or(format!("Typ für Variable nicht gefunden: {}", base_name))?
            .clone();

        for selector in &designator.selectors {
            match selector {
                Selector::Field(field_name) => {
                    // Für Struct-Felder: struct_gep verwenden
                    let field_idx = 0; // TODO: Richtigen Index aus Typ-Info ermitteln
                    
                    ptr = self.builder.build_struct_gep(
                        current_type,
                        ptr,
                        field_idx,
                        field_name
                    ).map_err(|e| format!("build_struct_gep Fehler: {:?}", e))?;
                    
                    // Typ aktualisieren (vereinfacht: nehmen wir an es ist ein Struct)
                    if let BasicTypeEnum::StructType(struct_type) = current_type {
                        if let Some(field_type) = struct_type.get_field_type_at_index(field_idx) {
                            current_type = field_type;
                        }
                    }
                }
                Selector::Index(indices) => {
                    for index_expr in indices {
                        let index = self.generate_expression(index_expr)?;
                        let zero = self.context.i64_type().const_zero();
                        
                        ptr = unsafe {
                            self.builder.build_gep(
                                current_type,
                                ptr,
                                &[zero, index.into_int_value()],
                                "arrayidx"
                            ).map_err(|e| format!("build_gep Fehler: {:?}", e))?
                        };
                        
                        // Typ aktualisieren (Element-Typ des Arrays)
                        if let BasicTypeEnum::ArrayType(array_type) = current_type {
                            current_type = array_type.get_element_type();
                        }
                    }
                }
                _ => return Err("Selector nicht implementiert".to_string()),
            }
        }

        Ok(ptr)
    }

    // ========================================================================
    // Built-in Funktionen
    // ========================================================================

    fn generate_write_int(&mut self) -> Result<(), String> {
        // Funktion aus Modul holen (wurde bereits in declare_function deklariert)
        let function = self.module
            .get_function("oberon_WriteInt")
            .ok_or("WriteInt nicht deklariert")?;

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // printf("%lld", n)
        let format_str = self.create_string_literal("%lld");
        let printf = self.module.get_function("printf").unwrap();
        let n = function.get_nth_param(0).unwrap();

        self.builder.build_call(
            printf,
            &[format_str.into(), n.into()],
            "printf_call"
        ).unwrap();

        self.builder.build_return(None).unwrap();
        Ok(())
    }

    fn generate_write_ln(&mut self) -> Result<(), String> {
        // Funktion aus Modul holen (wurde bereits in declare_function deklariert)
        let function = self.module
            .get_function("oberon_WriteLn")
            .ok_or("WriteLn nicht deklariert")?;

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // puts("")
        let newline = self.create_string_literal("");
        let puts = self.module.get_function("puts").unwrap();

        self.builder.build_call(
            puts,
            &[newline.into()],
            "puts_call"
        ).unwrap();

        self.builder.build_return(None).unwrap();
        Ok(())
    }

    fn create_string_literal(&self, text: &str) -> PointerValue<'ctx> {
        // String als globale Konstante erstellen
        let string_val = self.context.const_string(text.as_bytes(), true);
        let global = self.module.add_global(string_val.get_type(), None, ".str");
        global.set_linkage(inkwell::module::Linkage::Private);
        global.set_initializer(&string_val);
        global.set_constant(true);

        unsafe {
            self.builder.build_gep(
                string_val.get_type(),
                global.as_pointer_value(),
                &[self.context.i32_type().const_zero(), self.context.i32_type().const_zero()],
                "str_ptr"
            ).unwrap()
        }
    }

    // ========================================================================
    // Hilfsfunktionen
    // ========================================================================

    fn mangle_name(&self, name: &str) -> String {
        format!("oberon_{}", name)
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), String> {
        self.module
            .print_to_file(filename)
            .map_err(|e| e.to_string())
    }

}