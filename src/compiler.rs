#![allow(warnings)]
use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::bytecode::{Chunk, OpCode, Value};

#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
}

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
}

#[derive(Debug)]
pub struct CompileError {
    pub message: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
        }
    }

    pub fn compile(mut self, program: &Program) -> Result<Chunk, CompileError> {
        for stmt in &program.statements {
            self.compile_stmt(stmt)?;
        }
        self.chunk.write(OpCode::Halt, 0);
        Ok(self.chunk)
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Let { name, initializer, .. } => {
                self.compile_expr(initializer)?;
                if self.scope_depth > 0 {
                    self.locals.push(Local {
                        name: name.clone(),
                        depth: self.scope_depth,
                    });
                } else {
                    self.chunk.write(OpCode::DefineGlobal(name.clone()), 0);
                }
            }

            Stmt::Assign { name, value } => {
                self.compile_expr(value)?;
                if let Some(idx) = self.resolve_local(name) {
                    self.chunk.write(OpCode::SetLocal(idx), 0);
                } else {
                    self.chunk.write(OpCode::SetGlobal(name.clone()), 0);
                }
            }

            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
                self.chunk.write(OpCode::Pop, 0);
            }

            Stmt::Block(stmts) => {
                self.begin_scope();
                for s in stmts {
                    self.compile_stmt(s)?;
                }
                self.end_scope();
            }

            Stmt::If { condition, then_branch, else_branch } => {
                self.compile_expr(condition)?;
                let then_jump = self.chunk.write(OpCode::JumpIfFalse(0), 0);

                self.compile_stmt(then_branch)?;

                if let Some(else_b) = else_branch {
                    let else_jump = self.chunk.write(OpCode::Jump(0), 0);
                    self.patch_jump(then_jump);
                    self.compile_stmt(else_b)?;
                    self.patch_jump(else_jump);
                } else {
                    self.patch_jump(then_jump);
                }
            }

            Stmt::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                self.compile_expr(condition)?;
                let exit_jump = self.chunk.write(OpCode::JumpIfFalse(0), 0);

                self.compile_stmt(body)?;
                self.chunk.write(OpCode::Loop(loop_start), 0);
                self.patch_jump(exit_jump);
            }

            Stmt::Loop { body } => {
                let loop_start = self.chunk.code.len();
                self.compile_stmt(body)?;
                self.chunk.write(OpCode::Loop(loop_start), 0);
            }

            Stmt::Step { dt_ident, body } => {
                // Fixed-timestep simulation hook
                self.begin_scope();
                let dt_const = self.chunk.add_constant(Value::Float(0.016667)); // 60 FPS standard
                self.chunk.write(OpCode::Constant(dt_const), 0);
                self.locals.push(Local {
                    name: dt_ident.clone(),
                    depth: self.scope_depth,
                });
                self.compile_stmt(body)?;
                self.end_scope();
            }

            Stmt::Break | Stmt::Continue => {
                // Future expansion: break/continue jump tables
            }

            Stmt::Return(maybe_expr) => {
                if let Some(expr) = maybe_expr {
                    self.compile_expr(expr)?;
                } else {
                    self.chunk.write(OpCode::Nil, 0);
                }
                self.chunk.write(OpCode::Return, 0);
            }

            Stmt::Function { body, .. } => {
                // For main or global scripts, compile body directly
                self.compile_stmt(body)?;
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Int(n) => {
                let idx = self.chunk.add_constant(Value::Int(*n));
                self.chunk.write(OpCode::Constant(idx), 0);
            }
            Expr::Float(f) => {
                let idx = self.chunk.add_constant(Value::Float(*f));
                self.chunk.write(OpCode::Constant(idx), 0);
            }
            Expr::Str(s) => {
                let idx = self.chunk.add_constant(Value::Str(s.clone()));
                self.chunk.write(OpCode::Constant(idx), 0);
            }
            Expr::Bool(b) => {
                if *b {
                    self.chunk.write(OpCode::True, 0);
                } else {
                    self.chunk.write(OpCode::False, 0);
                }
            }
            Expr::Nil => {
                self.chunk.write(OpCode::Nil, 0);
            }
            Expr::Variable(name) => {
                if let Some(idx) = self.resolve_local(name) {
                    self.chunk.write(OpCode::GetLocal(idx), 0);
                } else {
                    self.chunk.write(OpCode::GetGlobal(name.clone()), 0);
                }
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                match op {
                    BinaryOp::Add => self.chunk.write(OpCode::Add, 0),
                    BinaryOp::Sub => self.chunk.write(OpCode::Sub, 0),
                    BinaryOp::Mul => self.chunk.write(OpCode::Mul, 0),
                    BinaryOp::Div => self.chunk.write(OpCode::Div, 0),
                    BinaryOp::Equal => self.chunk.write(OpCode::Equal, 0),
                    BinaryOp::NotEqual => self.chunk.write(OpCode::NotEqual, 0),
                    BinaryOp::Less => self.chunk.write(OpCode::Less, 0),
                    BinaryOp::LessEqual => self.chunk.write(OpCode::LessEqual, 0),
                    BinaryOp::Greater => self.chunk.write(OpCode::Greater, 0),
                    BinaryOp::GreaterEqual => self.chunk.write(OpCode::GreaterEqual, 0),
                };
            }
            Expr::Unary { op, expr } => {
                self.compile_expr(expr)?;
                match op {
                    UnaryOp::Negate => self.chunk.write(OpCode::Negate, 0),
                    UnaryOp::Not => self.chunk.write(OpCode::Not, 0),
                };
            }
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }
                match callee.as_str() {
                    "println" => {
                        self.chunk.write(OpCode::Println, 0);
                        self.chunk.write(OpCode::Nil, 0);
                    }
                    "print" => {
                        self.chunk.write(OpCode::Print, 0);
                        self.chunk.write(OpCode::Nil, 0);
                    }
                    _ => {
                        return Err(CompileError {
                            message: format!("Unknown function call: {}", callee),
                        });
                    }
                }
            }
            Expr::Vec2(x, y) => {
                self.compile_expr(x)?;
                self.compile_expr(y)?;
                self.chunk.write(OpCode::MakeVec2, 0);
            }
            Expr::Vec3(x, y, z) => {
                self.compile_expr(x)?;
                self.compile_expr(y)?;
                self.compile_expr(z)?;
                self.chunk.write(OpCode::MakeVec3, 0);
            }
            Expr::Vec4(x, y, z, w) => {
                self.compile_expr(x)?;
                self.compile_expr(y)?;
                self.compile_expr(z)?;
                self.compile_expr(w)?;
                self.chunk.write(OpCode::MakeVec4, 0);
            }
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.locals.pop();
                self.chunk.write(OpCode::Pop, 0);
            } else {
                break;
            }
        }
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i);
            }
        }
        None
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len();
        match &mut self.chunk.code[offset] {
            OpCode::JumpIfFalse(target) => *target = jump,
            OpCode::Jump(target) => *target = jump,
            _ => {}
        }
    }
}
