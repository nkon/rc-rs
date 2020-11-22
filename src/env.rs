use std::collections::HashMap;

pub struct Env<'a> {
    pub constant: HashMap<&'a str, f64>,
    pub var: HashMap<&'a str, f64>,
    pub func: HashMap<&'a str, (fn(), isize)>, // (function pointer, arg num)
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            constant: HashMap::new(),
            var: HashMap::new(),
            func: HashMap::new(),
        }
    }

    pub fn built_in(&mut self) {
        self.constant.insert("pi", std::f64::consts::PI);
        self.constant.insert("e", std::f64::consts::E);
        self.constant.insert("eps", std::f64::EPSILON);
    }

    pub fn is_const(&mut self, key: &str) -> Option<f64> {
        match self.constant.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }
}

impl Default for Env<'_> {
    fn default() -> Self {
        Self::new()
    }
}
