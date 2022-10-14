use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ast::ASTNode;

pub struct NumberNode {
    value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl ASTNode for NumberNode {
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        Ok(self.value)
    }
}
