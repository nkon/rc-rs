use super::*;
use std::collections::HashMap;

pub type TypeFn = fn(&mut Env, &[Node]) -> f64;

pub struct Env<'a> {
    pub constant: HashMap<&'a str, f64>,
    pub func: HashMap<&'a str, (TypeFn, usize)>, // (function pointer, arg num: 0=variable)
    pub debug: bool,
    // TODO: imprement command and status.
}

// warp all functions
fn impl_sin(env: &mut Env, arg: &[Node]) -> f64 {
    eval_fvalue(env, &arg[0]).sin()
}

fn impl_abs(env: &mut Env, arg: &[Node]) -> f64 {
    eval_fvalue(env, &arg[0]).abs()
}

fn impl_max(env: &mut Env, arg: &[Node]) -> f64 {
    if arg.is_empty() {
        return 0.0;
    }
    let mut max = eval_fvalue(env, &arg[0]);
    for i in arg {
        if max < eval_fvalue(env, &i) {
            max = eval_fvalue(env, &i);
        }
    }
    max
}

fn impl_ave(env: &mut Env, arg: &[Node]) -> f64 {
    if arg.is_empty() {
        return 0.0;
    }
    let mut sum: f64 = 0.0;
    for i in arg {
        sum += eval_fvalue(env, &i);
    }
    sum / arg.len() as f64
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            constant: HashMap::new(),
            func: HashMap::new(),
            debug: false,
        }
    }

    pub fn built_in(&mut self) {
        self.constant.insert("pi", std::f64::consts::PI);
        self.constant.insert("e", std::f64::consts::E);
        self.constant.insert("eps", std::f64::EPSILON);
        self.func.insert("sin", (impl_sin as TypeFn, 1));
        self.func.insert("abs", (impl_abs as TypeFn, 1));
        self.func.insert("max", (impl_max as TypeFn, 0));
        self.func.insert("ave", (impl_ave as TypeFn, 0));
    }

    pub fn is_const(&mut self, key: &str) -> Option<f64> {
        match self.constant.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }

    pub fn is_func(&mut self, key: &str) -> Option<(TypeFn, usize)> {
        match self.func.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }

    pub fn set_debug(&mut self, flag: bool) {
        self.debug = flag;
    }
    pub fn is_debug(&self) -> bool {
        self.debug
    }
}

impl Default for Env<'_> {
    fn default() -> Self {
        Self::new()
    }
}
