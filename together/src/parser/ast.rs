use std::fmt;

// ============================================================================
// Module
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub imports: Vec<Import>,
    pub declarations: DeclSequence,
    pub body: Option<Vec<Statement>>,
    pub end_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub alias: Option<String>,
    pub module_name: String,
}

// ============================================================================
// Deklarationen
// ============================================================================

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeclSequence {
    pub constants: Vec<ConstDeclaration>,
    pub types: Vec<TypeDeclaration>,
    pub variables: Vec<VariableDeclaration>,
    pub procedures: Vec<ProcedureDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDeclaration {
    pub name: IdentDef,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration {
    pub name: IdentDef,
    pub type_def: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub names: Vec<IdentDef>,
    pub var_type: Type,
}

// ============================================================================
// Typen
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Qualident(Qualident),
    Array {
        lengths: Vec<Expression>,
        element_type: Box<Type>,
    },
    Record {
        base_type: Option<Qualident>,
        fields: Vec<FieldList>,
    },
    Pointer {
        target_type: Box<Type>,
    },
    Procedure {
        params: Option<FormalParameters>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldList {
    pub names: Vec<IdentDef>,
    pub field_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormalParameters {
    pub sections: Vec<FPSection>,
    pub return_type: Option<Qualident>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FPSection {
    pub is_var: bool,
    pub names: Vec<String>,
    pub param_type: Type,
}

// ============================================================================
// Prozeduren
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureDeclaration {
    pub name: IdentDef,
    pub params: Option<FormalParameters>,
    pub declarations: DeclSequence,
    pub body: Option<Vec<Statement>>,
    pub return_expr: Option<Expression>,
    pub end_name: String,
    pub is_forward: bool,
}

// ============================================================================
// Statements
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        target: Designator,
        value: Expression,
    },
    ProcedureCall {
        designator: Designator,
        arguments: Vec<Expression>,
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        elsif_parts: Vec<(Expression, Vec<Statement>)>,
        else_body: Option<Vec<Statement>>,
    },
    Case {
        expr: Expression,
        cases: Vec<CaseClause>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
        elsif_parts: Vec<(Expression, Vec<Statement>)>,
    },
    Repeat {
        body: Vec<Statement>,
        condition: Expression,
    },
    For {
        variable: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseClause {
    pub labels: Vec<CaseLabel>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseLabel {
    pub start: Expression,
    pub end: Option<Expression>,
}

// ============================================================================
// Ausdrücke
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    IntegerLiteral(i64),
    RealLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Nil,
    Set(Vec<SetElement>),
    Designator(Designator),
    FunctionCall {
        designator: Designator,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetElement {
    pub start: Box<Expression>,
    pub end: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetische Operatoren
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,

    // Logische Operatoren
    And,
    Or,

    // Vergleichsoperatoren
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    In,
    Is,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

// ============================================================================
// Designator
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Designator {
    pub base: Qualident,
    pub selectors: Vec<Selector>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Field(String),
    Index(Vec<Expression>),
    Dereference,
    TypeGuard(Qualident),
}

// ============================================================================
// Hilfsdefinitionen
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct IdentDef {
    pub name: String,
    pub exported: ExportMark,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportMark {
    None,
    ReadOnly,     // *
    ReadWrite,    // -
}

#[derive(Debug, Clone, PartialEq)]
pub struct Qualident {
    pub module: Option<String>,
    pub name: String,
}

// ============================================================================
// Display Implementierungen für bessere Ausgabe
// ============================================================================

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MODULE {}", self.name)
    }
}

impl fmt::Display for Qualident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref module) = self.module {
            write!(f, "{}.{}", module, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl fmt::Display for IdentDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        match self.exported {
            ExportMark::ReadOnly => write!(f, "*"),
            ExportMark::ReadWrite => write!(f, "-"),
            ExportMark::None => Ok(()),
        }
    }
}

impl Qualident {
    pub fn new(name: String) -> Self {
        Qualident {
            module: None,
            name,
        }
    }

    pub fn with_module(module: String, name: String) -> Self {
        Qualident {
            module: Some(module),
            name,
        }
    }
}

// ... existing code ...

impl IdentDef {
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        IdentDef {
            name,
            exported: ExportMark::None,
        }
    }

    #[allow(dead_code)]
    pub fn with_export(name: String, exported: ExportMark) -> Self {
        IdentDef { name, exported }
    }
}