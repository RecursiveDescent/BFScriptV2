use std::fs::File;
use std::io::prelude::*;

mod bfscript;

mod tests;

use bfscript::*;

fn main() {
	let mut file = File::open("code.bfs").unwrap();

	let mut contents: Vec<u8> = Vec::new();

	file.read_to_end(&mut contents).unwrap();

	let mut compiler = Compiler::new(&contents);

	println!("{}", compiler.compile());
}