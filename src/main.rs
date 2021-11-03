use std::io;
use std::thread::sleep;
use std::time::Duration;
use wt_missile_calc_lib::missiles::Missile;

use calc::generate;

use crate::launch_parameters::LaunchParameters;

mod launch_parameters;
mod calc;

fn main() {
	println!("{}", "Welcome to FlareFlo's missile-range calculator!");
	println!("{}", "Please note that this tool is very WIP,\n\
					and currently only simulates:\n\
					Sea Altitude\n\
					Launch at mach 1");

	loop {
		let missiles = Missile::new_from_generated(Some("./all.json"), None);
		run_calc(missiles)
	}
}

fn run_calc(missiles: Vec<Missile>) {
	println!("{}", "Enter which which missile to test (all lowercase)");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	if let None = Missile::select_by_name(&missiles, line.trim()) {
		println!("{}", "Cannot find missile");
	}else {
		let missile = Missile::select_by_name(&missiles, line.trim()).unwrap();

		generate(missile, LaunchParameters::new_from_default_hor(), 0.01, false);
	}
}