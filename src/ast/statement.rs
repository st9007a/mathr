use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTNode, ASTSemanticAnalysis, AssignNode};

pub struct StatementListNode {
    nodes: Vec<Box<AssignNode>>,
}

impl StatementListNode {
    pub fn new(nodes: Vec<Box<AssignNode>>) -> Self {
        Self { nodes }
    }
}

impl ASTNode for StatementListNode {
    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        let mut value: f64 = 0.;

        for node in self.nodes.iter() {
            value = node.eval(symtab)?;
        }

        Ok(value)
    }
}

impl ASTSemanticAnalysis for StatementListNode {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        for node in self.nodes.iter() {
            let res = node.check_semantic(symtab);

            if res.is_err() {
                return res;
            }
        }

        Ok(())
    }
}
