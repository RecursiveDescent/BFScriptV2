mod bfintrp;

mod bfvm;

mod bfstokenizer;

mod bfsparser;

mod bfscompiler;

mod bfextensions;

type CellSize = u32;

pub use bfintrp::*;

pub use bfvm::*;

pub use bfstokenizer::*;

pub use bfsparser::*;

pub use bfscompiler::*;

pub struct Debug {}

static DEBUG: bool = false;

impl Debug {
	pub fn log(s: &str) {
		if DEBUG {
			println!("\x1b[1;32m[INFO]\x1b[0m: {}", s);
		}
	}
}
