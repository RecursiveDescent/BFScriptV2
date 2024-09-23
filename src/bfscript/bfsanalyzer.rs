#![allow(unused_variables, dead_code)]

use std::borrow::BorrowMut;

use super::*;

#[derive(Clone, Debug)]
pub struct ValueInfo {
    pub type_name: String,
    pub size: usize,
}

impl ValueInfo {
    pub fn new(type_name: String, size: usize) -> ValueInfo {
        return ValueInfo { type_name, size };
    }

    pub fn default() -> ValueInfo {
        return ValueInfo { type_name: "Unknown".to_string(), size: 0 };
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: ValueInfo
}

impl Variable {
    pub fn new(name: String, value: ValueInfo) -> Variable {
        return Variable { name, value: value };
    }
}

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<Box<Scope>>,
    
    pub variables: Vec<Variable>
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Scope {
        return Scope {
            parent: match parent { Some(parent) => Some(Box::new(parent)), _ => None },
            variables: Vec::new()
        };
    }

    pub fn new_child() -> Scope {
        return Scope {
            parent: None,
            variables: Vec::new()
        };
    }

    pub fn set(&mut self, name: String, value: ValueInfo) {
        for variable in self.variables.iter_mut() {
            if variable.name == name {
                variable.value = value;

                return;
            }
        }

        self.variables.push(Variable { name, value });
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        for variable in self.variables.iter() {
            if variable.name == name {
                return Some(variable);
            }
        }

        return None;
    }
}

pub struct Analyzer<'a> {
    pub parser: Parser<'a>,

    pub scope: Scope
}

impl<'a> Analyzer<'a> {
    pub fn new(source: &[u8]) -> Analyzer {
        return Analyzer {
            parser: Parser::new(Tokenizer::new(source)),
            scope: Scope::new(None)
        };
    }

    pub fn analyze(&mut self) {
        while self.parser.tokenizer.peek_token().kind != TokenType::EndOfFile {
            let stmt = self.parser.statement();

            self.analyze_stmt(stmt);
        }
    }

    pub fn analyze_stmt(&mut self, stmt: Statement) {
        match stmt.kind {
            StatementType::VarDecl => {
                let expr = stmt.expression.as_ref().unwrap();

                let value = self.analyze_expr(expr.clone());

                self.scope.set(stmt.name.unwrap().string, value);
            }

            StatementType::Assignment => {
                let expr = stmt.expression.as_ref().unwrap();

                let value = self.analyze_expr(expr.clone());

                let variable = self.scope.get(&stmt.name.as_ref().unwrap().string);

                if variable.is_none() {
                    panic!("Variable {} not found", stmt.name.unwrap().string);
                }

                let variable = variable.unwrap();

                if variable.value.type_name != value.type_name {
                    // Temporary panic
                    panic!("Type mismatch: {} != {}", variable.value.type_name, value.type_name);
                }

                // replace variable

                self.scope.set(stmt.name.unwrap().string, value);
            }

            _ => {
                
            }
        }
    }

    pub fn analyze_expr(&mut self, expr: Expression) -> ValueInfo {
        if expr.kind == ExpressionType::Binary {
            let left = self.analyze_expr(expr.left.unwrap().as_ref().clone()).clone();
            let right = self.analyze_expr(expr.right.unwrap().as_ref().clone()).clone();

            return ValueInfo::new(left.type_name, left.size);
        }

        if expr.kind == ExpressionType::Call {
            let target = expr.target.as_ref().unwrap();

            // hardcoded for now
            if target.string == "open" || target.string == "write" {
                return ValueInfo::new("int".to_string(), 1);
            }

            if target.string == "read" {
                return ValueInfo::new("string".to_string(), expr.args.unwrap().get(0).as_ref().unwrap().value.as_ref().unwrap().number as usize);
            }

            return ValueInfo::default();
        }

        if expr.kind == ExpressionType::Literal {
            let token = expr.value.as_ref().unwrap();


            match token.kind {
                TokenType::String => {
                    return ValueInfo::new("string".to_string(), token.string.len());
                }

                TokenType::Char => {
                    return ValueInfo::new("char".to_string(), 1);
                }

                TokenType::Number => {
                    return ValueInfo::new("int".to_string(), 1);
                }

                TokenType::Identifier => {
                    return self.scope.get(&token.string).unwrap().value.clone();
                }

                _ => {
                    panic!("Unexpected literal type");
                }
            }
        }

        return ValueInfo::default();
    }
}