extern crate dy;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use dy::vm::dy_parser::DyParser;
use dy::vm::dy_common::DyRef;

fn main() {

//	let args: Vec<String> = env::args().collect();

	let mut filename = env::current_dir().expect("error");
	filename.push("examples/hello_world.dy"); //&args[1];
	let filename = filename.as_path();

	println!("In file {:?}", filename);

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");
	let mut dy_parser = DyParser::new(contents);
	dy_parser.lex_line();

	println!("With parser:\n{:#?}", dy_parser);
	println!("Hello World, DY!");

	// ---------------  test -------------------
	let a1 = DyRef::new(11);
	let a2 = DyRef::new(12);
	println!("----->{}, {:?}", a1, a1.parent());
}
