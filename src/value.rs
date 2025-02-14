use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use crate::ast::Stmt;


pub type Argument = String;

#[derive(Clone, Debug, PartialEq)]
pub enum LoxType {
    VNumb(f64),
    VStr(String),
    VBool(bool),
    VNil,
    VCallable(String, Vec<Argument>, Box<Stmt>),
}

use LoxType::*;


impl fmt::Display for LoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            VNumb(_) => "Number",
            VStr(_) => "String",
            VBool(_) => "Bool",
            VNil => "Nil",
            VCallable(_,_,_) => "Callable",
        })
    }
}


#[derive(Clone, Debug)]
pub struct LoxValue(Rc<LoxType>);

impl<'a> Deref for LoxValue {
    type Target = LoxType;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl LoxValue {
    pub fn new(t: LoxType) -> LoxValue {
        LoxValue(Rc::new(t))
    }

    pub fn value_string(&self) -> String {
        match &**self {
            VNumb(v) => format!("{}", v),
            VStr(v) => format!("{}", v),
            VBool(v) => format!("{}", v),
            VNil => "nil".to_string(),
            VCallable(name, _, _) => format!("{}", name),
        }
    }

    pub fn _is_truthy(&self) -> bool {
        match &**self {
            VBool(false) | VNil => false,
            _ => true,
        }
    }

    pub fn is_truthy(&self) -> LoxValue {
        match &**self {
            VBool(v) => LoxValue::new(VBool(*v)),
            _ => LoxValue::new(VBool(self._is_truthy())),
        }
    }

    pub fn not(&self) -> Result<LoxValue, String> {
        match &**self {
            VBool(v) => Ok(LoxValue::new(VBool(!v))),
            _ => self.is_truthy().not(),
        }
    }

    pub fn negate(&self) -> Result<LoxValue, String> {
        match &**self {
            VNumb(v) => Ok(LoxValue::new(VNumb(-v))),
            typ => Err(format!("Cannot negate {}", typ)),
        }
    }

    pub fn sub(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VNumb(a - b))),
            (a, b) => Err(format!("Cannot subtract {} from {}", a, b)),
        }
    }

    pub fn add(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VNumb(a + b))),
            (VStr(a), VStr(b)) => Ok(LoxValue::new(VStr(a.to_string() + &b))),
            (a, b) => Err(format!("Cannot add {} to {}", a, b)),
        }
    }

    pub fn mul(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VNumb(a * b))),
            (VStr(a), VNumb(b)) => Ok(LoxValue::new(VStr(a.repeat(*b as usize)))),
            (VNumb(a), VStr(b)) => Ok(LoxValue::new(VStr(b.repeat(*a as usize)))),
            (a, b) => Err(format!("Cannot multiply {} by {}", a, b)),
        }
    }

    pub fn div(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VNumb(a / b))),
            (a, b) => Err(format!("Cannot divide {} by {}", a, b)),
        }
    }

    pub fn neq(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VBool(a != b))),
            (VStr(a), VStr(b)) => Ok(LoxValue::new(VBool(a != b))),
            (VBool(a), VBool(b)) => Ok(LoxValue::new(VBool(a != b))),
            _ => Ok(LoxValue::new(VBool(true))),
        }
    }

    pub fn eq(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VBool(a == b))),
            (VStr(a), VStr(b)) => Ok(LoxValue::new(VBool(a == b))),
            (VBool(a), VBool(b)) => Ok(LoxValue::new(VBool(a == b))),
            _ => Ok(LoxValue::new(VBool(false))),
        }
    }

    pub fn gt(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VBool(a > b))),
            (VStr(a), VStr(b)) => Ok(LoxValue::new(VBool(a > b))),
            (VBool(a), VBool(b)) => Ok(LoxValue::new(VBool(a > b))),
            _ => Ok(LoxValue::new(VBool(false))),
        }
    }

    pub fn ge(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VBool(a >= b))),
            (VStr(a), VStr(b)) => Ok(LoxValue::new(VBool(a >= b))),
            (VBool(a), VBool(b)) => Ok(LoxValue::new(VBool(a >= b))),
            _ => Ok(LoxValue::new(VBool(false))),
        }
    }

    pub fn lt(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VBool(a < b))),
            (VStr(a), VStr(b)) => Ok(LoxValue::new(VBool(a < b))),
            (VBool(a), VBool(b)) => Ok(LoxValue::new(VBool(a < b))),
            _ => Ok(LoxValue::new(VBool(false))),
        }
    }

    pub fn le(&self, b: &LoxValue) -> Result<LoxValue, String> {
        match (&**self, &**b) {
            (VNumb(a), VNumb(b)) => Ok(LoxValue::new(VBool(a <= b))),
            (VStr(a), VStr(b)) => Ok(LoxValue::new(VBool(a <= b))),
            (VBool(a), VBool(b)) => Ok(LoxValue::new(VBool(a <= b))),
            _ => Ok(LoxValue::new(VBool(false))),
        }
    }

    pub fn and(&self, b: &LoxValue) -> Result<LoxValue, String> {
        Ok(LoxValue::new(VBool(match &**self {
            VBool(v) => *v,
            _ => self._is_truthy(),
        } && match &**self {
            VBool(v) => *v,
            _ => b._is_truthy(),
        })))
    }

    pub fn or(&self, b: &LoxValue) -> Result<LoxValue, String> {
        Ok(LoxValue::new(VBool(match &**self {
            VBool(v) => *v,
            _ => self._is_truthy(),
        } || match &**self {
            VBool(v) => *v,
            _ => b._is_truthy(),
        })))
    }
}
