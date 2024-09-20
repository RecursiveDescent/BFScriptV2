#![allow(dead_code, unused_imports)]

use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

#[derive(Debug)]
pub struct Interpreter<'a> {
	position: usize,
	size: usize,
	source: &'a [u8],
	cells: Vec<u8>,
	pointer: i32,
	level: i32,
	stack: Vec::<usize>,
	skipping: bool,
	skiplevel: i32
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
			skiplevel: 0
		};
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
	}
	
	pub fn run(&mut self) {
		while self.position < self.size {
			self.step();

			self.position += 1;
		}
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