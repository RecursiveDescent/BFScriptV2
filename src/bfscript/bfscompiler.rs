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

	pub output: String,

	pub analyzer: Analyzer<'a>,

	pub last_return: ValueInfo
}

impl<'a> Compiler<'a> {
	pub fn new(source: &[u8]) -> Compiler {
		return Compiler {
			gen: Generator::new(),

			scope: Scope::new(),

			parser: Parser::new(Tokenizer::new(source)),

			output: String::new(),

			analyzer: Analyzer::new(source),

			last_return: ValueInfo::default()
		};
	}

	pub fn compile_expression(&mut self, branch: &mut BFBlock, expr: Expression) -> usize {
		Debug::log(&format!("Compiling expression: {}", expr.stringify()));
		
		if expr.kind == ExpressionType::Binary {
			match expr.operator.as_ref().unwrap().kind {
				TokenType::EqualEqual => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let sub = Sub::new(left, right);

					let negate = BoolNegate::new(sub.a, self.gen.memory.alloc(1));

					let result = negate.a;

					branch.add(sub);

					branch.add(negate);

					return result;
				},

				TokenType::NotEqual => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let sub = Sub::new(left, right);

					// self.output += &dst.compile(&mut self.gen);

					// dst.simulate(&mut self.gen);
					/*self.output += &sum.compile(&mut self.gen);

					sum.simulate(&mut self.gen);*/

					let result = sub.a;

					branch.add(sub);

					return result;
				}

				TokenType::GT | TokenType::GTEqual => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let mut dst = Distance::new(left, right);

					// Result
					let gt = dst.gt(&mut self.gen);

					// This might not be a good long term solution, consider using BoolNegate if this fails
					if expr.operator.unwrap().kind != TokenType::GTEqual {
						// Hack because it acts like >= otherwise for some reason
						let tmp = self.gen.memory.alloc(1);
	
						branch.add(Set::new(tmp, 1));
	
						branch.add(Sub::new(left, tmp));

						branch.add(dst);

						return gt;
					}
					
					let lt = dst.lt(&mut self.gen);

					// If not greater, make the cell a 1 and add it to the result so the condition is true if equal
					let negate = BoolNegate::new(lt, self.gen.memory.alloc(1));

					let na = negate.a;

					branch.add(dst);

					branch.add(negate);

					// Add other distance to test if equal
					branch.add(Add::new(gt, na));

					return gt;
				}

				TokenType::LT | TokenType::LTEqual => {
					let left = self.compile_expression(branch, *expr.left.unwrap());

					let right = self.compile_expression(branch, *expr.right.unwrap());

					let mut dst = Distance::new(left, right);

					// Result
					let lt = dst.lt(&mut self.gen);

					// This might not be a good long term solution, consider using BoolNegate if this fails
					if expr.operator.unwrap().kind != TokenType::LTEqual {
						// Hack because it acts like <= otherwise for some reason
						let tmp = self.gen.memory.alloc(1);
	
						branch.add(Set::new(tmp, 1));
	
						branch.add(Sub::new(right, tmp));

						branch.add(dst);

						return lt;
					}
				
					let gt = dst.gt(&mut self.gen);

					// If not greater, make the cell a 1 and add it to the result so the condition is true if equal
					let negate = BoolNegate::new(gt, self.gen.memory.alloc(1));

					let na = negate.a;

					branch.add(dst);

					branch.add(negate);

					// Add other distance to test if equal
					branch.add(Add::new(lt, na));	

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

			if expr.value.as_ref().unwrap().kind == TokenType::String {
				let value = expr.value.as_ref().unwrap().string.clone();

				let loc = self.gen.memory.alloc(value.len() + 1);

				Debug::log(&format!("String literal parsed: ({})", expr.value.as_ref().unwrap().string));

				for (i, c) in value.chars().enumerate() {
					let set = Set::new(loc + i, c as CellSize);

					branch.add(set);
				}

				let set = Set::new(loc + value.len(), 0);

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

				let var = self.scope.get(&token.string);

				if var.is_none() {
					panic!("Variable not defined!");
				}

				/*let mut updated = var.unwrap().clone();

				updated.cell = self.compile_expression(branch, updated.expression.clone());
				
				self.scope.define(&updated);*/

				let info = self.analyzer.scope.get(&token.string).unwrap().value.clone();

				let cell = self.gen.memory.alloc(info.size);

				let tmp = self.gen.memory.alloc(1);

				for i in 0..info.size {
					let copy = Copy::new(cell + i, tmp, var.unwrap().cell + i);

					branch.add(copy);
				}

				self.gen.memory.free(tmp);

				return cell;
			}
		}

