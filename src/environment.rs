use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::LoxValue;


#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    env: RefCell<HashMap<String, Option<LoxValue>>>,
    parent: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Rc<Environment> {
        Rc::new(Environment {
            env: RefCell::new(HashMap::new()),
            parent: None,
        })
    }

    pub fn new_child(parent: &Rc<Environment>) -> Rc<Environment> {
        Rc::new(Environment {
            env: RefCell::new(HashMap::new()),
            parent: Some(parent.clone()),
        })
    }

    pub fn var(&self, name: &str, val: Option<LoxValue>) -> Option<LoxValue> {
        self.env.borrow_mut().insert(name.to_string(), val.clone());
        val.clone()
    }

    pub fn lookup(&self, name: &str) -> Result<LoxValue, String> {
        match self.env.borrow().get(name) {
            Some(Some(v)) => return Ok(v.clone()),
            Some(None) =>
                return Err("ValueError: variable used before initialization".to_string()),
            None => (),
        };
        match &self.parent {
            Some(p) => p.lookup(name),
            None => Err(format!("NameError: {} not declared", name)),
        }
    }

    pub fn assign(&self, name: &str, val: LoxValue) -> Result<LoxValue, String> {
        let has = self.env.borrow().contains_key(name);
        match has {
            true => Ok(self.var(name, Some(val)).unwrap()),
            false => match &self.parent {
                Some(p) => p.assign(name, val),
                None => Err(format!("{} not declared", name)),
            },
        }
    }
}
