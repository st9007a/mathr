use crate::ast::{ASTStatement, StatementListNode, ASTSemanticAnalysis};
use crate::error::InterpreterError;
use crate::parser::Parser;
use crate::symbol_table::SymbolTable;
use crate::tokenizer::Tokenizer;

pub struct Interpreter {
    symtab: SymbolTable,
    semantic_symtab: SymbolTable,
    nodes: Vec<Box<StatementListNode>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            symtab: SymbolTable::new(),
            semantic_symtab: SymbolTable::new(),
            nodes: vec![],
        }
    }

    pub fn interpret(&mut self, content: &str) -> Result<f64, InterpreterError> {
        let tokens = Tokenizer::new(content).try_collect()?;
        let statement_list_node = Parser::new(tokens).parse()?;

        statement_list_node.check_semantic(&mut self.semantic_symtab)?;

        let value = statement_list_node.execute(&mut self.symtab)?;
        self.nodes.push(statement_list_node);

        Ok(value)
    }

    pub fn clear_state(&mut self) {
        self.nodes.clear();
        self.symtab.clear();
    }

    pub fn query(&self, symbol: &String) -> Option<&f64> {
        self.symtab.get(symbol)
    }
}
