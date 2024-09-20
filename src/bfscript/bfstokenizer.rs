#![allow(dead_code)]

use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
	Identifier,
	Number,
	Char,
	String,
	Keyword,
	Operator,
	Separator,
	Comment,

	// Operators

	Plus,
	Minus,
	Times,
	Slash,
	Mod,
	CompoundAdd,
	CompoundSub,
	CompoundMul,
	CompoundDiv,
	CompoundMod,

	Not,
	NotEqual,

	Equal,
	EqualEqual,
	
	LParen,
	RParen,

	LBrace,
	RBrace,

	GT,
	LT,
	GTEqual,
	LTEqual,
	
	EndOfFile
}

#[derive(Debug, Clone)]
pub struct Token {
	pub kind: TokenType,

	pub char: char,

	pub string: String,
	
	pub number: i64,

	pub line: u32,
	
	pub column: u32
}

impl Token {
	pub fn new(kind: TokenType, line: u32, column: u32) -> Token {
		return Token {
			kind,
			char: '\0',
			string: String::new(),
			number: 0,
			line,
			column
		};
	}

	pub fn keyword_literal(string: String, line: u32, column: u32) -> Token {
		let mut token = Token::new(TokenType::Keyword, line, column);
		
		token.string = string;
		
		return token;
	}

	pub fn identifier_literal(string: String, line: u32, column: u32) -> Token {
		let mut token = Token::new(TokenType::Identifier, line, column);
		
		token.string = string;
		
		return token;
	}

	pub fn char_literal(char: char, line: u32, column: u32) -> Token {
		let mut token = Token::new(TokenType::Char, line, column);
		
		token.char = char;
		
		return token;
	}

	pub fn string_literal(string: String, line: u32, column: u32) -> Token {
		let mut token = Token::new(TokenType::String, line, column);
		
		token.string = string;
		
		return token;
	}

	pub fn number_literal(number: i64, line: u32, column: u32) -> Token {
		let mut token = Token::new(TokenType::Number, line, column);
		
		token.number = number;
		
		return token;
	}

	pub fn operator(kind: TokenType, string: &str, line: u32, column: u32) -> Token {
		return {
			let mut tok = Token::new(kind, line, column);

			tok.string = String::from(string);

			tok
		};
	}
}

pub struct Tokenizer<'a> {
	pub position: usize,

	pub source: &'a [u8],

	pub line: u32,

	pub column: u32
}

fn is_whitespace(c: char) -> bool {
	return c == ' '|| c == '\t' || c == '\n' || c == '\r';
}

