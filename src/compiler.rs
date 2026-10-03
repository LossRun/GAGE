#![allow(warnings)]
use crate::ast::*;
use crate::bytecode::{Chunk, OpCode, Value};

pub struct Compiler {
    pub chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Self { chunk: Chunk::new() }
    }

    pub fn compile(&mut self, program: &Program) -> Result<Chunk, String> {
        for stmt in &program.statements {
            self.compile_stmt(stmt)?;
        }
        self.chunk.write(OpCode::Halt, 0);
        Ok(self.chunk.clone())
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let(name, expr) => {
                self.compile_expr(expr)?;
                self.chunk.write(OpCode::DefineGlobal(name.clone()), 0);
            }
            Stmt::Assign(name, expr) => {
                self.compile_expr(expr)?;
                self.chunk.write(OpCode::SetGlobal(name.clone()), 0);
            }
            Stmt::Print(expr) | Stmt::Println(expr) => {
                self.compile_expr(expr)?;
                self.chunk.write(OpCode::Print, 0);
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
            }
            Stmt::If { cond, then_branch, else_branch } => {
                self.compile_expr(cond)?;
                let jump_then = self.chunk.code.len();
                self.chunk.write(OpCode::JumpIfFalse(0), 0);

                for s in then_branch { self.compile_stmt(s)?; }

                let jump_else = self.chunk.code.len();
                self.chunk.write(OpCode::Jump(0), 0);

                let else_pos = self.chunk.code.len();
                self.chunk.code[jump_then] = OpCode::JumpIfFalse(else_pos);

                if let Some(el) = else_branch {
                    for s in el { self.compile_stmt(s)?; }
                }
                let end_pos = self.chunk.code.len();
                self.chunk.code[jump_else] = OpCode::Jump(end_pos);
            }
            Stmt::While { cond, body } => {
                let loop_start = self.chunk.code.len();
                self.compile_expr(cond)?;
                let exit_jump = self.chunk.code.len();
                self.chunk.write(OpCode::JumpIfFalse(0), 0);

                for s in body { self.compile_stmt(s)?; }
                self.chunk.write(OpCode::Jump(loop_start), 0);

                let end_pos = self.chunk.code.len();
                self.chunk.code[exit_jump] = OpCode::JumpIfFalse(end_pos);
            }
            Stmt::Loop { body } => {
                let loop_start = self.chunk.code.len();
                for s in body { self.compile_stmt(s)?; }
                self.chunk.write(OpCode::Jump(loop_start), 0);
            }
            Stmt::Break => {
                self.chunk.write(OpCode::Halt, 0);
            }
            Stmt::Step { dt_var, body } => {
                let idx = self.chunk.add_constant(Value::Float(0.016667));
                self.chunk.write(OpCode::Constant(idx), 0);
                self.chunk.write(OpCode::DefineGlobal(dt_var.clone()), 0);

                for s in body { self.compile_stmt(s)?; }
            }
            _ => {}
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), String> {
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
                let idx = self.chunk.add_constant(Value::Bool(*b));
                self.chunk.write(OpCode::Constant(idx), 0);
            }
            Expr::Nil => {
                let idx = self.chunk.add_constant(Value::Nil);
                self.chunk.write(OpCode::Constant(idx), 0);
            }
            Expr::Ident(name) => {
                self.chunk.write(OpCode::GetGlobal(name.clone()), 0);
            }
            Expr::Binary(left, op, right) => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                match op {
                    BinaryOp::Add => { self.chunk.write(OpCode::Add, 0); }
                    BinaryOp::Sub => { self.chunk.write(OpCode::Sub, 0); }
                    BinaryOp::Mul => { self.chunk.write(OpCode::Mul, 0); }
                    BinaryOp::Div => { self.chunk.write(OpCode::Div, 0); }
                    BinaryOp::Mod => {  }
                    BinaryOp::Eq => { self.chunk.write(OpCode::Equal, 0); }
                    BinaryOp::Neq => { self.chunk.write(OpCode::NotEqual, 0); }
                    BinaryOp::Lt => { self.chunk.write(OpCode::Less, 0); }
                    BinaryOp::Lte => { self.chunk.write(OpCode::LessEqual, 0); }
                    BinaryOp::Gt => { self.chunk.write(OpCode::Greater, 0); }
                    BinaryOp::Gte => { self.chunk.write(OpCode::GreaterEqual, 0); }
                    _ => {}
                };
            }
            Expr::Unary(op, e) => {
                self.compile_expr(e)?;
                match op {
                    UnaryOp::Neg => { self.chunk.write(OpCode::Negate, 0); }
                    UnaryOp::Not => { self.chunk.write(OpCode::Not, 0); }
                };
            }
            _ => {}
        }
        Ok(())
    }
}
