use crate::ast::{BinaryOp, Expr, Program, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Vec2,
    Vec3,
    Vec4,
    Nil,
    Any,
}

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut global = HashMap::new();
        global.insert("println".to_string(), Type::Any);
        global.insert("print".to_string(), Type::Any);
        Self {
            scopes: vec![global],
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    fn lookup(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, initializer, .. } => {
                let init_ty = self.check_expr(initializer)?;
                self.declare(name, init_ty);
                Ok(())
            }
            Stmt::Assign { name, value } => {
                let var_ty = self.lookup(name)
                    .ok_or_else(|| format!("[Type Error] Variable '{}' assigned before declaration", name))?;
                let val_ty = self.check_expr(value)?;
                if var_ty != Type::Any && val_ty != Type::Any && var_ty != val_ty {
                    return Err(format!("[Type Error] Cannot assign {:?} to variable '{}' of type {:?}", val_ty, name, var_ty));
                }
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.check_expr(expr)?;
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.enter_scope();
                for s in stmts {
                    self.check_stmt(s)?;
                }
                self.exit_scope();
                Ok(())
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.check_expr(condition)?;
                self.check_stmt(then_branch)?;
                if let Some(eb) = else_branch {
                    self.check_stmt(eb)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                self.check_expr(condition)?;
                self.check_stmt(body)?;
                Ok(())
            }
            Stmt::Loop { body } => {
                self.check_stmt(body)?;
                Ok(())
            }
            Stmt::Step { dt_ident, body } => {
                self.enter_scope();
                self.declare(dt_ident, Type::Float);
                self.check_stmt(body)?;
                self.exit_scope();
                Ok(())
            }
            Stmt::Function { name, params, body, .. } => {
                self.declare(name, Type::Any);
                self.enter_scope();
                for (param_name, _) in params {
                    self.declare(param_name, Type::Any);
                }
                self.check_stmt(body)?;
                self.exit_scope();
                Ok(())
            }
            Stmt::Return(maybe_expr) => {
                if let Some(e) = maybe_expr {
                    self.check_expr(e)?;
                }
                Ok(())
            }
            Stmt::Break | Stmt::Continue => Ok(()),
        }
    }

    fn check_expr(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Nil => Ok(Type::Nil),
            Expr::Variable(name) => {
                self.lookup(name)
                    .ok_or_else(|| format!("[Semantic Error] Undefined variable '{}'", name))
            }
            Expr::Vec2(x, y) => {
                self.check_expr(x)?;
                self.check_expr(y)?;
                Ok(Type::Vec2)
            }
            Expr::Vec3(x, y, z) => {
                self.check_expr(x)?;
                self.check_expr(y)?;
                self.check_expr(z)?;
                Ok(Type::Vec3)
            }
            Expr::Vec4(x, y, z, w) => {
                self.check_expr(x)?;
                self.check_expr(y)?;
                self.check_expr(z)?;
                self.check_expr(w)?;
                Ok(Type::Vec4)
            }
            Expr::Binary { left, op, right } => {
                let lt = self.check_expr(left)?;
                let rt = self.check_expr(right)?;
                match op {
                    BinaryOp::Add | BinaryOp::Sub => {
                        if lt == rt {
                            Ok(lt)
                        } else if (lt == Type::Float && rt == Type::Int) || (lt == Type::Int && rt == Type::Float) {
                            Ok(Type::Float)
                        } else {
                            Ok(Type::Any)
                        }
                    }
                    BinaryOp::Mul => {
                        if (lt == Type::Vec3 && (rt == Type::Float || rt == Type::Int)) ||
                           (rt == Type::Vec3 && (lt == Type::Float || lt == Type::Int)) {
                            Ok(Type::Vec3)
                        } else {
                            Ok(lt)
                        }
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                        Ok(Type::Bool)
                    }
                    _ => Ok(lt),
                }
            }
            Expr::Unary { expr, .. } => self.check_expr(expr),
            Expr::Call { callee, args } => {
                for arg in args {
                    self.check_expr(arg)?;
                }
                if callee == "vec2" { return Ok(Type::Vec2); }
                if callee == "vec3" { return Ok(Type::Vec3); }
                if callee == "vec4" { return Ok(Type::Vec4); }
                Ok(Type::Any)
            }
        }
    }
}
