use std::fmt;


#[derive(Clone, Debug, PartialEq)]
pub enum LoxValue {
    VNumb(f64),
    VStr(String),
    VBool(bool),
    VNil,
    VUninitialized,
}

use LoxValue::*;

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            VNumb(_) => "Number",
            VStr(_) => "String",
            VBool(_) => "Bool",
            VNil => "Nil",
            VUninitialized => "<!>",
        })
    }
}

impl LoxValue {
    pub fn value_string(&self) -> String {
        match self {
            VNumb(v) => format!("{}", v),
            VStr(v) => format!("{}", v),
            VBool(v) => format!("{}", v),
            VNil => "nil".to_string(),
            VUninitialized => "<1>".to_string(),
        }
    }

    pub fn _is_truthy(&self) -> bool {
        match self {
            VBool(false) | VNil => false,
            _ => true,
        }
    }

    pub fn is_truthy(&self) -> LoxValue {
        match self {
            VBool(v) => VBool(*v),
            _ => VBool(self._is_truthy()),
        }
    }

    pub fn not(&self) -> Result<LoxValue, String> {
        match self {
            VBool(v) => Ok(VBool(!v)),
            _ => self.is_truthy().not(),
        }
    }

    pub fn negate(&self) -> Result<LoxValue, String> {
        match self {
            VNumb(v) => Ok(VNumb(-v)),
            _ => Err(format!("Cannot negate {}", self)),
        }
    }

    pub fn sub(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VNumb(a - b)),
            (a, b) => Err(format!("Cannot subtract {} from {}", *a, b)),
        }
    }

    pub fn add(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VNumb(a + b)),
            (VStr(a), VStr(b)) => Ok(VStr(a.to_string() + &b)),
            (a, b) => Err(format!("Cannot add {} to {}", a, b)),
        }
    }

    pub fn mul(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VNumb(a * b)),
            (VStr(a), VNumb(b)) => Ok(VStr(a.repeat(*b as usize))),
            (VNumb(a), VStr(b)) => Ok(VStr(b.repeat(*a as usize))),
            (a, b) => Err(format!("Cannot multiply {} by {}", a, b)),
        }
    }

    pub fn div(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VNumb(a / b)),
            (a, b) => Err(format!("Cannot divide {} by {}", a, b)),
        }
    }

    pub fn neq(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VBool(a != b)),
            (VStr(a), VStr(b)) => Ok(VBool(a != b)),
            (VBool(a), VBool(b)) => Ok(VBool(a != b)),
            _ => Ok(VBool(true)),
        }
    }

    pub fn eq(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VBool(a == b)),
            (VStr(a), VStr(b)) => Ok(VBool(a == b)),
            (VBool(a), VBool(b)) => Ok(VBool(a == b)),
            _ => Ok(VBool(false)),
        }
    }

    pub fn gt(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VBool(a > b)),
            (VStr(a), VStr(b)) => Ok(VBool(a > b)),
            (VBool(a), VBool(b)) => Ok(VBool(a > b)),
            _ => Ok(VBool(false)),
        }
    }

    pub fn ge(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VBool(a >= b)),
            (VStr(a), VStr(b)) => Ok(VBool(a >= b)),
            (VBool(a), VBool(b)) => Ok(VBool(a >= b)),
            _ => Ok(VBool(false)),
        }
    }

    pub fn lt(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VBool(a < b)),
            (VStr(a), VStr(b)) => Ok(VBool(a < b)),
            (VBool(a), VBool(b)) => Ok(VBool(a < b)),
            _ => Ok(VBool(false)),
        }
    }

    pub fn le(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (self, b) {
            (VNumb(a), VNumb(b)) => Ok(VBool(a <= b)),
            (VStr(a), VStr(b)) => Ok(VBool(a <= b)),
            (VBool(a), VBool(b)) => Ok(VBool(a <= b)),
            _ => Ok(VBool(false)),
        }
    }

    pub fn and(&self, b: &LoxValue) -> Result<LoxValue, String> {
        Ok(VBool(match self {
            VBool(v) => *v,
            _ => self._is_truthy(),
        } && match self {
            VBool(v) => *v,
            _ => b._is_truthy(),
        }))
    }

    pub fn or(&self, b: &LoxValue) -> Result<LoxValue, String> {
        Ok(VBool(match self {
            VBool(v) => *v,
            _ => self._is_truthy(),
        } || match self {
            VBool(v) => *v,
            _ => b._is_truthy(),
        }))
    }
}
