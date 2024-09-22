#![allow(dead_code, unused_imports)]

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

use super::bfextensions;

#[derive(Debug)]
pub enum ExtendedBF {
	OpenFile = 1,
	Write = 2,
	Read = 3,
}

#[derive(Debug)]
pub struct Interpreter<'a> {
	pub position: usize,
	pub size: usize,
	pub source: &'a [u8],
	pub cells: Vec<u8>,
	pub pointer: i32,
	pub level: i32,
	pub stack: Vec::<usize>,
	pub skipping: bool,
	pub skiplevel: i32,
	pub extended_mode: bool,

	pub commands: HashMap<u8, fn(&mut Interpreter)>,
	pub files: Vec<File>,
}

impl<'a> Interpreter<'a> {
	pub fn new(src: &'a [u8]) -> Interpreter {
		let mut vc = Vec::new();

		vc.push(0);
		
		return Interpreter {
			position: 0,
			size: src.len(),
			source: src,
			cells: vc,
			pointer: 0,
			level: 0,
			stack: Vec::<usize>::new(),
			skipping: false,
			skiplevel: 0,
			
			extended_mode: false,
			commands: HashMap::new(),
			files: Vec::new(),
		};
	}

	pub fn enable_extended(&mut self) {
		self.extended_mode = true;

		self.register_defaults();
	}

	pub fn register_defaults(&mut self) {
		self.register_command(ExtendedBF::OpenFile as u8, bfextensions::bf_open_file);
		self.register_command(ExtendedBF::Write as u8, bfextensions::bf_write);
	}

	pub fn step(&mut self) {
		if self.skipping && self.source[self.position] != b'[' && self.source[self.position] != b']' {
			return;
		}
		
		if self.source[self.position] == b'+' {
			self.cells[self.pointer as usize] = self.cells[self.pointer as usize].wrapping_add(1);
		}

		if self.source[self.position] == b'-' {
			self.cells[self.pointer as usize] = self.cells[self.pointer as usize].wrapping_sub(1);
		}

		if self.source[self.position] == b'<' {
			self.left();
		}

		if self.source[self.position] == b'>' {
			self.right();
		}

		if self.source[self.position] == b',' {
			self.cells[self.pointer as usize] = std::io::stdin().bytes().next().unwrap().unwrap();
		}

		if self.source[self.position] == b'.' {
			print!("{}", self.cells[self.pointer as usize] as char);
		}

		if self.source[self.position] == b'[' {
			self.level += 1;

			self.stack.push(self.position - 1);
			
			if ! self.skipping && self.cells[self.pointer as usize] == 0 {
				self.skipping = true;

				self.skiplevel = self.level;
			}
		}

		if self.source[self.position] == b']' {
			let open = match self.stack.pop() {
				Some(pos) => pos,
				
				None => {
					panic!("Mismatched ]");
				}
			};

			if self.skipping {
				if self.level == self.skiplevel {
					self.skipping = false;
				}

				self.level -= 1;
		
				return;
			}

			self.level -= 1;
			
			if self.cells[self.pointer as usize] != 0 {
				self.skipping = false;

				self.position = open;
			}
		}

		if self.source[self.position] == b'@' {
			let op = self.cells[self.pointer as usize];

			let pos = self.position;

			self.run_command(op);

			// Has to end on the same cell
			assert!(self.position == pos, "Commands are not allowed to modify the cell position!");
		}
	}
	
	pub fn run(&mut self) {
		while self.position < self.size {
			self.step();

			self.position += 1;
		}
	}

	pub fn run_command(&mut self, op: u8) {
		if ! self.commands.contains_key(&op) {
			panic!("Unknown command: {}", op);
		}

		let func = self.commands.get(&op);

		func.unwrap()(self);
	}

	pub fn register_command(&mut self, op: u8, func: fn(&mut Interpreter)) {
		self.commands.insert(op, func);
	}

	pub fn read_string(&mut self, pos: usize, max_len: usize) -> String {
		let mut s = String::new();

		for i in pos..pos + max_len {
			if i >= self.cells.len() || self.cells[i] == 0 {
				break;
			}

			s.push(self.cells[i] as char);
		}

		return s;
	}

	pub fn dump(&self, file: &str) {
		let mut file = File::create(file).unwrap();

		let mut data = String::new();

		data += format!("Pointer [{}]\n", self.pointer).as_str();
		data += format!("Cells [{}]\n\n", self.cells.len()).as_str();

		let mut l = 1;
		
		for i in 0..self.cells.len() {
			data += if i == self.pointer as usize {
				format!(">{}< ", self.cells[i])
			}
			else {
				format!("{} ", self.cells[i])
			}.as_str();

			if l % 10 == 0 {
				data += "\n";
			}

			l += 1;
		}

		file.write_all(data.as_bytes()).unwrap();
	}

	pub fn left(&mut self) {
		self.pointer -= 1;

		if self.pointer < 0 {
			self.cells.insert(0, 0);
		}
	}

	pub fn right(&mut self) {
		self.pointer += 1;

		if self.pointer as usize >= self.cells.len() {
			self.cells.push(0);
		}
	}
}