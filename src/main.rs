extern crate dyscript;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use dyscript::vm::dy_parser::DyParser;

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
}
