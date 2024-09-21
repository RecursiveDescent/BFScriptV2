#![allow(dead_code, unused_variables, unused_imports)]

use std::io::prelude::*;
use std::vec::Vec;
use std::marker::PhantomData;

use super::{CellSize, Debug};

pub struct BFBuilder {
	pub data: String,
	
	pub indent: usize
}

impl BFBuilder {
	pub fn new(indent: usize) -> BFBuilder {
		return BFBuilder { data: String::new(), indent };
	}

	pub fn indent(&mut self) -> &mut BFBuilder {
		self.data += &"\t".repeat(self.indent);

		return self;
	}

	pub fn nl(&mut self) -> &mut BFBuilder {
		self.data += &"\n";

		return self;
	}

	pub fn string(&mut self, text: &str) -> &mut BFBuilder {
		self.data += &String::from(text);
		// self.data += text;

		return self;
	}

	pub fn instruction<'a>(&mut self, owner: &mut Generator, ins: &'a mut impl Instruction) -> &'a mut dyn Instruction {
		self.data += &("\t".repeat(self.indent) + &ins.compile(owner));

		return ins;
	}

	pub fn bfloop(&mut self, indent: bool) -> BFBuilder {
		let mut l = BFBuilder::new(self.indent);

		if ! indent {
			l.indent = 0;
		}

		l.string(&("\t".repeat(self.indent) + &String::from("[")));

		return l;
	}

	pub fn end(&mut self) -> &mut BFBuilder {
		if self.indent == 0 {
			self.string(&String::from("]"));

			return self;
		}
		
		self.string(&("\t".repeat(self.indent - 1) + &String::from("]")));

		return self;
	}
}

#[derive(Debug)]
pub struct Goto {
	pub cell: usize
}

impl Goto {
	pub fn new(cell: usize) -> Goto {
		return Goto {
			cell
		};
	}

	/*pub fn simulate_new(owner: &mut Generator, cell: usize) -> Goto {
		let goto = Goto {
			cell
		};

		goto.simulate(owner);

		return goto;
	}*/
}

impl Instruction for Goto {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.cell;
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		let dif = owner.cell as isize - self.cell as isize;

		for _ in 0..dif.abs() {
			if dif < 0 {
				builder.string(">");
			}
			else {
				builder.string("<");
			}
		}

		// builder.nl();

		return builder.data;
	}
}

#[derive(Debug)]
pub struct Set {
	pub cell: usize,

	pub value: CellSize
}

impl Set {
	pub fn new(cell: usize, value: CellSize) -> Set {
		return Set {
			cell,
			value
		};
	}
}

impl Instruction for Set {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.cell;

		owner.memory.set(self.cell, self.value);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Goto::new(self.cell)).simulate(owner);

		builder.instruction(owner, &mut Clear::new(self.cell));

		builder.string(&"+".repeat(self.value as usize));

		return builder.data;
	}
}

#[derive(Debug)]
pub struct Add {
	pub a: usize,

	pub b: usize
}

impl Add {
	pub fn new(a: usize, b: usize) -> Add {
		return Add {
			a,
			b
		};
	}
}

impl Instruction for Add {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.b;

		if owner.memory.is_dirty(self.a) {
			owner.memory.set(self.b, 0);
			
			return;
		}

		if owner.memory.is_dirty(self.b) {
			owner.memory.dirty(self.a);
			
			owner.memory.set(self.b, 0);
			
			return;
		}

		let left: CellSize = owner.memory.get(self.a);

		let right: CellSize = owner.memory.get(self.b);

		owner.memory.set(self.a, CellSize::overflowing_add(left, right).0);

		owner.memory.set(self.b, 0);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		// builder.string(&"Add:\n");

		// println!("Add: {} {}", self.a, self.b);

		if self.a == self.b {
			panic!("Add: Cells cannot be the same");
		}

		builder.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		let mut lp = builder.bfloop(false);

