#![allow(non_snake_case)]

use std::process::Command;

fn main()
{
	let mut program = "cmd";
	let mut firstArg = "/C";
	
	if !cfg!(target_os = "windows")
	{
		program = "sh";
		firstArg = "-c";
	}
	
	Command::new(program)
		.args(&[firstArg, "npm run stylus"])
		.output()
		.expect("Failed to execute Stylus script");
	
	//Just always re-run this script
	println!("cargo:rerun-if-changed=stylus/**/*.styl");
}
