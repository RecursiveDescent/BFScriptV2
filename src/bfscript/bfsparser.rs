#![allow(dead_code)]

use super::Tokenizer;
use super::Token;
use super::TokenType;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ExpressionType {
	#[default]
	Binary,
	Unary,
	Literal,
	Call,
}

#[derive(Clone, Debug, Default)]
pub struct Expression {
	pub kind: ExpressionType,

	pub left: Option<Box<Expression>>,

	pub operator: Option<Token>,
	
	pub right: Option<Box<Expression>>,

	pub value: Option<Token>,

	pub target: Option<Token>,

	pub args: Option<Vec<Expression>>
}

impl Expression {
	pub fn new(kind: ExpressionType) -> Expression {
		return Expression {
			kind,
			..Default::default()
		};
	}

	pub fn new_binary(left: Expression, operator: Token, right: Expression) -> Expression {
		return Expression {
			kind: ExpressionType::Binary,
			left: Some(Box::new(left)),
			operator: Some(operator),
			right: Some(Box::new(right)),
			..Default::default()
		};
	}

	pub fn new_literal(value: Token) -> Expression {
		return Expression {
			kind: ExpressionType::Literal,
			value: Some(value),
			..Default::default()
		};
	}

	pub fn new_call(target: Token, args: Vec<Expression>) -> Expression {
		return Expression {
			kind: ExpressionType::Call,
			target: Some(target),
			args: Some(args),
			..Default::default()
		};
	}