		lp.string("-");
		lp.instruction(owner, &mut Goto::new(self.a)).simulate(owner);
		lp.string("+");
		lp.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		builder.string(&lp.end().data);

		builder.nl();

		return builder.data;
	}
}

#[derive(Debug)]
pub struct Sub {
	pub a: usize,

	pub b: usize
}

impl Sub {
	pub fn new(a: usize, b: usize) -> Sub {
		return Sub {
			a,
			b
		};
	}
}

impl Instruction for Sub {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.b;

		if owner.memory.is_dirty(self.a) {
			owner.memory.set(self.b, 0);
			
			return;
		}

		if owner.memory.is_dirty(self.b) {
			owner.memory.dirty(self.a);
			
			owner.memory.set(self.b, 0);
			
			return;
		}

		let left: CellSize = owner.memory.get(self.a).into();

		let right: CellSize = owner.memory.get(self.b).into();

		owner.memory.set(self.a, CellSize::overflowing_sub(left, right).0);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		let mut lp = builder.bfloop(false);

		lp.string("-");
		lp.instruction(owner, &mut Goto::new(self.a)).simulate(owner);
		lp.string("-");
		lp.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		builder.string(&lp.end().data);

		builder.nl();

		return builder.data;
	}
}

pub struct Clear {
	pub cell: usize
}

impl Clear {
	pub fn new(cell: usize) -> Clear {
		return Clear { cell }
	}
}

impl Instruction for Clear {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.cell;

		owner.memory.set(self.cell, 0);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Goto::new(self.cell)).simulate(owner);

		let mut lp = builder.bfloop(false);

		lp.string("-");

		builder.string(&lp.end().data);

		builder.nl();

		return builder.data;
	}
}

pub struct Move {
	pub a: usize,

	pub b: usize
}

impl Move {
	pub fn new(a: usize, b: usize) -> Move {
		return Move { a, b };
	}
}

impl Instruction for Move {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.b;

		if owner.memory.is_dirty(self.b) {
			owner.memory.dirty(self.a);

			owner.memory.set(self.b, 0);

			return;
		}
		
		owner.memory.set(self.a, owner.memory.get(self.b));

		owner.memory.set(self.b, 0);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Clear::new(self.a));

		builder.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		let mut lp = builder.bfloop(false);

		lp.string("-");
		lp.instruction(owner, &mut Goto::new(self.a)).simulate(owner);
		lp.string("+");
		lp.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		builder.string(&lp.end().data);

		builder.nl();

		return builder.data;
	}
}

pub struct BoolNegate {
	pub a: usize,

	pub tmp: usize
}

impl BoolNegate {
	pub fn new(a: usize, tmp: usize) -> BoolNegate {
		return BoolNegate { a, tmp };
	}

	pub fn clean(&self, owner: &mut Generator) {
		owner.memory.free(self.tmp);
	}
}

impl Instruction for BoolNegate {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.a;

		Debug::log(&format!("Negating {} with temp cell {}", self.a, self.tmp));

		if owner.memory.is_dirty(self.a) {
			owner.memory.dirty(self.a);
			owner.memory.dirty(self.tmp);

			return;
		}

		owner.memory.set(self.tmp, owner.memory.get(self.a));
		
		owner.memory.set(self.a, match owner.memory.get(self.a) {
			0 => {
				1
			},
			_ => {
				0
			}
		});
	}

	/*
	temp0[-]+
x[[-]temp0-x]temp0[-x+temp0] */
	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Goto::new(self.tmp)).simulate(owner);

		builder.string("[-]+\n");

		builder.instruction(owner, &mut Goto::new(self.a)).simulate(owner);

		let mut lp = builder.bfloop(false);

		lp.string("[-]");

		lp.instruction(owner, &mut Goto::new(self.tmp)).simulate(owner);

		lp.string("-");

		lp.instruction(owner, &mut Goto::new(self.a)).simulate(owner);

		builder.string(&lp.end().data);

		builder.instruction(owner, &mut Goto::new(self.tmp)).simulate(owner);

		let mut lp2 = builder.bfloop(false);

		lp2.string("-");

		lp2.instruction(owner, &mut Goto::new(self.a)).simulate(owner);

		lp2.string("+");

		lp2.instruction(owner, &mut Goto::new(self.tmp)).simulate(owner);

		builder.string(&lp2.end().data);

		builder.instruction(owner, &mut Goto::new(self.a)).simulate(owner);

		builder.nl();

		return builder.data;
	}
}

