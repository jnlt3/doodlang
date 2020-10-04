use std::collections::HashMap;
use crate::doodlang::parser::FunctionManager;

pub trait Function {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64;
}

pub struct Variable {
    pub id: usize,
}

impl Function for Variable {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        eq.variables[self.id]
    }
}

pub struct Constant {
    pub val: f64,
}

impl Function for Constant {
    fn output(&mut self, _eq: &Box<FunctionManager>) -> f64 {
        self.val
    }
}


pub struct Sum {
    pub left: Box<dyn Function>,
    pub right: Box<dyn Function>,
}

impl Function for Sum {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.left.output(eq) + self.right.output(eq);
    }
}


pub struct Sub {
    pub left: Box<dyn Function>,
    pub right: Box<dyn Function>,
}

impl Function for Sub {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.left.output(eq) - self.right.output(eq);
    }
}


pub struct Mul {
    pub left: Box<dyn Function>,
    pub right: Box<dyn Function>,
}

impl Function for Mul {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.left.output(eq) * self.right.output(eq);
    }
}

pub struct Div {
    pub left: Box<dyn Function>,
    pub right: Box<dyn Function>,
}

impl Function for Div {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.left.output(eq) / self.right.output(eq);
    }
}

pub struct Pow {
    pub base: Box<dyn Function>,
    pub power: Box<dyn Function>,
}

impl Function for Pow {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.base.output(eq).powf(self.power.output(eq));
    }
}

pub struct Log {
    pub base: Box<dyn Function>,
    pub input: Box<dyn Function>,
}

impl Function for Log {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.input.output(eq).log(self.base.output(eq));
    }
}

pub struct Ln {
    pub input: Box<dyn Function>,
}

impl Function for Ln {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.input.output(eq).ln();
    }
}

pub struct Sine {
    pub input: Box<dyn Function>,
}

impl Function for Sine {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.input.output(eq).sin();
    }
}

pub struct Cosine {
    pub input: Box<dyn Function>,
}

impl Function for Cosine {
    fn output(&mut self, eq: &Box<FunctionManager>) -> f64 {
        return self.input.output(eq).cos();
    }
}
