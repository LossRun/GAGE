#![allow(warnings)]
use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Vec2,
    Vec3,
    Vec4,
    Array(Box<Type>),
    Custom(String),
    Nil,
    Any,
}

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
    functions: HashMap<String, (Vec<Type>, Type)>,
    classes: HashMap<String, HashMap<String, Type>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn pop_scope(&mut self) { self.scopes.pop(); }

    fn insert(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    fn lookup(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) { return Some(ty.clone()); }
        }
        None
    }

    pub fn check(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            if let Stmt::Class(c) = stmt {
                let mut fields = HashMap::new();
                for f in &c.fields { fields.insert(f.clone(), Type::Any); }
                self.classes.insert(c.name.clone(), fields);
            }
        }
        for stmt in &program.statements {
            self.check_stmt(stmt)?;
        }
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
            Stmt::MemberAssign(obj, field, expr) => {
                self.check_expr(obj)?;
                self.check_expr(expr)?;
                Ok(())
            }
            Stmt::IndexAssign(arr, idx, expr) => {
                self.check_expr(arr)?;
                self.check_expr(idx)?;
                self.check_expr(expr)?;
                Ok(())
            }
            Stmt::Print(expr) | Stmt::Println(expr) | Stmt::Expr(expr) => {
                self.check_expr(expr)?;
                Ok(())
            }
            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt { self.check_expr(expr)?; }
                Ok(())
            }
            Stmt::Function(f) => {
                self.push_scope();
                for p in &f.params { self.insert(p, Type::Any); }
                for s in &f.body { self.check_stmt(s)?; }
                self.pop_scope();
                self.functions.insert(f.name.clone(), (vec![Type::Any; f.params.len()], Type::Any));
                Ok(())
            }
            Stmt::Class(c) => {
                for m in &c.methods {
                    self.push_scope();
                    self.insert("this", Type::Custom(c.name.clone()));
                    for p in &m.params { self.insert(p, Type::Any); }
                    for s in &m.body { self.check_stmt(s)?; }
                    self.pop_scope();
                }
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
            Stmt::For { var, iter, body } => {
                self.check_expr(iter)?;
                self.push_scope();
                self.insert(var, Type::Any);
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
            Expr::This => self.lookup("this").ok_or_else(|| "[Semantic Error] 'this' used outside class method".into()),
            Expr::Ident(name) => self.lookup(name).ok_or_else(|| format!("[Semantic Error] Undefined variable '{}'", name)),
            Expr::Vec2(x, y) => { self.check_expr(x)?; self.check_expr(y)?; Ok(Type::Vec2) }
            Expr::Vec3(x, y, z) => { self.check_expr(x)?; self.check_expr(y)?; self.check_expr(z)?; Ok(Type::Vec3) }
            Expr::Vec4(x, y, z, w) => { self.check_expr(x)?; self.check_expr(y)?; self.check_expr(z)?; self.check_expr(w)?; Ok(Type::Vec4) }
            Expr::Array(elements) => {
                for el in elements { self.check_expr(el)?; }
                Ok(Type::Array(Box::new(Type::Any)))
            }
            Expr::New(name, args) => {
                for a in args { self.check_expr(a)?; }
                Ok(Type::Custom(name.clone()))
            }
            Expr::MemberAccess(obj, _) => {
                self.check_expr(obj)?;
                Ok(Type::Any)
            }
            Expr::IndexAccess(arr, idx) => {
                self.check_expr(arr)?;
                self.check_expr(idx)?;
                Ok(Type::Any)
            }
            Expr::Call(_, args) => {
                for a in args { self.check_expr(a)?; }
                Ok(Type::Any)
            }
            Expr::MethodCall(obj, _, args) => {
                self.check_expr(obj)?;
                for a in args { self.check_expr(a)?; }
                Ok(Type::Any)
            }
            Expr::Dot(a, b) => { self.check_expr(a)?; self.check_expr(b)?; Ok(Type::Float) }
            Expr::Cross(a, b) => { self.check_expr(a)?; self.check_expr(b)?; Ok(Type::Vec3) }
            Expr::Length(v) => { self.check_expr(v)?; Ok(Type::Float) }
            Expr::Normalize(v) => {
                let ty = self.check_expr(v)?;
                Ok(ty)
            }
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
                        Expr::Random => Ok(Type::Float),
            Expr::ClearScreen => Ok(Type::Nil),
            Expr::ParseInt(e) => { self.check_expr(e)?; Ok(Type::Int) },
            Expr::Clamp(x, mi, ma) => {
                self.check_expr(x)?; self.check_expr(mi)?; self.check_expr(ma)?;
                Ok(Type::Float)
            },
            Expr::Lerp(a, b, t) => {
                let ta = self.check_expr(a)?;
                self.check_expr(b)?; self.check_expr(t)?;
                Ok(ta)
            },
            Expr::Distance(a, b) => {
                self.check_expr(a)?; self.check_expr(b)?;
                Ok(Type::Float)
            },
            Expr::Reflect(v, n) => {
                let tv = self.check_expr(v)?;
                self.check_expr(n)?;
                Ok(tv)
            },
            Expr::WriteFile(p, c) => { self.check_expr(p)?; self.check_expr(c)?; Ok(Type::Bool) }
        }
    }
}