pub struct Copy {
	pub a: usize,

	pub tmp: usize,

	pub b: usize
}

impl Copy {
	pub fn new(a: usize, tmp: usize, b: usize) -> Copy {
		return Copy { a, tmp, b };
	}

	pub fn clean(&self, owner: &mut Generator) {
		owner.memory.free(self.tmp);
	}
}

impl Instruction for Copy {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.a;

		Debug::log(&format!("Copying {} to {} with temp cell {}", self.b, self.a, self.tmp));

		owner.memory.set(self.tmp, 0);

		if owner.memory.is_dirty(self.b) {
			owner.memory.dirty(self.a);

			return;
		}
		
		owner.memory.set(self.a, owner.memory.get(self.b));
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Clear::new(self.a));

		// builder.instruction(owner, &mut Clear::new(self.tmp));

		builder.instruction(owner, &mut Move::new(self.tmp, self.b));

		builder.instruction(owner, &mut Goto::new(self.tmp)).simulate(owner);

		// builder.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		let mut lp = builder.bfloop(false);

		lp.string("-");
		lp.instruction(owner, &mut Goto::new(self.a)).simulate(owner);
		lp.string("+");
		lp.instruction(owner, &mut Goto::new(self.b)).simulate(owner);
		lp.string("+");
		lp.instruction(owner, &mut Goto::new(self.tmp)).simulate(owner);

		builder.string(&lp.end().data);

		builder.instruction(owner, &mut Goto::new(self.a)).simulate(owner);

		builder.nl();

		return builder.data;
	}
}

pub struct Div {
	pub block: Option<usize>,

	pub a: usize,

	pub b: usize
}

impl Div {
	pub fn new(a: usize, b: usize) -> Div {
		return Div { block: None, a, b }
	}

	pub fn result(&mut self, owner: &mut Generator) -> usize {
		if self.block == None {
			self.block = Some(owner.memory.alloc(6));
		}
		
		return self.block.unwrap() + 5;
	}

	pub fn clean(&self, owner: &mut Generator) {
		owner.memory.free(self.block.unwrap());

		owner.memory.free(self.block.unwrap() + 2);

		owner.memory.free(self.block.unwrap() + 3);

		owner.memory.free(self.block.unwrap() + 4);
	}
}

