#![allow(unused_variables, dead_code)]

use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub struct Variable {
    type_name: String,
    name: String,
    expression: Expression,
	cell: usize
}

impl Variable {
    fn new(type_name: String, name: String, expression: Expression, cell: usize) -> Variable {
        Variable {
            type_name,
            name,
            expression,
			cell
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
	pub parent: Option<Box<Scope>>,
	
	pub variables: HashMap<String, Variable>
}

impl Scope {
	pub fn new() -> Scope {
		Scope {
			parent: None,
			
			variables: HashMap::new()
		}
	}

	pub fn define(&mut self, var: &Variable) {
		self.variables.insert(var.name.clone(), var.clone());
	}

	pub fn get(&self, name: &String) -> Option<&Variable> {
		let var = self.variables.get(name);

		if var.is_some() {
			return var;
		}
		
		if let Some(parent) = &self.parent {
			return parent.get(name);
		}

		return None;
	}

	pub fn get_mut(&mut self, name: &String) -> Option<&mut Variable> {
		let var = self.variables.get_mut(name);
		
		if var.is_some() {
			return var;
		}
		
		if let Some(parent) = &mut self.parent {
			return parent.get_mut(name);
		}

		return None;
	}

	pub fn create_child(&mut self) -> Scope {
		let mut child = Scope::new();

		child.parent = Some(Box::new(self.clone()));

		return child;
	}
}
		

pub struct Compiler<'a> {
	pub gen: Generator,

	pub scope: Scope,

	pub parser: Parser<'a>,

	pub output: String
}

impl<'a> Compiler<'a> {
	pub fn new(source: &[u8]) -> Compiler {
		return Compiler {
			gen: Generator::new(),

			scope: Scope::new(),

			parser: Parser::new(Tokenizer::new(source)),

			output: String::new()
		};
	}

	pub fn compile_expression(&mut self, branch: &mut BFBlock, expr: Expression) -> usize {
		Debug::log(&format!("Compiling expression: {}", expr.stringify()));
		
		if expr.kind == ExpressionType::Binary {
			match expr.operator.as_ref().unwrap().kind {
				TokenType::EqualEqual => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let dst = Distance::new(left, right);

					panic!("Not implemented!");
				},

				TokenType::NotEqual => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let mut dst = Distance::new(left, right);

					// self.output += &dst.compile(&mut self.gen);

					// dst.simulate(&mut self.gen);

					// Sum cells so condition will be true if there is any difference
					let sum = Add::new(dst.gt(&mut self.gen), dst.lt(&mut self.gen));

					/*self.output += &sum.compile(&mut self.gen);

					sum.simulate(&mut self.gen);*/

					let result = sum.a;

					branch.add(dst);

					branch.add(sum);

					return result;
				}

				TokenType::GT | TokenType::GTEqual => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let mut dst = Distance::new(left, right);

					// Result
					let gt = dst.gt(&mut self.gen);

					if expr.operator.unwrap().kind != TokenType::GTEqual {
						// Hack because it acts like >= otherwise for some reason
						let tmp = self.gen.memory.alloc(1);
	
						branch.add(Set::new(tmp, 1));
	
						branch.add(Sub::new(left, tmp));
					}

					branch.add(dst);

					return gt;
				}

				TokenType::LT => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let mut dst = Distance::new(left, right);

					// Result
					let lt = dst.lt(&mut self.gen);

					if expr.operator.unwrap().kind != TokenType::GTEqual {
						// Hack because it acts like <= otherwise for some reason
						let tmp = self.gen.memory.alloc(1);
	
						branch.add(Set::new(tmp, 1));
	
						branch.add(Sub::new(right, tmp));
					}

					branch.add(dst);

					return lt;
				}
				
