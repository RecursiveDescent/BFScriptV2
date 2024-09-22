#![allow(dead_code, unused_imports)]

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

use super::*;

pub fn bf_open_file(state: &mut Interpreter) {
    let name = &state.read_string(state.pointer as usize + 1, 255);

    // println!("Opening file: {}", name);
    // open if exists and create it if it doesn't
	let mut file = match std::path::Path::exists(std::path::Path::new(name))  {
        true => File::open(name).unwrap_or_else(|_| {
            panic!("Could not open file: {}", name);
        }),

        _ => File::create_new(name).unwrap_or_else(|_| {
            panic!("Could not create file: {}", name);
        })
	};

	state.files.push(file);

	state.cells[state.pointer as usize] = (state.files.len() - 1) as u8;
}

pub fn bf_write(state: &mut Interpreter) {
    let index = state.cells[state.pointer as usize + 1];

    let mut file: &File = match state.files.get(index as usize) {
        Some(file) => file,
        
        None => {
            panic!("Invalid file index: {}", index);
        }
    };

    // write single byte
    match file.write(&[state.cells[state.pointer as usize + 2]]) {
        Ok(_) => {
            state.cells[state.pointer as usize] = 1;
        },

        Err(e) => {
            Debug::log(format!("Error writing to file: {}", e).as_str());

            state.cells[state.pointer as usize] = 0;
        }
    }
}

pub fn bf_close(state: &mut Interpreter) {
    let index = state.cells[state.pointer as usize + 1];

    let mut file: &File = match state.files.get(index as usize) {
        Some(file) => file,
        
        None => {
            panic!("Invalid file index: {}", index);
        }
    };

    file.flush().unwrap();

    state.files.remove(index as usize);
}