impl Instruction for Div {
	// A, 0, 0, 0, B, 0
	// 0, R, 0, 0, B', Q
	fn simulate(&mut self, owner: &mut Generator) {
		if self.block == None {
			self.block = Some(owner.memory.alloc(6));
		}
		
		owner.cell = self.block.unwrap();

		let a = owner.memory.get(self.a);

		let b = owner.memory.get(self.b);

		owner.memory.set(self.a, 0);

		owner.memory.set(self.b, 0);

		owner.memory.set(self.block.unwrap(), 0);
		
		if a != 0 && b != 0 {
			owner.memory.set(self.block.unwrap() + 1, a % b);
		}

		owner.memory.set(self.block.unwrap() + 2, 0);

		owner.memory.set(self.block.unwrap() + 3, 0);

		owner.memory.set(self.block.unwrap() + 4, b);

		if a != 0 && b != 0 {
			owner.memory.set(self.block.unwrap() + 5, a / b);
		}
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		if self.block == None {
			self.block = Some(owner.memory.alloc(6));
		}
		
		let mut builder = BFBuilder::new(owner.indent);

		builder.string(&"\nDIV:\n");

		builder.instruction(owner, &mut Move::new(self.block.unwrap(), self.a));

		builder.instruction(owner, &mut Move::new(self.block.unwrap() + 4, self.b));

		builder.instruction(owner, &mut Set::new(self.block.unwrap() + 1, 0));

		builder.instruction(owner, &mut Set::new(self.block.unwrap() + 2, 0));

		builder.instruction(owner, &mut Set::new(self.block.unwrap() + 3, 0));

		builder.instruction(owner, &mut Set::new(self.block.unwrap() + 5, 0));
		
		builder.instruction(owner, &mut Goto::new(self.block.unwrap())).simulate(owner);

		/*let mut lp = builder.bfloop(false);

		lp.string("-");
		lp.instruction(owner, &mut Goto::new(self.a)).simulate(owner);
		lp.string("-");
		lp.instruction(owner, &mut Goto::new(self.b)).simulate(owner);

		builder.string(&lp.end().data);*/

		builder.string(&"[->+>>+>-[<-]<[<<[->>>+<<<]>>>>+<<-<]<<]");

		builder.nl();

		return builder.data;
	}
}

#[derive(Debug)]
pub struct Mul {
	pub a: usize,

	pub b: usize,

	pub block: Option<usize>
}

impl Mul {
	pub fn new(a: usize, b: usize) -> Mul {
		return Mul {
			a,
			b,
			block: None
		};
	}

	pub fn result(&mut self, owner: &mut Generator) -> usize {
		if self.block == None {
			self.block = Some(owner.memory.alloc(4));
		}
		
		return self.block.unwrap() + 3;
	}
}

impl Instruction for Mul {
	fn simulate(&mut self, owner: &mut Generator) {
		if self.block == None {
			self.block = Some(owner.memory.alloc(4));
		}

		let loc = self.block.unwrap();
		
		owner.cell = loc;

		owner.memory.set(loc, 0);

		owner.memory.set(loc + 2, 0);

		if ! owner.memory.is_dirty(self.b) {
			owner.memory.set(loc + 1, owner.memory.get(self.b));
		}

		if ! owner.memory.is_dirty(self.a) && ! owner.memory.is_dirty(self.b) {
			owner.memory.set(loc + 3, owner.memory.get(self.a).overflowing_mul(owner.memory.get(self.b)).0);
		}

		owner.memory.set(self.a, 0);

		owner.memory.set(self.b, 0);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Move::new(self.block.unwrap(), self.a));

		builder.instruction(owner, &mut Move::new(self.block.unwrap() + 1, self.b));

		builder.instruction(owner, &mut Goto::new(self.block.unwrap())).simulate(owner);

		// https://www.codingame.com/playgrounds/50426/getting-started-with-brainfuck/multiplication
		builder.string(&"[>[->+>+<<]>[-<+>]<<-]");

		builder.nl();

		return builder.data;
	}
}

pub struct Distance {
	pub a: usize,

	pub b: usize,

	pub block: Option<usize>,

	// Cells that contain values if the comparison is greater than or less than
	pub gt_cell: usize,

	pub lt_cell: usize
}

impl Distance {
	pub fn new(a: usize, b: usize) -> Distance {
		return Distance {
			a,
			b,
			block: None,
			gt_cell: 0,
			lt_cell: 0
		};
	}

	pub fn gt(&mut self, owner: &mut Generator) -> usize {
		if self.block == None {
			self.block = Some(owner.memory.alloc(7));

			self.gt_cell = self.block.unwrap() + 3;

			self.lt_cell = self.block.unwrap() + 5;
		}

		return self.gt_cell;
	}

	pub fn lt(&mut self, owner: &mut Generator) -> usize {
		if self.block == None {
			self.block = Some(owner.memory.alloc(7));

			self.gt_cell = self.block.unwrap() + 3;

			self.lt_cell = self.block.unwrap() + 5;
		}

		return self.lt_cell;
	}
}