				TokenType::Plus => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());
					
					let add = Add::new(left, right);

					/*self.output += &add.compile(&mut self.gen);

					add.simulate(&mut self.gen);*/

					branch.add(add);

					return left;
				},

				TokenType::Minus => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());
					
					let sub = Sub::new(left, right);

					/*self.output += &sub.compile(&mut self.gen);

					sub.simulate(&mut self.gen);*/

					branch.add(sub);

					return left;
				},

				TokenType::Times => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());
					
					let mut mul = Mul::new(left, right);

					/*self.output += &mul.compile(&mut self.gen);

					mul.simulate(&mut self.gen);*/

					let result = mul.result(&mut self.gen);

					branch.add(mul);

					return result;
				},

				TokenType::Slash => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					println!("Current {}", self.gen.memory.current);
					
					let mut div = Div::new(left, right);

					let result = div.result(&mut self.gen);

					/*self.output += &div.compile(&mut self.gen);

					div.simulate(&mut self.gen);*/

					branch.add(div);

					// div.clean(&mut self.gen);

					return result;
				},

				_ => panic!("Not implemented!")
			}
		}
		
		if expr.kind == ExpressionType::Literal {
			if expr.value.as_ref().unwrap().kind == TokenType::Number {
				let loc = self.gen.memory.alloc(1);

				Debug::log(&format!("Number literal parsed: ({})", expr.value.as_ref().unwrap().number));

				let set = Set::new(loc, expr.value.as_ref().unwrap().number as CellSize);

				/*self.output += &set.compile(&mut self.gen);

				set.simulate(&mut self.gen);*/

				branch.add(set);

				return loc;
			}

			if expr.value.as_ref().unwrap().kind == TokenType::Char {
				let loc = self.gen.memory.alloc(1);

				Debug::log(&format!("Char literal parsed: ({})", expr.value.as_ref().unwrap().char));

				let set = Set::new(loc, expr.value.as_ref().unwrap().char as CellSize);

				/*self.output += &set.compile(&mut self.gen);

				set.simulate(&mut self.gen);*/

				branch.add(set);

				return loc;
			}

			if expr.value.as_ref().unwrap().kind == TokenType::Identifier {
				let token = expr.value.as_ref().unwrap();

				if token.string == "read" {
					let cell = self.gen.memory.alloc(1);
					
					let input = Input::new(cell);

					branch.add(input);

					return cell;
				}

				let var = self.scope.get(&token.string);

				if var.is_none() {
					panic!("Variable not defined!");
				}

				/*let mut updated = var.unwrap().clone();

				updated.cell = self.compile_expression(branch, updated.expression.clone());
				
				self.scope.define(&updated);*/

				let cell = self.gen.memory.alloc(1);

				let tmp = self.gen.memory.alloc(1);

				let copy = Copy::new(cell, tmp, var.unwrap().cell);

				branch.add(copy);

				self.gen.memory.free(tmp);

				return cell;
			}
		}

		panic!("Not implemented!");
	}

	pub fn compile_statement(&mut self, branch: &mut BFBlock, stmt: Statement) {
		if stmt.kind == StatementType::If {
			let condition = self.compile_expression(branch, stmt.condition.unwrap());

			let mut check = If::new(condition);

			let current = self.gen.clone();

			// check.block.
			
			for stmt in stmt.block.unwrap() {
				self.compile_statement(&mut check.block, stmt);
			}

			branch.add(check);

			return;
		}

		if stmt.kind == StatementType::While {
			let condition = self.compile_expression(branch, stmt.condition.clone().unwrap());

			let mut check = While::new(condition);

			let current = self.gen.clone();

			// check.block.
			
			for stmt in stmt.block.unwrap() {
				self.compile_statement(&mut check.block, stmt);
			}
			
			let newcondition = self.compile_expression(&mut check.block, stmt.condition.clone().unwrap());

			check.block.add(Move::new(condition, newcondition));

			branch.add(check);

			return;
		}

		if stmt.kind == StatementType::VarDecl {
			let expr = stmt.expression.unwrap();

			let cell = self.compile_expression(branch, expr.clone());

			Debug::log(&format!("Defined {} as {}", stmt.name.as_ref().unwrap().string, cell));
			
			self.scope.define(&Variable::new(stmt.type_name.unwrap().string, stmt.name.unwrap().string, expr, cell));

			return;
		}

		if stmt.kind == StatementType::Assignment {
			let expr = stmt.expression.unwrap();

			let cell = self.compile_expression(branch, expr.clone());
			
			let var = self.scope.get_mut(&stmt.name.as_ref().unwrap().string).unwrap();

			Debug::log(&format!("Assigning {}({}) = {}", stmt.name.as_ref().unwrap().string, var.cell, cell));
			
			let mv = Move::new(var.cell, cell);

			branch.add(mv);

			return;
		}

		if stmt.kind == StatementType::Expression {
			self.compile_expression(branch, stmt.expression.unwrap());

			return;
		}

		if stmt.kind == StatementType::Print {
			let output = Output::new(self.compile_expression(branch, stmt.expression.clone().unwrap()));
			
			branch.add(output);
		}
	}

	pub fn compile(&mut self) -> String {
		// let mut str = String::new();

		let mut branch = BFBlock::new();

		while self.parser.tokenizer.peek_token().kind != TokenType::EndOfFile {
			let stmt = self.parser.statement();
			
			self.compile_statement(&mut branch, stmt);
		}

		let mut str = String::new();

		for mut instr in branch.instructions {
			str += &instr.compile(&mut self.gen);

			instr.simulate(&mut self.gen);
		}

		return str;
	}
}