fn is_digit(c: char) -> bool {
	return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

const KEYWORDS: [&str;3] = ["if", "while", "for"];

impl<'a> Tokenizer<'a> {
	pub fn new(source: &'a [u8]) -> Tokenizer {
		return Tokenizer { position: 0, source, line: 1, column: 1 };
	}

	pub fn eof(&self) -> bool {
		return self.position >= self.source.len();
	}

	pub fn get(&mut self) -> Option<char> {
		return {
			if self.position >= self.source.len() {
				return None;
			}
			
			let tmp = self.source[self.position];

			self.position += 1;

			self.column += 1;

			if tmp == b'\n' {
				self.line += 1;
				
				self.column = 1;
			}

			Some(tmp as char)
		};
	}

	pub fn peek(&self) -> Option<char> {
		if self.position >= self.source.len() {
			return None;
		}
		
		return Some(self.source[self.position] as char);
	}

	pub fn next(&mut self) -> Token {
		if self.position >= self.source.len() {
			return Token::new(TokenType::EndOfFile, self.line, self.column);
		}

		while is_whitespace(self.peek().unwrap()) {
			self.position += 1;
		}

		let c = self.get().unwrap();

		match c {
			'"' => {
				let mut string = String::new();

				while self.peek().unwrap() != '"' {
					string.push(self.get().unwrap());

					if self.peek() == None {
						panic!("Unexpected end of file, expecting '\"' to close string.");
					}
				}

				self.get().unwrap();

				return Token::string_literal(string, self.line, self.column);
			}

			'\'' => {
				let char = self.get().unwrap();

				if self.peek() == None || self.peek().unwrap() != '\'' {
					panic!("Expected ' to close character literal.");
				}

				self.get();

				return Token::char_literal(char, self.line, self.column);
			}

			_ => {}
		}

		match c {
			'=' => match self.peek() {
				Some('=') => {
					self.position += 2;

					return Token::operator(TokenType::EqualEqual, "==", self.line, self.column);
				}

					
				_ => {
					self.position += 1;

					return Token::operator(TokenType::Equal, "=", self.line, self.column);
				}
			},

			'!' => match self.peek() {
				Some('=') => {
					self.position += 2;

					return Token::operator(TokenType::NotEqual, "!=", self.line, self.column);
				}

					
				_ => {
					self.position += 1;

					return Token::operator(TokenType::Not, "!", self.line, self.column);
				}
			},
			
			'+' => match self.peek() {
					Some('=') => {
						self.position += 2;
						
						return Token::operator(TokenType::CompoundAdd, "+=", self.line, self.column);
				},

				_ => {
					self.position += 1;
					
					return Token::operator(TokenType::Plus, "+", self.line, self.column);
				}
			},

			
			'-' => match self.peek() {
				Some('=') => {
					self.position += 2;

					return Token::operator(TokenType::CompoundSub, "-=", self.line, self.column);
				}

					
				_ => {
					self.position += 1;

					return Token::operator(TokenType::Minus, "-", self.line, self.column);
				}
			},

			'*' => match self.peek() {
				Some('=') => {
					self.position += 2;

					return Token::operator(TokenType::CompoundMul, "*=", self.line, self.column);
				}

					
				_ => {
					self.position += 1;

					return Token::operator(TokenType::Times, "*", self.line, self.column);
				}
			}

			'/' => match self.peek() {
				Some('=') => {
					self.position += 2;

					return Token::operator(TokenType::CompoundDiv, "/=", self.line, self.column);
				}

					
				_ => {
					self.position += 1;

					return Token::operator(TokenType::Slash, "/", self.line, self.column);
				}
			}

			'>' => match self.peek() {
				Some('=') => {
					self.position += 2;

					return Token::operator(TokenType::GTEqual, ">=", self.line, self.column);
				}

				_ => {
					self.position += 1;
					
					return Token::operator(TokenType::GT, ">", self.line, self.column);
				}
			},

			'<' => match self.peek() {
				Some('=') => {
					self.position += 2;

					return Token::operator(TokenType::LTEqual, "<=", self.line, self.column);
				}

				_ => {
					self.position += 1;

					return Token::operator(TokenType::LT, "<", self.line, self.column);
				}
			},

			_ => {
				self.position -= 1;
			}
		}

		match c {
			'(' => {
				self.get();
				
				return Token::operator(TokenType::LParen, "(", self.line, self.column);
			}

			')' => {
				self.get();
				
				return Token::operator(TokenType::RParen, ")", self.line, self.column);
			}

			'{' => {
				self.get();
				
				return Token::operator(TokenType::LBrace, "{", self.line, self.column);
			},
			
			'}' => {
				self.get();
				
				return Token::operator(TokenType::RBrace, "}", self.line, self.column);
			},

			_ => {}
		}

		if is_digit(self.peek().unwrap()) {
			let mut number = String::new();
			
			while ! self.eof() && is_digit(self.peek().unwrap()) {
				number.push(self.get().unwrap());
			}

			return Token::number_literal(number.parse().unwrap(), self.line, self.column);
		}

		let mut id = String::new();

		while ! self.eof() && (is_alpha(self.peek().unwrap()) || is_digit(self.peek().unwrap())) {
			id.push(self.get().unwrap());
		}

		if KEYWORDS.contains(&id.as_str()) {
			return Token::keyword_literal(id, self.line, self.column);
		}

		return Token::identifier_literal(id, self.line, self.column);
	}

	pub fn expect(&mut self, t: TokenType) -> Token {
		let tok = self.next();

		if tok.kind != t {
			panic!("Expected {:?} but got {:?}", t, tok);
		}

		return tok;
	}

	pub fn peek_token(&mut self) -> Token {
		let pos = self.position;
		
		let line = self.line;

		let column = self.column;
		
		let token = self.next();

		self.position = pos;

		self.line = line;

		self.column = column;

		return token;
	}

	pub fn peek_many(&mut self, n: usize) -> Vec<Token> {
		let pos = self.position;
		
		let line = self.line;

		let column = self.column;
		
		let mut tokens = Vec::new();

		for _ in 0..n {
			tokens.push(self.next());
		}

		self.position = pos;

		self.line = line;

		self.column = column;

		return tokens;
	}
}