/* Cell map
	1 1 0 4 0 6 0
      ^
	
	result
	
	1 1 0 0 0 2 0
	  ^
	---------

	1 1 0 6 0 5 0
	  ^

	result
	
	1 1 0 1 0 0 0
	    ^    
*/
impl Instruction for Distance {
	fn simulate(&mut self, owner: &mut Generator) {
		if self.block == None {
			self.block = Some(owner.memory.alloc(7));

			self.gt_cell = self.block.unwrap() + 3;

			self.lt_cell = self.block.unwrap() + 5;
		}

		let loc = self.block.unwrap();
		
		owner.cell = loc + 2;

		owner.memory.set(loc, 1);

		owner.memory.set(loc + 1, 1);

		if owner.memory.is_dirty(self.a) || owner.memory.is_dirty(self.b) {
			owner.memory.set(self.a, 0);

			owner.memory.set(self.b, 0);

			owner.memory.dirty(loc + 3);

			owner.memory.dirty(loc + 5);

			return;
		}

		let aval = owner.memory.get(self.a);

		let bval = owner.memory.get(self.b);

		// Input cells are moved at runtime
		owner.memory.set(self.a, 0);

		owner.memory.set(self.b, 0);

		if aval > bval {
			owner.memory.set(loc + 3, bval.overflowing_sub(aval).0);

			owner.memory.set(loc + 5, 0);

			return;
		}

		if aval < bval {
			owner.memory.set(loc + 3, 0);

			owner.memory.set(loc + 5, bval.overflowing_sub(aval).0);

			return;
		}

		owner.memory.set(loc + 3, 0);

		owner.memory.set(loc + 5, bval.overflowing_sub(aval).0);

		return;
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		if self.block == None {
			self.block = Some(owner.memory.alloc(7));

			self.gt_cell = self.block.unwrap() + 3;

			self.lt_cell = self.block.unwrap() + 5;
		}
		
		let mut builder = BFBuilder::new(owner.indent);

		let loc = self.block.unwrap();

		for i in 0..6 {
			builder.instruction(owner, &mut Set::new(loc + i, 0));//.simulate(owner);
		}

		builder.instruction(owner, &mut Set::new(loc, 1));//.simulate(owner);

		builder.instruction(owner, &mut Set::new(loc + 1, 1));

		builder.instruction(owner, &mut Move::new(loc + 3, self.a));
		builder.instruction(owner, &mut Move::new(loc + 5, self.b));

		// builder.instruction(owner, &mut Goto::new(loc + 3)).simulate(owner);
		builder.instruction(owner, &mut Goto::new(loc + 3)).simulate(owner);

		builder.string(&"[->>[-[<]]<]<<<[>]");

		builder.nl();

		return builder.data;
	}
}

pub struct BFBlock<'a> {
	pub instructions: Vec<Box<dyn Instruction + 'a>>,

	phantom: PhantomData<&'a ()>
}

impl<'a> BFBlock<'a> {
	pub fn new() -> BFBlock<'a> {
		return BFBlock {
			instructions: Vec::new(),

			phantom: PhantomData
		};
	}

	
	/*pub fn add(&mut self, mut instruction: Box<dyn Instruction>) {
		self.instructions.push(&mut *instruction);
	}*/

	/*pub fn add<T>(&mut self, mut instruction: T) where T: Instruction {
		self.instructions.push(&mut instruction);
	}*/

	pub fn add<T>(&mut self, instruction: T) -> &mut (dyn Instruction + 'a) where T: Instruction + 'a {
		self.instructions.push(Box::new(instruction));

		return &mut *(*self.instructions.last_mut().unwrap());
	}
}

pub struct If<'a> {
	pub condition: usize,

	pub block: BFBlock<'a>
}

