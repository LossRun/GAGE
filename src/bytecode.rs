#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Vec2(f64, f64),
    Vec3(f64, f64, f64),
    Vec4(f64, f64, f64, f64),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            _ => true,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Str(s) => write!(f, "{}", s),
            Value::Vec2(x, y) => write!(f, "vec2({}, {})", x, y),
            Value::Vec3(x, y, z) => write!(f, "vec3({}, {}, {})", x, y, z),
            Value::Vec4(x, y, z, w) => write!(f, "vec4({}, {}, {}, {})", x, y, z, w),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Constant(usize),
    Nil,
    True,
    False,

    // Vector operations
    MakeVec2,
    MakeVec3,
    MakeVec4,

    // Math & Comparison
    Add,
    Sub,
    Mul,
    Div,
    Negate,
    Not,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Variables & Stack
    Pop,
    DefineGlobal(String),
    GetGlobal(String),
    SetGlobal(String),
    GetLocal(usize),
    SetLocal(usize),

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),
    Loop(usize),

    // Standard I/O
    Print,
    Println,

    // Subroutine & VM Control
    Return,
    Halt,
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, op: OpCode, line: usize) -> usize {
        self.code.push(op);
        self.lines.push(line);
        self.code.len() - 1
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
