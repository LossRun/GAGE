#![allow(warnings)]
use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Type { Int, Float, Bool, Str, Vec2, Vec3, Vec4, Nil, Any }

pub struct TypeChecker { scopes: Vec<HashMap<String, Type>> }

impl TypeChecker {
    pub fn new() -> Self { Self { scopes: vec![HashMap::new()] } }
    fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn pop_scope(&mut self) { self.scopes.pop(); }
    fn insert(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() { scope.insert(name.to_string(), ty); }
    }
    fn lookup(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) { return Some(ty.clone()); }
        }
        None
    }

    pub fn check(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements { self.check_stmt(stmt)?; }
        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let(name, expr) => {
                let ty = self.check_expr(expr)?;
                self.insert(name, ty);
                Ok(())
            }
            Stmt::Assign(name, expr) => {
                let expr_ty = self.check_expr(expr)?;
                if let Some(var_ty) = self.lookup(name) {
                    if var_ty != Type::Any && expr_ty != Type::Any && var_ty != expr_ty {
                        if var_ty == Type::Float && expr_ty == Type::Int { return Ok(()); }
                    }
                    Ok(())
                } else {
                    Err(format!("[Semantic Error] Undefined variable '{}'", name))
                }
            }
            Stmt::Print(expr) | Stmt::Println(expr) | Stmt::Expr(expr) => {
                self.check_expr(expr)?;
                Ok(())
            }
            Stmt::If { cond, then_branch, else_branch } => {
                self.check_expr(cond)?;
                self.push_scope();
                for s in then_branch { self.check_stmt(s)?; }
                self.pop_scope();
                if let Some(el) = else_branch {
                    self.push_scope();
                    for s in el { self.check_stmt(s)?; }
                    self.pop_scope();
                }
                Ok(())
            }
            Stmt::While { cond, body } => {
                self.check_expr(cond)?;
                self.push_scope();
                for s in body { self.check_stmt(s)?; }
                self.pop_scope();
                Ok(())
            }
            Stmt::Loop { body } => {
                self.push_scope();
                for s in body { self.check_stmt(s)?; }
                self.pop_scope();
                Ok(())
            }
            Stmt::Break => Ok(()),
            Stmt::Step { dt_var, body } => {
                self.push_scope();
                self.insert(dt_var, Type::Float);
                for s in body { self.check_stmt(s)?; }
                self.pop_scope();
                Ok(())
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Nil => Ok(Type::Nil),
            Expr::Ident(name) => self.lookup(name).ok_or_else(|| format!("[Semantic Error] Undefined variable '{}'", name)),
            Expr::Vec2(x, y) => { self.check_expr(x)?; self.check_expr(y)?; Ok(Type::Vec2) }
            Expr::Vec3(x, y, z) => { self.check_expr(x)?; self.check_expr(y)?; self.check_expr(z)?; Ok(Type::Vec3) }
            Expr::Vec4(x, y, z, w) => { self.check_expr(x)?; self.check_expr(y)?; self.check_expr(z)?; self.check_expr(w)?; Ok(Type::Vec4) }
            Expr::Binary(left, op, right) => {
                let lt = self.check_expr(left)?;
                let rt = self.check_expr(right)?;
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        if lt == Type::Vec3 || rt == Type::Vec3 { Ok(Type::Vec3) }
                        else if lt == Type::Vec2 || rt == Type::Vec2 { Ok(Type::Vec2) }
                        else if lt == Type::Vec4 || rt == Type::Vec4 { Ok(Type::Vec4) }
                        else if lt == Type::Float || rt == Type::Float { Ok(Type::Float) }
                        else { Ok(Type::Int) }
                    }
                    BinaryOp::Mod => Ok(Type::Int),
                    BinaryOp::Eq | BinaryOp::Neq | BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte |
                    BinaryOp::And | BinaryOp::Or => Ok(Type::Bool),
                }
            }
            Expr::Unary(UnaryOp::Neg, e) => self.check_expr(e),
            Expr::Unary(UnaryOp::Not, e) => { self.check_expr(e)?; Ok(Type::Bool) }
            Expr::Input(p) => { if let Some(prompt) = p { self.check_expr(prompt)?; } Ok(Type::Str) }
            Expr::ReadFile(p) => { self.check_expr(p)?; Ok(Type::Str) }
            Expr::WriteFile(p, c) => { self.check_expr(p)?; self.check_expr(c)?; Ok(Type::Bool) }
        }
    }
}
