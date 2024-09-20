#![allow(unused_imports)]

use std::fs::File;
use std::io::prelude::*;

use crate::bfscript;

use bfscript::*;

#[test]
fn goto_test() {
	let mut gen = Generator::new();

	let mut goto = Goto::new(4);
	
	assert_eq!(goto.compile(&mut gen).match_indices(">").count(), 4);

	goto.simulate(&mut gen);

	let mut goto2 = Goto::new(6);

	assert_eq!(goto2.compile(&mut gen).match_indices(">").count(), 2);

	goto2.simulate(&mut gen);

	let mut goto3 = Goto::new(3);

	assert_eq!(goto3.compile(&mut gen).match_indices("<").count(), 3);

	goto3.simulate(&mut gen);

	assert_eq!(gen.cell, 3);
}

#[test]
fn set_test() {
	let mut gen = Generator::new();

	let mut set = Set::new(4, 5);
	
	assert_eq!(set.compile(&mut gen).match_indices(">").count(), 4);
	assert_eq!(set.compile(&mut gen).match_indices("+").count(), 5);

	set.simulate(&mut gen);

	let mut set2 = Set::new(6, 10);

	assert_eq!(set2.compile(&mut gen).match_indices(">").count(), 2);
	assert_eq!(set2.compile(&mut gen).match_indices("+").count(), 10);

	set2.simulate(&mut gen);

	assert_eq!(gen.cell, 6);
}

#[test]
fn negate_test() {
	let mut gen = Generator::new();

	let mut set = Set::new(3, 5);

	set.simulate(&mut gen);

	let mut set = Set::new(4, 0);

	set.simulate(&mut gen);

	let mut neg = BoolNegate::new(3, 5);

	neg.simulate(&mut gen);

	let mut neg2 = BoolNegate::new(4, 5);

	neg2.simulate(&mut gen);

	assert_eq!(gen.memory.get(3), 0);
	assert_eq!(gen.memory.get(4), 1);
}

#[test]
fn add_test() {
	let mut gen = Generator::new();

	Set::new(1, 5).simulate(&mut gen);

	Set::new(2, 10).simulate(&mut gen);

	let mut add = Add::new(1, 2);

	let result = add.compile(&mut gen);

	assert_eq!(result, "[-<+>]\n");

	add.simulate(&mut gen);

	assert_eq!(gen.cell, 2);
}

#[test]
fn sub_test() {
	let mut gen = Generator::new();

	Set::new(1, 5).simulate(&mut gen);

	Set::new(2, 10).simulate(&mut gen);

	let mut sub = Sub::new(1, 2);

	let result = sub.compile(&mut gen);

	assert_eq!(result, "[-<->]\n");

	sub.simulate(&mut gen);

	assert_eq!(gen.cell, 2);
}

#[test]
fn dist_test() {
	let mut gen = Generator::new();

	let mut str = String::new();

	let mut s1 = Set::new(1, 5);

	str += &s1.compile(&mut gen);

	s1.simulate(&mut gen);

	let mut s2 = Set::new(2, 10);

	str += &s2.compile(&mut gen);

	s2.simulate(&mut gen);

	let mut dst = Distance::new(1, 2);

	/*let result = dst.compile(&mut gen);

	assert_eq!(result, "[-<->]\n");*/

	//dst.simulate(&mut gen);

	let c = dst.gt(&mut gen);
	println!("{}", gen.memory.get(c));

	println!("{}", dst.block.unwrap());

	println!("{} {}", str + &dst.compile(&mut gen), gen.cell);

	// dst.simulate(&mut gen);

	assert_eq!(gen.cell, 5);
}