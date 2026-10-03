import re

with open("/sdcard/GAGE/src/codegen.rs", "w") as f:
    f.write(r'''#![allow(warnings)]
use std::collections::HashMap;
use crate::ast::*;

pub struct CodeGen {
    out: String,
    indent_level: usize,
    var_classes: HashMap<String, String>,
    method_classes: HashMap<String, String>,
    current_class: Option<String>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            indent_level: 0,
            var_classes: HashMap::new(),
            method_classes: HashMap::new(),
            current_class: None,
        }
    }

    fn emit_line(&mut self, text: &str) {
        let ind = "  ".repeat(self.indent_level);
        self.out.push_str(&format!("{}{}\n", ind, text));
    }

    pub fn generate(mut self, program: &Program) -> String {
        // Collect class methods and variable instance classes
        for stmt in &program.statements {
            if let Stmt::Class(c) = stmt {
                for m in &c.methods {
                    self.method_classes.insert(m.name.clone(), c.name.clone());
                }
            }
        }
        for stmt in &program.statements {
            if let Stmt::Let(name, expr) = stmt {
                if let Expr::New(c_name, _) = expr {
                    self.var_classes.insert(name.clone(), c_name.clone());
                }
            }
        }

        self.emit_line("#pragma GCC diagnostic ignored \"-Wunused-function\"");
        self.emit_line("#pragma GCC diagnostic ignored \"-Wunused-variable\"");
        self.emit_line("#include <stdio.h>");
        self.emit_line("#include <stdlib.h>");
        self.emit_line("#include <stdbool.h>");
        self.emit_line("#include <string.h>");
        self.emit_line("#include <math.h>\n");

        // Clang native SIMD vector types supporting +, -, * operators
        self.emit_line("typedef double gage_vec2 __attribute__((ext_vector_type(2)));");
        self.emit_line("typedef double gage_vec3 __attribute__((ext_vector_type(3)));");
        self.emit_line("typedef double gage_vec4 __attribute__((ext_vector_type(4)));\n");

        self.emit_line("static inline gage_vec2 make_vec2(double x, double y) { return (gage_vec2){x, y}; }");
        self.emit_line("static inline gage_vec3 make_vec3(double x, double y, double z) { return (gage_vec3){x, y, z}; }");
        self.emit_line("static inline gage_vec4 make_vec4(double x, double y, double z, double w) { return (gage_vec4){x, y, z, w}; }\n");

        self.emit_line("static inline double gage_dot_vec3(gage_vec3 a, gage_vec3 b) { return a.x*b.x + a.y*b.y + a.z*b.z; }");
        self.emit_line("static inline gage_vec3 gage_cross_vec3(gage_vec3 a, gage_vec3 b) { return (gage_vec3){a.y*b.z - a.z*b.y, a.z*b.x - a.x*b.z, a.x*b.y - a.y*b.x}; }");
        self.emit_line("static inline double gage_length_vec3(gage_vec3 v) { return sqrt(v.x*v.x + v.y*v.y + v.z*v.z); }");
        self.emit_line("static inline gage_vec3 gage_normalize_vec3(gage_vec3 v) { double l = gage_length_vec3(v); return (gage_vec3){v.x/l, v.y/l, v.z/l}; }\n");

        // Native Double Array Runtime
        self.emit_line("typedef struct { double* data; size_t length; size_t capacity; } gage_array;");
        self.emit_line("static inline gage_array* gage_create_array(size_t cap) {");
        self.emit_line("  gage_array* a = malloc(sizeof(gage_array));");
        self.emit_line("  a->length = 0; a->capacity = cap > 0 ? cap : 8;");
        self.emit_line("  a->data = malloc(sizeof(double) * a->capacity);");
        self.emit_line("  return a;");
        self.emit_line("}");
        self.emit_line("static inline void gage_array_push(gage_array* a, double item) {");
        self.emit_line("  if (a->length >= a->capacity) { a->capacity *= 2; a->data = realloc(a->data, sizeof(double) * a->capacity); }");
        self.emit_line("  a->data[a->length++] = item;");
        self.emit_line("}\n");

        // Runtime I/O Helpers
        self.emit_line("static inline char* gage_input(const char* prompt) {");
        self.emit_line("  if (prompt && strlen(prompt) > 0) { printf(\"%s\", prompt); fflush(stdout); }");
        self.emit_line("  char buffer[4096];");
        self.emit_line("  if (fgets(buffer, sizeof(buffer), stdin) != NULL) {");
        self.emit_line("    size_t len = strlen(buffer);");
        self.emit_line("    if (len > 0 && buffer[len - 1] == '\\n') buffer[len - 1] = '\\0';");
        self.emit_line("    return strdup(buffer);");
        self.emit_line("  }");
        self.emit_line("  return strdup(\"\");");
        self.emit_line("}");
        self.emit_line("static inline char* gage_read_file(const char* path) {");
        self.emit_line("  FILE *f = fopen(path, \"rb\"); if (!f) return strdup(\"\");");
        self.emit_line("  fseek(f, 0, SEEK_END); long size = ftell(f); fseek(f, 0, SEEK_SET);");
        self.emit_line("  char *str = malloc(size + 1); fread(str, 1, size, f); fclose(f);");
        self.emit_line("  str[size] = '\\0'; return str;");
        self.emit_line("}");
        self.emit_line("static inline bool gage_write_file(const char* path, const char* content) {");
        self.emit_line("  FILE *f = fopen(path, \"wb\"); if (!f) return false;");
        self.emit_line("  fputs(content, f); fclose(f); return true;");
        self.emit_line("}\n");

        // C11 Generic Dispatchers
        self.emit_line("static inline void _gage_print_i64(long long v) { printf(\"%lld\", v); }");
        self.emit_line("static inline void _gage_print_f64(double v) { printf(\"%g\", v); }");
        self.emit_line("static inline void _gage_print_str(const char* v) { printf(\"%s\", v ? v : \"\"); }");
        self.emit_line("static inline void _gage_print_bool(bool v) { printf(\"%s\", v ? \"true\" : \"false\"); }");
        self.emit_line("static inline void _gage_print_vec2(gage_vec2 v) { printf(\"vec2(%g, %g)\", v.x, v.y); }");
        self.emit_line("static inline void _gage_print_vec3(gage_vec3 v) { printf(\"vec3(%g, %g, %g)\", v.x, v.y, v.z); }");
        self.emit_line("static inline void _gage_print_vec4(gage_vec4 v) { printf(\"vec4(%g, %g, %g, %g)\", v.x, v.y, v.z, v.w); }");
        self.emit_line("static inline void _gage_print_default(const void* v) { (void)v; printf(\"<val>\"); }\n");

        self.emit_line("#define gage_print(x) _Generic((x), \\");
        self.emit_line("  int: _gage_print_i64, \\");
        self.emit_line("  long: _gage_print_i64, \\");
        self.emit_line("  long long: _gage_print_i64, \\");
        self.emit_line("  unsigned int: _gage_print_i64, \\");
        self.emit_line("  unsigned long: _gage_print_i64, \\");
        self.emit_line("  unsigned long long: _gage_print_i64, \\");
        self.emit_line("  float: _gage_print_f64, \\");
        self.emit_line("  double: _gage_print_f64, \\");
        self.emit_line("  char*: _gage_print_str, \\");
        self.emit_line("  const char*: _gage_print_str, \\");
        self.emit_line("  bool: _gage_print_bool, \\");
        self.emit_line("  gage_vec2: _gage_print_vec2, \\");
        self.emit_line("  gage_vec3: _gage_print_vec3, \\");
        self.emit_line("  gage_vec4: _gage_print_vec4, \\");
        self.emit_line("  default: _gage_print_default \\");
        self.emit_line(")(x)");
        self.emit_line("#define gage_println(x) do { gage_print(x); printf(\"\\n\"); fflush(stdout); } while(0)\n");

        // Forward declare classes
        for stmt in &program.statements {
            if let Stmt::Class(c) = stmt {
                self.emit_line(&format!("typedef struct {} {{", c.name));
                self.indent_level += 1;
                for f in &c.fields {
                    self.emit_line(&format!("double {};", f));
                }
                self.indent_level -= 1;
                self.emit_line(&format!("}} {};", c.name));

                for m in &c.methods {
                    let mut params_str = format!("{}* this", c.name);
                    for p in &m.params { params_str.push_str(&format!(", double {}", p)); }
                    self.emit_line(&format!("double {}__{}({});", c.name, m.name, params_str));
                }
            }
        }

        // Functions
        for stmt in &program.statements {
            if let Stmt::Function(f) = stmt {
                let mut params_str = String::new();
                for (i, p) in f.params.iter().enumerate() {
                    if i > 0 { params_str.push_str(", "); }
                    params_str.push_str(&format!("double {}", p));
                }
                self.emit_line(&format!("double {}({}) {{", f.name, params_str));
                self.indent_level += 1;
                for s in &f.body { self.gen_stmt(s); }
                self.emit_line("return 0;");
                self.indent_level -= 1;
                self.emit_line("}\n");
            }
        }

        // Class Methods
        for stmt in &program.statements {
            if let Stmt::Class(c) = stmt {
                self.current_class = Some(c.name.clone());
                for m in &c.methods {
                    let mut params_str = format!("{}* this", c.name);
                    for p in &m.params { params_str.push_str(&format!(", double {}", p)); }
                    self.emit_line(&format!("double {}__{}({}) {{", c.name, m.name, params_str));
                    self.indent_level += 1;
                    for s in &m.body { self.gen_stmt(s); }
                    self.emit_line("return 0;");
                    self.indent_level -= 1;
                    self.emit_line("}\n");
                }
                self.current_class = None;
            }
        }

        // Main
        self.emit_line("int main(void) {");
        self.indent_level += 1;
        for stmt in &program.statements {
            if !matches!(stmt, Stmt::Function(_) | Stmt::Class(_)) {
                self.gen_stmt(stmt);
            }
        }
        self.emit_line("return 0;");
        self.indent_level -= 1;
        self.emit_line("}");

        self.out
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let(name, expr) => {
                if let Expr::New(c_name, _) = expr {
                    self.var_classes.insert(name.clone(), c_name.clone());
                }
                let e = self.gen_expr(expr);
                self.emit_line(&format!("__auto_type {} = {};", name, e));
            }
            Stmt::Assign(name, expr) => {
                let e = self.gen_expr(expr);
                self.emit_line(&format!("{} = {};", name, e));
            }
            Stmt::MemberAssign(obj, field, expr) => {
                let o = self.gen_expr(obj);
                let e = self.gen_expr(expr);
                self.emit_line(&format!("(({})->{}) = {};", o, field, e));
            }
            Stmt::IndexAssign(arr, idx, expr) => {
                let a = self.gen_expr(arr);
                let i = self.gen_expr(idx);
                let e = self.gen_expr(expr);
                self.emit_line(&format!("(({})->data[(size_t)({})]) = (double)({});", a, i, e));
            }
            Stmt::Print(expr) => {
                let e = self.gen_expr(expr);
                self.emit_line(&format!("gage_print({});", e));
                self.emit_line("fflush(stdout);");
            }
            Stmt::Println(expr) => {
                let e = self.gen_expr(expr);
                self.emit_line(&format!("gage_println({});", e));
            }
            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    let e = self.gen_expr(expr);
                    self.emit_line(&format!("return {};", e));
                } else {
                    self.emit_line("return 0;");
                }
            }
            Stmt::Expr(expr) => {
                let e = self.gen_expr(expr);
                self.emit_line(&format!("{};", e));
            }
            Stmt::If { cond, then_branch, else_branch } => {
                let c = self.gen_expr(cond);
                self.emit_line(&format!("if ({}) {{", c));
                self.indent_level += 1;
                for s in then_branch { self.gen_stmt(s); }
                self.indent_level -= 1;
                if let Some(el) = else_branch {
                    self.emit_line("} else {");
                    self.indent_level += 1;
                    for s in el { self.gen_stmt(s); }
                    self.indent_level -= 1;
                }
                self.emit_line("}");
            }
            Stmt::While { cond, body } => {
                let c = self.gen_expr(cond);
                self.emit_line(&format!("while ({}) {{", c));
                self.indent_level += 1;
                for s in body { self.gen_stmt(s); }
                self.indent_level -= 1;
                self.emit_line("}");
            }
            Stmt::For { var, iter, body } => {
                let it = self.gen_expr(iter);
                self.emit_line(&format!("for (size_t _i = 0; _i < ({})->length; ++_i) {{", it));
                self.indent_level += 1;
                self.emit_line(&format!("double {} = ({})->data[_i];", var, it));
                for s in body { self.gen_stmt(s); }
                self.indent_level -= 1;
                self.emit_line("}");
            }
            Stmt::Loop { body } => {
                self.emit_line("while (1) {");
                self.indent_level += 1;
                for s in body { self.gen_stmt(s); }
                self.indent_level -= 1;
                self.emit_line("}");
            }
            Stmt::Break => self.emit_line("break;"),
            Stmt::Step { dt_var, body } => {
                self.emit_line("{");
                self.indent_level += 1;
                self.emit_line(&format!("double {} = 0.016667;", dt_var));
                for s in body { self.gen_stmt(s); }
                self.indent_level -= 1;
                self.emit_line("}");
            }
            Stmt::Function(_) | Stmt::Class(_) => {}
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Int(n) => format!("{}", n),
            Expr::Float(f) => {
                let mut s = format!("{}", f);
                if !s.contains('.') { s.push_str(".0"); }
                s
            }
            Expr::Str(s) => {
                let escaped = s.replace('\\', "\\\\").replace('\"', "\\\"").replace('\n', "\\n").replace('\t', "\\t");
                format!("\"{}\"", escaped)
            }
            Expr::Bool(b) => if *b { "true".into() } else { "false".into() },
            Expr::Nil => "NULL".into(),
            Expr::This => "this".into(),
            Expr::Ident(name) => name.clone(),
            Expr::New(class_name, _args) => format!("({}*)calloc(1, sizeof({}))", class_name, class_name),
            Expr::MemberAccess(obj, member) => {
                let o = self.gen_expr(obj);
                format!("(({})->{})", o, member)
            }
            Expr::MethodCall(obj, method, args) => {
                let o = self.gen_expr(obj);
                let class_name = match obj.as_ref() {
                    Expr::This => self.current_class.clone(),
                    Expr::Ident(name) => self.var_classes.get(name).cloned(),
                    _ => None,
                }.or_else(|| self.method_classes.get(method).cloned())
                 .unwrap_or_else(|| o.clone());

                let mut args_str = o.clone();
                for a in args {
                    args_str.push_str(&format!(", {}", self.gen_expr(a)));
                }
                format!("{}__{}({})", class_name, method, args_str)
            }
            Expr::Call(name, args) => {
                let mut args_str = String::new();
                for (i, a) in args.iter().enumerate() {
                    if i > 0 { args_str.push_str(", "); }
                    args_str.push_str(&self.gen_expr(a));
                }
                format!("{}({})", name, args_str)
            }
            Expr::Array(elements) => {
                let mut s = format!("gage_create_array({})", elements.len());
                for el in elements {
                    s = format!("(gage_array_push({}, (double)({})), {})", s, self.gen_expr(el), s);
                }
                s
            }
            Expr::IndexAccess(arr, idx) => {
                format!("(({})->data[(size_t)({})])", self.gen_expr(arr), self.gen_expr(idx))
            }
            Expr::Vec2(x, y) => format!("make_vec2({}, {})", self.gen_expr(x), self.gen_expr(y)),
            Expr::Vec3(x, y, z) => format!("make_vec3({}, {}, {})", self.gen_expr(x), self.gen_expr(y), self.gen_expr(z)),
            Expr::Vec4(x, y, z, w) => format!("make_vec4({}, {}, {}, {})", self.gen_expr(x), self.gen_expr(y), self.gen_expr(z), self.gen_expr(w)),
            Expr::Dot(a, b) => format!("gage_dot_vec3({}, {})", self.gen_expr(a), self.gen_expr(b)),
            Expr::Cross(a, b) => format!("gage_cross_vec3({}, {})", self.gen_expr(a), self.gen_expr(b)),
            Expr::Length(v) => format!("gage_length_vec3({})", self.gen_expr(v)),
            Expr::Normalize(v) => format!("gage_normalize_vec3({})", self.gen_expr(v)),
            Expr::Binary(left, op, right) => {
                let l = self.gen_expr(left);
                let r = self.gen_expr(right);
                let o = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Mod => "%",
                    BinaryOp::Eq => "==",
                    BinaryOp::Neq => "!=",
                    BinaryOp::Lt => "<",
                    BinaryOp::Lte => "<=",
                    BinaryOp::Gt => ">",
                    BinaryOp::Gte => ">=",
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                };
                format!("({} {} {})", l, o, r)
            }
            Expr::Unary(op, e) => {
                let inner = self.gen_expr(e);
                match op {
                    UnaryOp::Neg => format!("(-{})", inner),
                    UnaryOp::Not => format!("(!{})", inner),
                }
            }
            Expr::Input(p) => {
                if let Some(prompt) = p {
                    format!("gage_input({})", self.gen_expr(prompt))
                } else {
                    "gage_input(\"\")".into()
                }
            }
            Expr::ReadFile(p) => format!("gage_read_file({})", self.gen_expr(p)),
            Expr::WriteFile(p, c) => format!("gage_write_file({}, {})", self.gen_expr(p), self.gen_expr(c)),
        }
    }
}
''')
print("✔ codegen.rs rewritten with class resolution and double array typing")
