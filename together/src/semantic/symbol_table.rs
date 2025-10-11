
use crate::parser::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Constant {
        value: Expression,
        resolved_type: ResolvedType,
    },
    Type {
        type_def: ResolvedType,
    },
    Variable {
        var_type: ResolvedType,
        is_parameter: bool,
        is_var_param: bool,
    },
    Procedure {
        params: Vec<Parameter>,
        return_type: Option<ResolvedType>,
    },
    Module,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: ResolvedType,
    pub is_var: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    Integer,
    Real,
    Boolean,
    Char,
    String,
    Nil,
    Set,
    Array {
        dimensions: Vec<usize>,
        element_type: Box<ResolvedType>,
    },
    Record {
        fields: HashMap<String, ResolvedType>,
        base_type: Option<Box<ResolvedType>>,
    },
    Pointer {
        target_type: Box<ResolvedType>,
    },
    Procedure {
        params: Vec<Parameter>,
        return_type: Option<Box<ResolvedType>>,
    },
    Named(String),
    Unresolved,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub exported: ExportMark,
    pub defined_at: Option<(usize, usize)>, // line, column
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    current_procedure: Option<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable {
            scopes: vec![HashMap::new()],
            current_procedure: None,
        };
        table.add_predefined_types();
        table
    }

    fn add_predefined_types(&mut self) {
        // Grundtypen
        self.define(Symbol {
            name: "INTEGER".to_string(),
            kind: SymbolKind::Type {
                type_def: ResolvedType::Integer,
            },
            exported: ExportMark::None,
            defined_at: None,
        }).ok();

        self.define(Symbol {
            name: "REAL".to_string(),
            kind: SymbolKind::Type {
                type_def: ResolvedType::Real,
            },
            exported: ExportMark::None,
            defined_at: None,
        }).ok();

        self.define(Symbol {
            name: "BOOLEAN".to_string(),
            kind: SymbolKind::Type {
                type_def: ResolvedType::Boolean,
            },
            exported: ExportMark::None,
            defined_at: None,
        }).ok();

        self.define(Symbol {
            name: "CHAR".to_string(),
            kind: SymbolKind::Type {
                type_def: ResolvedType::Char,
            },
            exported: ExportMark::None,
            defined_at: None,
        }).ok();

        self.define(Symbol {
            name: "SET".to_string(),
            kind: SymbolKind::Type {
                type_def: ResolvedType::Set,
            },
            exported: ExportMark::None,
            defined_at: None,
        }).ok();
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        let scope = self.scopes.last_mut().unwrap();

        if scope.contains_key(&symbol.name) {
            return Err(format!("Symbol '{}' bereits definiert", symbol.name));
        }

        scope.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn lookup_in_current_scope(&self, name: &str) -> Option<&Symbol> {
        self.scopes.last().and_then(|scope| scope.get(name))
    }

    pub fn set_current_procedure(&mut self, name: Option<String>) {
        self.current_procedure = name;
    }

    pub fn get_current_procedure(&self) -> Option<&String> {
        self.current_procedure.as_ref()
    }
}

impl ResolvedType {
    pub fn is_numeric(&self) -> bool {
        matches!(self, ResolvedType::Integer | ResolvedType::Real)
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, ResolvedType::Integer)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, ResolvedType::Boolean)
    }

    pub fn is_comparable(&self) -> bool {
        matches!(
            self,
            ResolvedType::Integer
                | ResolvedType::Real
                | ResolvedType::Boolean
                | ResolvedType::Char
                | ResolvedType::String
        )
    }

    pub fn is_assignable_to(&self, other: &ResolvedType) -> bool {
        if self == other {
            return true;
        }

        // Integer kann zu Real konvertiert werden
        if matches!(self, ResolvedType::Integer) && matches!(other, ResolvedType::Real) {
            return true;
        }

        // NIL kann zu Pointer zugewiesen werden
        if matches!(self, ResolvedType::Nil) && matches!(other, ResolvedType::Pointer { .. }) {
            return true;
        }

        false
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}