		if expr.kind == ExpressionType::Call {
			if expr.target.as_ref().unwrap().string == "print" {
				let arg = self.compile_expression(branch, expr.args.as_ref().unwrap().get(0).unwrap().clone());

				let info = self.analyzer.analyze_expr(expr.args.as_ref().unwrap().get(0).unwrap().clone());

				for i in 0..info.size {
					let output = Output::new(arg + i);
				
					branch.add(output);
				}

				self.last_return = ValueInfo::new("void".to_string(), 0);

				return arg;
			}

			if expr.target.as_ref().unwrap().string == "read" {
				assert!(expr.args.as_ref().unwrap().get(0).unwrap().value.as_ref().unwrap().kind == TokenType::Number, "Argument to read() must be a constant");
				
				let amount = expr.args.as_ref().unwrap().get(0).unwrap().value.as_ref().unwrap().number;

				let cell = self.gen.memory.alloc(amount as usize + 1);
				
				for i in 0..amount {
					let input = Input::new(cell + i as usize);

					branch.add(input);
				}

				let set = Set::new(cell + amount as usize, 0);

				branch.add(set);

				self.last_return = ValueInfo::new("string".to_string(), amount as usize);

				return cell;
			}

			if expr.target.as_ref().unwrap().string == "open" {
				let expr = expr.args.as_ref().unwrap().get(0).unwrap();

				let arg = self.compile_expression(branch, expr.clone());

				let len = match expr.kind {
					ExpressionType::Literal => {
						match expr.value.as_ref().unwrap().kind {
							TokenType::String => {
								expr.value.as_ref().unwrap().string.len()
							},

							TokenType::Identifier => {
								self.analyzer.scope.get(&expr.value.as_ref().unwrap().string).unwrap_or_else(|| { panic!("Undefined variable {}", expr.value.as_ref().unwrap().string) }).value.size
							}

							_ => 1
						}
					},

					ExpressionType::Call => {
						self.last_return.size
					}

					_ => {
						1
					}
				};

				let op = self.gen.memory.alloc(len + 1);

				branch.add(Set::new(op, ExtendedBF::OpenFile as CellSize));

				let tmp = self.gen.memory.alloc(1);

				for i in 0..len {
					branch.add(Copy::new(op + i + 1, tmp, arg + i));
				}

				let open = Command::new(op);

				branch.add(open);

				self.last_return = ValueInfo::new("int".to_string(), 1);

				return op;
			}

			if expr.target.as_ref().unwrap().string == "write" {
				let argexpr = expr.args.as_ref().unwrap().get(1).unwrap();

				let data = self.compile_expression(branch, argexpr.clone());

				let len = match argexpr.kind {
					ExpressionType::Literal => {
						match argexpr.value.as_ref().unwrap().kind {
							TokenType::String => {
								argexpr.value.as_ref().unwrap().string.len() + 1
							},

							TokenType::Identifier => {
								self.analyzer.scope.get(&argexpr.value.as_ref().unwrap().string).unwrap_or_else(|| { panic!("Undefined variable {}", argexpr.value.as_ref().unwrap().string) }).value.size
							},

							_ => 1
						}
					},

					ExpressionType::Call => {
						self.last_return.size
					}

					_ => {
						Debug::log(&format!("Unknown data type: {:?}", argexpr.kind));

						1
					}
				};

				if len == 0 {
					panic!("Empty argument passed to write");
				}

				let op = self.gen.memory.alloc(len + 2);

				branch.add(Set::new(op, ExtendedBF::Write as CellSize));

				let handle = self.compile_expression(branch, expr.args.as_ref().unwrap().get(0).unwrap().clone());

				let tmp = self.gen.memory.alloc(1);

				branch.add(Copy::new(op + 1, tmp, handle));

				for i in 0..len + 1 {
					let tmp = self.gen.memory.alloc(1);

					branch.add(Copy::new(op + i + 2, tmp, data + i));
				}

				let write = Command::new(op);

				branch.add(write);

				self.last_return = ValueInfo::new("int".to_string(), 1);

				return op;
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
	}

	pub fn compile(&mut self) -> String {
		// let mut str = String::new();

		self.analyzer.analyze();

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