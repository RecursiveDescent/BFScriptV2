use std::borrow::Borrow;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod bfscript;

mod tests;

use bfscript::*;

fn interpret_file(file: &str, output_file: &str, extended: bool){
	let mut f = File::open(file).unwrap_or_else(|_| {
		panic!("Could not open file: {}", file);
	});

    let mut contents= Vec::new();

    f.read_to_end(&mut contents).unwrap();

	let mut interpreter = Interpreter::new(&contents);

	if extended {
		interpreter.enable_extended();
	}

	interpreter.run();

	println!();

	println!("\nDumping interpreter state -> {}", output_file);

	interpreter.dump(output_file);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <input_file> [flags]", args[0]);
		println!("Pass -h or --help to display help message.");

        return;
    }

    let input_file = &args[1];

    let mut output_file = "output.bf";

	let mut dump_file = "output.bfm";

	let mut extended = false;

	let mut interpret = false;

    // Example flag processing - you'll need to define your own flags
    for arg in args.iter().skip(1) {
        if arg == "-e" || arg == "--extended" {
			extended = true;
        }

		if arg == "-i" || arg == "--interpret" {
			interpret = true;
        }

		if arg == "-h" || arg == "--help" {
			println!("[BFScript]");
			println!("Usage: {} <input_file> [flags]", args[0]);
			println!();
			println!("Flags:");
			println!("  -o --output     Specify output file, behaves like --dump if running existing brainfuck (default: output.bf)");
			println!("  -d --dump       Specify dump file (default: output.bfm)");
			println!("  -e --extended   Enable extended brainfuck features (experimental)");
			println!("  -i --interpret  Interpret compiled brainfuck, can be used with --extended");
			println!("  -h, --help      Display this help message");

			return;
		}
        
		if arg == "-o" || arg == "--output" {
			output_file = &args[args.iter().position(|x| x == arg).unwrap() + 1];
		}

		if arg == "-d" || arg == "--dump" {
			dump_file = &args[args.iter().position(|x| x == arg).unwrap() + 1];
		}
    }

	let input_type = input_file.split(".").last().unwrap();

	if input_type == "bf" {
		println!("Running as brainfuck file.");

		if extended {
			println!("[Extended Mode]");
		}

		if output_file == "output.bf" {
			output_file = dump_file
		}

		interpret_file(input_file, output_file, extended);

		return;
	}

	let mut file = File::open(input_file).unwrap_or_else(|_| {
		panic!("Could not open file: {}", input_file);
	});

    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();

	let mut compiler = Compiler::new(&contents);

    let output = compiler.compile();

    let mut file = File::create(output_file).unwrap();

    file.write_all(output.as_bytes()).unwrap();

    if interpret {
		file.flush().unwrap_or_else(|_| {
			panic!("Could not flush file: {}", output_file);
		});

		println!("Running as brainfuck file.");

		if extended {
			println!("[Extended Mode]");
		}

		interpret_file(output_file, dump_file, extended);
	}
}
