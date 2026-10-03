#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,

    // Variables & Identifiers
    Variable(String),

    // Binary operations: a + b, x * y, etc.
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    // Unary operations: -x, !flag
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    // Function calls: foo(a, b) or println(x)
    Call {
        callee: String,
        args: Vec<Expr>,
    },

    // Simulation Vector Constructors
    Vec2(Box<Expr>, Box<Expr>),
    Vec3(Box<Expr>, Box<Expr>, Box<Expr>),
    Vec4(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Variable Declaration: let [mut] name [: type] = expr;
    Let {
        name: String,
        is_mut: bool,
        type_annotation: Option<String>,
        initializer: Expr,
    },

    // Assignment: x = 42;
    Assign {
        name: String,
        value: Expr,
    },

    // Standalone expression statement
    Expression(Expr),

    // Code block: { ... }
    Block(Vec<Stmt>),

    // Conditionals: if (cond) { ... } else { ... }
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    // Loops
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Loop {
        body: Box<Stmt>,
    },

    // Dedicated simulation step construct: step(dt) { ... }
    Step {
        dt_ident: String,
        body: Box<Stmt>,
    },

    // Jump statements
    Break,
    Continue,
    Return(Option<Expr>),

    // Function declaration: function name(args) => ret_type { ... }
    Function {
        name: String,
        params: Vec<(String, Option<String>)>, // (name, type)
        return_type: Option<String>,
        body: Box<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
