use std::cell::RefCell;
use std::collections::HashMap;

pub type Environment = HashMap<String, i32>;

pub struct Scope {
    env: RefCell<HashMap<String, i32>>,
}
