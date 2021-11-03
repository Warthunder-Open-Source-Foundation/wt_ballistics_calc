use std::io;
use wt_missile_calc_lib::missiles::Missile;

use calc::generate;

use crate::launch_parameters::LaunchParameters;

mod launch_parameters;
mod calc;

fn main() {
	println!("Welcome to FlareFlo's missile-range calculator!");
	println!("Please note that this tool is very WIP,\n\
					and currently only simulates:\n\
					Sea Altitude\n\
					Launch at mach 1");
	println!("enable debug mode? y/n");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	let debug = match line.trim() {
		"y" => {true}
		_ => {false}
	};

	loop {
		let missiles = Missile::new_from_generated(Some("./all.json"), None);
		run_calc(&missiles, debug);
	}
}

#[allow(clippy::ptr_arg)]
fn run_calc(missiles: &Vec<Missile>, debug: bool) {
	println!("Enter which which missile to test (all lowercase)");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	if Missile::select_by_name(missiles, line.trim()).is_none() {
		println!("Cannot find missile");
	}else {
		let missile = Missile::select_by_name(missiles, line.trim()).unwrap();

		generate(&missile, &LaunchParameters::new_from_default_hor(), 0.1, debug);
	}
}