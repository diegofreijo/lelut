use std::{rc::Rc, fmt::{Display}, result::Result};

use crate::vm::InterpretResult;


#[derive(Debug)]
pub struct  ObjString {
    value: String,
}

impl ObjString {
    pub fn from(value: &str) -> Self {
        ObjString {
            value: String::from(value)
        }
    }

	pub fn from_owned(value: String) -> Self {
        ObjString {
            value: value
        }
    }

	pub fn value(&self) -> &String {
		&self.value
	}
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(Rc<ObjString>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0.value == r0.value,
            _ => false,
        }
    }
}

impl Value {
	pub fn new_string(value: &str) -> Self {
		Value::String(
			Rc::from(ObjString::from(value))
		)
	}

    pub fn expect_number(&self) -> Result<&f64, InterpretResult> {
        match self {
            Value::Number(n) => Ok(n),
            other => Err(InterpretResult::RuntimeError(format!("Expected a number, found {:?}", other))),
        }
    }

}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => f.write_str("nil"),
            Value::Boolean(b) => f.write_str(&b.to_string()),
            Value::Number(n) => f.write_str(&n.to_string()),
            Value::String(obj) => f.write_str(&obj.value),
        }
    }
}

impl Value {
 
}

// impl InterpreterValidator for Option<Value> {
//     fn expect_number(&self) {
        
//     }
// }