impl<'a> If<'a> {
	pub fn new(condition: usize) -> If<'a> {
		return If {
			condition,
			
			block: BFBlock::new()
		};
	}
}

impl<'a> Instruction for If<'a> {
	fn simulate(&mut self, owner: &mut Generator) {
		// let mut local = owner.clone();

		/*for instr in &mut self.block.instructions {
			instr.simulate(&mut local);
		}*/

		owner.cell = self.condition;

		let local = owner.clone();

		if owner.memory.is_dirty(self.condition) {
			Debug::log("If condition was dirty");
			
			/*for instr in &mut self.block.instructions {
				instr.simulate(owner);
			}*/

			// Compare differences between memory

			for (i, &v) in owner.memory.cells.clone().iter().enumerate() {
				if v != local.memory.cells[i] {
					owner.memory.dirty(i);

					Debug::log(&format!("If block made cell {} dirty", i));
				}
			}

			owner.memory.set(self.condition, 0);

			return;
		}

		if owner.memory.get(self.condition) != 0 {
			/*for instr in &mut self.block.instructions {
				instr.simulate(owner);
			}*/
		}

		// Has to be set after the block is simulated
		owner.memory.set(self.condition, 0);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		let cond = match owner.memory.is_dirty(self.condition) { true => 1, false => owner.memory.get(self.condition) };

		builder.instruction(owner, &mut Goto::new(self.condition)).simulate(owner);

		let mut lp = builder.bfloop(true);

			for instr in &mut self.block.instructions {
				lp.string(&instr.compile(owner));

				if cond != 0 {
					instr.simulate(owner);
				}
			}
	
			lp.instruction(owner, &mut Goto::new(self.condition)).simulate(owner);
	
			lp.instruction(owner, &mut Clear::new(self.condition));

		builder.string(&lp.end().data);

		builder.nl();

		Debug::log(&format!("If compiled: {}", builder.data));

		return builder.data;
	}
}

pub struct While<'a> {
	pub condition: usize,

	pub block: BFBlock<'a>
}

impl<'a> While<'a> {
	pub fn new(condition: usize) -> While<'a> {
		return While {
			condition,
			
			block: BFBlock::new()
		};
	}
}

impl<'a> Instruction for While<'a> {
	fn simulate(&mut self, owner: &mut Generator) {
		// let mut local = owner.clone();

		/*for instr in &mut self.block.instructions {
			instr.simulate(&mut local);
		}*/

		owner.cell = self.condition;

		let local = owner.clone();

		if owner.memory.is_dirty(self.condition) {
			Debug::log("While condition was dirty");
			
			/*for instr in &mut self.block.instructions {
				instr.simulate(owner);
			}*/

			// Compare differences between memory

			for (i, &v) in owner.memory.cells.clone().iter().enumerate() {
				if v != local.memory.cells[i] {
					owner.memory.dirty(i);

					Debug::log(&format!("While block made cell {} dirty", i));
				}
			}

			owner.memory.set(self.condition, 0);

			return;
		}

		if owner.memory.get(self.condition) != 0 {
			/*for instr in &mut self.block.instructions {
				instr.simulate(owner);
			}*/
		}

		// Has to be set after the block is simulated
		owner.memory.set(self.condition, 0);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		let cond = match owner.memory.is_dirty(self.condition) { true => 1, false => owner.memory.get(self.condition) };

		builder.instruction(owner, &mut Goto::new(self.condition)).simulate(owner);

		let mut lp = builder.bfloop(true);

			for instr in &mut self.block.instructions {
				lp.string(&instr.compile(owner));

				if cond != 0 {
					instr.simulate(owner);
				}
			}
	
			lp.instruction(owner, &mut Goto::new(self.condition)).simulate(owner);

		builder.string(&lp.end().data);

		builder.nl();

		return builder.data;
	}
}

pub struct Input {
	pub cell: usize
}

impl Input {
	pub fn new(cell: usize) -> Input {
		return Input { cell };
	}
}

