mod bfintrp;

mod bfvm;

mod bfstokenizer;

mod bfsparser;

mod bfscompiler;

mod bfsanalyzer;

mod bfextensions;

type CellSize = u32;

pub use bfintrp::*;

pub use bfvm::*;

pub use bfstokenizer::*;

pub use bfsparser::*;

pub use bfscompiler::*;

pub use bfsanalyzer::*;

use std::cell::Cell;

pub struct Debug {}

pub static DEBUG: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

impl Debug {
	pub fn log(s: &str) {
		if *DEBUG.lock().unwrap() {
			println!("\x1b[1;32m[INFO]\x1b[0m: {}", s);
		}
	}
}