	pub fn stringify(&self) -> String {
		let mut str = String::new();
		
		match self.kind {
			ExpressionType::Binary => {
				str += "(";
				str += &self.left.as_ref().unwrap().stringify();
				str += &self.operator.as_ref().unwrap().string;
				str += &self.right.as_ref().unwrap().stringify();
				str += ")";

				return str;
			},

			ExpressionType::Literal => {
				let value = self.value.as_ref().unwrap();

				if value.kind == TokenType::Keyword || value.kind == TokenType::String || value.kind == TokenType::Identifier {
					str += &value.string;
				}

				if value.kind == TokenType::Number {
					str += &value.number.to_string();
				}

				return str;
			},

			ExpressionType::Call => {
				str += &self.target.as_ref().unwrap().string;

				str += "(";

				for arg in self.args.as_ref().unwrap() {
					str += &arg.stringify();

					str += ",";
				}

				str += ")";

				return str;
			},

			_ => {
				return str;
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
	If,
	While,
	VarDecl,
	Assignment,
	Return,
	Expression
}

#[derive(Debug, Clone)]
pub struct Statement {
	pub kind: StatementType,

	pub name: Option<Token>,

	pub type_name: Option<Token>,

	pub condition: Option<Expression>,

	pub block: Option<Vec<Statement>>,

	pub expression: Option<Expression>
}

impl Statement {
	pub fn new(kind: StatementType) -> Statement {
		return Statement {
			kind,
			condition: None,
			block: None,
			expression: None,
		    name: None,
		    type_name: None,
		}
	}
}

pub struct Parser<'a> {
	pub tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
	pub fn new(tokenizer: Tokenizer<'a>) -> Parser<'a> {
		return Parser { tokenizer };
	}

	pub fn statement(&mut self) -> Statement {
		let tokens = self.tokenizer.peek_many(3);

		let token = &tokens[0];

		if tokens[2].kind == TokenType::Equal {
			let vartype = self.tokenizer.expect(TokenType::Identifier);

			let varname = self.tokenizer.expect(TokenType::Identifier);

			self.tokenizer.next(); // equals

			let varvalue = self.expression();

			let mut stmt = Statement::new(StatementType::VarDecl);

			stmt.type_name = Some(vartype);
			
			stmt.name = Some(varname);

			stmt.expression = Some(varvalue);

			return stmt;
		}

		if tokens[1].kind == TokenType::Equal {
			let varname = self.tokenizer.expect(TokenType::Identifier);

			self.tokenizer.next(); // equals

			let varvalue = self.expression();

			let mut stmt = Statement::new(StatementType::Assignment);
			
			stmt.name = Some(varname);

			stmt.expression = Some(varvalue);

			return stmt;
		}

		if token.string == "if" {
			self.tokenizer.next();

			let condition = self.expression();

			let mut stmt = Statement::new(StatementType::If);

			self.tokenizer.expect(TokenType::LBrace);

			let mut block: Vec<Statement> = vec![];

			while ! self.tokenizer.eof() && self.tokenizer.peek_token().kind != TokenType::RBrace {
				block.push(self.statement());
			}

			self.tokenizer.expect(TokenType::RBrace);

			stmt.condition = Some(condition);

			stmt.block = Some(block);

			return stmt;
		}

		if token.string == "while" {
			self.tokenizer.next();

			let condition = self.expression();

			let mut stmt = Statement::new(StatementType::While);

			self.tokenizer.expect(TokenType::LBrace);

			let mut block: Vec<Statement> = vec![];

			while ! self.tokenizer.eof() && self.tokenizer.peek_token().kind != TokenType::RBrace {
				block.push(self.statement());
			}

			self.tokenizer.expect(TokenType::RBrace);

			stmt.condition = Some(condition);

			stmt.block = Some(block);

			return stmt;
		}

		if token.string == "return" {
			self.tokenizer.next();

			let expression = self.expression();

			let mut stmt = Statement::new(StatementType::Return);

			stmt.expression = Some(expression);

			return stmt;
		}
		
		let mut stmt = Statement::new(StatementType::Expression);

		stmt.expression = Some(self.expression());

		return stmt;
	}

	pub fn expression(&mut self) -> Expression {
		return self.logic();
	}

	pub fn logic(&mut self) -> Expression {
		return self.comparison();
	}

	pub fn comparison(&mut self) -> Expression {
		let mut left = self.addsub();

		while matches!(self.tokenizer.peek_token().kind, TokenType::EqualEqual | TokenType::NotEqual | TokenType::GT | TokenType::LT | TokenType::GTEqual | TokenType::LTEqual) {
			let operator = self.tokenizer.next();

			left = Expression::new_binary(left, operator, self.addsub());
		}

		return left;
	}

	pub fn addsub(&mut self) -> Expression {
		let mut left = self.mul();
	
		while self.tokenizer.peek_token().kind == TokenType::Plus || self.tokenizer.peek_token().kind == TokenType::Minus {
			let operator = self.tokenizer.next();

			let right = self.mul();

			left = Expression::new_binary(left, operator, right);
		}

		return left;
	}

	pub fn mul(&mut self) -> Expression {
		let mut left = self.div();

		while self.tokenizer.peek_token().kind == TokenType::Times {
	        let operator = self.tokenizer.next();

			let right = self.div();

			left = Expression::new_binary(left, operator, right);
		}

		return left;
	}

	pub fn div(&mut self) -> Expression {
		let mut left = self.primary();

		while self.tokenizer.peek_token().kind == TokenType::Slash {
	        let operator = self.tokenizer.next();

			let right = self.primary();

			left = Expression::new_binary(left, operator, right);
		}

		return left;
	}

	/*pub fn call(&mut self) -> Expression {
		let mut left = self.primary();

		while self.tokenizer.peek_token().kind == TokenType::LParen {
			self.tokenizer.next();

			if self.tokenizer.peek_token().kind == TokenType::RParen {
				self.tokenizer.next();

				left = Expression::new_call(left, Vec::new());

				continue;
			}

			let mut args = Vec::new();

			while self.tokenizer.peek_token().kind != TokenType::RParen {
				args.push(self.expression());

				if self.tokenizer.peek_token().kind == TokenType::Comma {
					self.tokenizer.next();
				}
				else if self.tokenizer.peek_token().kind != TokenType::RParen {
					panic!("Expected , or )");
				}
			}

			self.tokenizer.next();

			left = Expression::new_call(left, args);
		}

		return left;
			
	}*/

	pub fn primary(&mut self) -> Expression {
		let token = self.tokenizer.next();

		if self.tokenizer.peek_token().kind == TokenType::LParen {
			self.tokenizer.next();

			if self.tokenizer.peek_token().kind == TokenType::RParen {
				self.tokenizer.next();

				return Expression::new_call(token, Vec::new());
			}

			let mut args = Vec::new();

			while self.tokenizer.peek_token().kind != TokenType::RParen {
				args.push(self.expression());

				if self.tokenizer.peek_token().kind == TokenType::Comma {
					self.tokenizer.next();
				}
				else if self.tokenizer.peek_token().kind != TokenType::RParen {
					panic!("Expected , or )");
				}
			}

			self.tokenizer.next();
				
			return Expression::new_call(token, args);
		}

		return Expression::new_literal(token);
	}
}