impl Instruction for Input {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.memory.dirty(self.cell);
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Goto::new(self.cell)).simulate(owner);

		builder.string(&",");

		builder.nl();

		return builder.data;
	}
}

pub struct Output {
	pub cell: usize
}

impl Output {
	pub fn new(cell: usize) -> Output {
		return Output { cell };
	}
}

impl Instruction for Output {
	fn simulate(&mut self, owner: &mut Generator) {
		owner.cell = self.cell;
	}

	fn compile(&mut self, owner: &mut Generator) -> String {
		let mut builder = BFBuilder::new(owner.indent);

		builder.instruction(owner, &mut Goto::new(self.cell)).simulate(owner);

		builder.string(&".");

		builder.nl();

		return builder.data;
	}
}

pub trait Instruction {
	fn simulate(&mut self, owner: &mut Generator);

	fn compile(&mut self, owner: &mut Generator) -> String;
}

#[derive(Clone, Debug)]
pub struct MemoryPool {
	pub cells: Vec<CellSize>,

	pub used: Vec<usize>,

	pub free: Vec<usize>,

	// Cells in this pool are dynamically set at runtime and can't be simulated
	// Reading from cells in this pool is illegal
	pub runtime: Vec<usize>,
	
	pub current: usize
}

impl MemoryPool {
	pub fn new() -> MemoryPool {
		return MemoryPool { cells: vec![], used: vec![], free: vec![], runtime: vec![], current: 0 };
	}

	pub fn alloc(&mut self, size: usize) -> usize {
		if size == 1 && self.free.len() > 0 {
			return self.free.pop().unwrap();
		}
		
		let block = self.current;
		
		self.current += size;

		for _ in self.cells.len()..self.cells.len() + size {
			self.cells.push(0);

			self.used.push(self.cells.len() - 1);
		}

		return block;
	}

	pub fn free(&mut self, cell: usize) {
		if self.used.contains(&cell) {
			self.used.retain(|&x| x != cell);

			self.free.push(cell);

			self.clean(cell);
		}
	}

	pub fn set_used(&mut self, cell: usize) {
		if ! self.used.contains(&cell) {
			self.used.push(cell);
		}
	}

	pub fn set(&mut self, cell: usize, value: CellSize) {
		if cell >= self.cells.len() {
			self.alloc(((self.cells.len() as isize) - cell as isize).abs() as usize + 1);
		}
		
		self.cells[cell] = value;

		// If the cell is set, it's returned to a value known at compile time.
		self.clean(cell);
	}

	pub fn get(&self, cell: usize) -> CellSize {
		if cell > self.cells.len() {
			panic!("Invalid cell! At {}", cell);
		}

		if self.runtime.contains(&cell) {
			panic!("Runtime cell accessed! At {}", cell);
		}
		
		return self.cells[cell];
	}

	pub fn get_raw(&self, cell: usize) -> CellSize {
		if cell > self.cells.len() {
			panic!("Invalid cell! At {}", cell);
		}
		
		return self.cells[cell];
	}

	pub fn dirty(&mut self, cell: usize) {
		if cell > self.cells.len() {
			panic!("Invalid cell! At {}", cell);
		}

		self.runtime.push(cell);
	}

	pub fn clean(&mut self, cell: usize) {
		if cell > self.cells.len() {
			panic!("Invalid cell! At {}", cell);
		}

		self.runtime.retain(|&x| x != cell);
	}

	pub fn is_dirty(&self, cell: usize) -> bool {
		return self.runtime.contains(&cell);
	}
}

#[derive(Clone, Debug)]
pub struct Generator {
	pub cell: usize,

	pub indent: usize,

	pub memory: MemoryPool
}

impl Generator {
	pub fn new() -> Generator {
		return Generator { cell: 0, indent: 0, memory: MemoryPool::new() };
	}
	
	pub fn indent(&self) -> String {
		return "\t".repeat(self.indent);
	}
}