use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::LoxValue;


pub struct Environment<'a> {
    env: RefCell<HashMap<String, Rc<LoxValue>>>,
    parent: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(parent: Option<&'a Environment>) -> Environment<'a> {
        Environment {
            env: RefCell::new(HashMap::new()),
            parent,
        }
    }

    pub fn new_child(&'a self) -> Environment<'a> {
        Environment::new(Some(self))
    }

    pub fn var(&self, name: &str, val: Rc<LoxValue>) -> Rc<LoxValue> {
        self.env.borrow_mut().insert(name.to_string(), Rc::clone(&val));
        Rc::clone(&val)
    }

    pub fn lookup(&self, name: &str) -> Result<Rc<LoxValue>, String> {
        {
            let env = self.env.borrow();
            if let Some(v) = env.get(name) {
                return Ok(Rc::clone(&v));
            };
        }
        match self.parent {
            Some(p) => p.lookup(name),
            None => Err(format!("{} not declared", name)),
        }
    }

    pub fn assign(&self, name: &str, val: Rc<LoxValue>) -> Result<Rc<LoxValue>, String> {
        let has = self.env.borrow().contains_key(name);
        match has {
            true => Ok(self.var(name, val)),
            false => match self.parent {
                Some(p) => p.assign(name, val),
                None => Err(format!("{} not declared", name)),
            },
        }
    }
}
