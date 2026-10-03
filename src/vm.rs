use crate::bytecode::{Chunk, OpCode, Value};
use std::collections::HashMap;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum VMError {
    RuntimeError(String),
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<Value, VMError> {
        loop {
            if self.ip >= self.chunk.code.len() {
                return Ok(Value::Nil);
            }

            let instruction = self.chunk.code[self.ip].clone();
            self.ip += 1;

            match instruction {
                OpCode::Constant(idx) => {
                    let val = self.chunk.constants[idx].clone();
                    self.stack.push(val);
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),

                OpCode::MakeVec2 => {
                    let y = self.pop_as_float()?;
                    let x = self.pop_as_float()?;
                    self.stack.push(Value::Vec2(x, y));
                }
                OpCode::MakeVec3 => {
                    let z = self.pop_as_float()?;
                    let y = self.pop_as_float()?;
                    let x = self.pop_as_float()?;
                    self.stack.push(Value::Vec3(x, y, z));
                }
                OpCode::MakeVec4 => {
                    let w = self.pop_as_float()?;
                    let z = self.pop_as_float()?;
                    let y = self.pop_as_float()?;
                    let x = self.pop_as_float()?;
                    self.stack.push(Value::Vec4(x, y, z, w));
                }

                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x + y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x + y)),
                        (Value::Int(x), Value::Float(y)) => self.stack.push(Value::Float(x as f64 + y)),
                        (Value::Float(x), Value::Int(y)) => self.stack.push(Value::Float(x + y as f64)),
                        (Value::Str(x), Value::Str(y)) => self.stack.push(Value::Str(format!("{}{}", x, y))),
                        // Simulation vector additions
                        (Value::Vec2(x1, y1), Value::Vec2(x2, y2)) => {
                            self.stack.push(Value::Vec2(x1 + x2, y1 + y2))
                        }
                        (Value::Vec3(x1, y1, z1), Value::Vec3(x2, y2, z2)) => {
                            self.stack.push(Value::Vec3(x1 + x2, y1 + y2, z1 + z2))
                        }
                        (Value::Vec4(x1, y1, z1, w1), Value::Vec4(x2, y2, z2, w2)) => {
                            self.stack.push(Value::Vec4(x1 + x2, y1 + y2, z1 + z2, w1 + w2))
                        }
                        (v1, v2) => {
                            return Err(VMError::RuntimeError(format!(
                                "Invalid operands for '+': {} and {}",
                                v1, v2
                            )))
                        }
                    }
                }

                OpCode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x - y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x - y)),
                        (Value::Int(x), Value::Float(y)) => self.stack.push(Value::Float(x as f64 - y)),
                        (Value::Float(x), Value::Int(y)) => self.stack.push(Value::Float(x - y as f64)),
                        (Value::Vec2(x1, y1), Value::Vec2(x2, y2)) => {
                            self.stack.push(Value::Vec2(x1 - x2, y1 - y2))
                        }
                        (Value::Vec3(x1, y1, z1), Value::Vec3(x2, y2, z2)) => {
                            self.stack.push(Value::Vec3(x1 - x2, y1 - y2, z1 - z2))
                        }
                        (Value::Vec4(x1, y1, z1, w1), Value::Vec4(x2, y2, z2, w2)) => {
                            self.stack.push(Value::Vec4(x1 - x2, y1 - y2, z1 - z2, w1 - w2))
                        }
                        (v1, v2) => {
                            return Err(VMError::RuntimeError(format!(
                                "Invalid operands for '-': {} and {}",
                                v1, v2
                            )))
                        }
                    }
                }

                OpCode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x * y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x * y)),
                        (Value::Int(x), Value::Float(y)) => self.stack.push(Value::Float(x as f64 * y)),
                        (Value::Float(x), Value::Int(y)) => self.stack.push(Value::Float(x * y as f64)),
                        // Vector * Scalar scaling
                        (Value::Vec2(x, y), Value::Float(s)) | (Value::Float(s), Value::Vec2(x, y)) => {
                            self.stack.push(Value::Vec2(x * s, y * s))
                        }
                        (Value::Vec3(x, y, z), Value::Float(s)) | (Value::Float(s), Value::Vec3(x, y, z)) => {
                            self.stack.push(Value::Vec3(x * s, y * s, z * s))
                        }
                        (Value::Vec4(x, y, z, w), Value::Float(s)) | (Value::Float(s), Value::Vec4(x, y, z, w)) => {
                            self.stack.push(Value::Vec4(x * s, y * s, z * s, w * s))
                        }
                        (v1, v2) => {
                            return Err(VMError::RuntimeError(format!(
                                "Invalid operands for '*': {} and {}",
                                v1, v2
                            )))
                        }
                    }
                }

                OpCode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => {
                            if y == 0 {
                                return Err(VMError::RuntimeError("Division by zero".to_string()));
                            }
                            self.stack.push(Value::Int(x / y));
                        }
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x / y)),
                        (Value::Vec2(x, y), Value::Float(s)) => self.stack.push(Value::Vec2(x / s, y / s)),
                        (Value::Vec3(x, y, z), Value::Float(s)) => {
                            self.stack.push(Value::Vec3(x / s, y / s, z / s))
                        }
                        (v1, v2) => {
                            return Err(VMError::RuntimeError(format!(
                                "Invalid operands for '/': {} and {}",
                                v1, v2
                            )))
                        }
                    }
                }

                OpCode::Negate => {
                    let val = self.pop()?;
                    match val {
                        Value::Int(n) => self.stack.push(Value::Int(-n)),
                        Value::Float(f) => self.stack.push(Value::Float(-f)),
                        _ => return Err(VMError::RuntimeError("Operand must be a number".to_string())),
                    }
                }

                OpCode::Not => {
                    let val = self.pop()?;
                    self.stack.push(Value::Bool(!val.is_truthy()));
                }

                OpCode::Equal => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Bool(a == b));
                }

                OpCode::NotEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Bool(a != b));
                }

                OpCode::Less => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x < y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x < y)),
                        _ => return Err(VMError::RuntimeError("Operands must be numbers".to_string())),
                    }
                }

                OpCode::LessEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x <= y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x <= y)),
                        _ => return Err(VMError::RuntimeError("Operands must be numbers".to_string())),
                    }
                }

                OpCode::Greater => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x > y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x > y)),
                        _ => return Err(VMError::RuntimeError("Operands must be numbers".to_string())),
                    }
                }

                OpCode::GreaterEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x >= y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x >= y)),
                        _ => return Err(VMError::RuntimeError("Operands must be numbers".to_string())),
                    }
                }

                OpCode::Pop => {
                    self.pop()?;
                }

                OpCode::DefineGlobal(name) => {
                    let val = self.pop()?;
                    self.globals.insert(name, val);
                }

                OpCode::GetGlobal(name) => {
                    if let Some(val) = self.globals.get(&name) {
                        self.stack.push(val.clone());
                    } else {
                        return Err(VMError::RuntimeError(format!("Undefined global variable '{}'", name)));
                    }
                }

                OpCode::SetGlobal(name) => {
                    let val = self.peek()?.clone();
                    if self.globals.contains_key(&name) {
                        self.globals.insert(name, val);
                    } else {
                        return Err(VMError::RuntimeError(format!("Cannot assign to undefined global '{}'", name)));
                    }
                }

                OpCode::GetLocal(idx) => {
                    let val = self.stack[idx].clone();
                    self.stack.push(val);
                }

                OpCode::SetLocal(idx) => {
                    let val = self.peek()?.clone();
                    self.stack[idx] = val;
                }

                OpCode::Jump(target) => {
                    self.ip = target;
                }

                OpCode::JumpIfFalse(target) => {
                    let val = self.peek()?;
                    if !val.is_truthy() {
                        self.ip = target;
                    }
                }

                OpCode::Loop(target) => {
                    self.ip = target;
                }

                OpCode::Print => {
                    let val = self.pop()?;
                    print!("{}", val);
                }

                OpCode::Println => {
                    let val = self.pop()?;
                    println!("{}", val);
                }

                OpCode::Return => {
                    return Ok(self.pop().unwrap_or(Value::Nil));
                }

                OpCode::Halt => {
                    return Ok(Value::Nil);
                }
            }
        }
    }

    fn pop(&mut self) -> Result<Value, VMError> {
        self.stack
            .pop()
            .ok_or_else(|| VMError::RuntimeError("Stack underflow".to_string()))
    }

    fn peek(&self) -> Result<&Value, VMError> {
        self.stack
            .last()
            .ok_or_else(|| VMError::RuntimeError("Stack underflow on peek".to_string()))
    }

    fn pop_as_float(&mut self) -> Result<f64, VMError> {
        let val = self.pop()?;
        match val {
            Value::Float(f) => Ok(f),
            Value::Int(i) => Ok(i as f64),
            _ => Err(VMError::RuntimeError("Expected floating-point number".to_string())),
        }
    }
}
