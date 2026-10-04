#![allow(warnings)]

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Neq, Lt, Lte, Gt, Gte,
    And, Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
    Ident(String),
    This,

    // Constructors & Collections
    Vec2(Box<Expr>, Box<Expr>),
    Vec3(Box<Expr>, Box<Expr>, Box<Expr>),
    Vec4(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
    New(String, Vec<Expr>),

    // Member Access & Invocations
    MemberAccess(Box<Expr>, String),
    IndexAccess(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    MethodCall(Box<Expr>, String, Vec<Expr>),

    // Math & Built-ins
    Dot(Box<Expr>, Box<Expr>),
    Cross(Box<Expr>, Box<Expr>),
    Length(Box<Expr>),
    Normalize(Box<Expr>),
    Input(Option<Box<Expr>>),
    ReadFile(Box<Expr>),
    WriteFile(Box<Expr>, Box<Expr>),
    Random,
    Clamp(Box<Expr>, Box<Expr>, Box<Expr>),
    ParseInt(Box<Expr>),
    ClearScreen,
    Lerp(Box<Expr>, Box<Expr>, Box<Expr>),
    Distance(Box<Expr>, Box<Expr>),
    Reflect(Box<Expr>, Box<Expr>),

    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Assign(String, Expr),
    MemberAssign(Box<Expr>, String, Expr),
    IndexAssign(Box<Expr>, Box<Expr>, Expr),
    Print(Expr),
    Println(Expr),
    Return(Option<Expr>),
    Expr(Expr),
    Function(FunctionDecl),
    Class(ClassDecl),
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
    },
    Loop {
        body: Vec<Stmt>,
    },
    Break,
    Step {
        dt_var: String,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
