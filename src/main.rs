use std::io;
use wt_missile_calc_lib::missiles::Missile;

use calc::generate;

use crate::launch_parameters::LaunchParameter;

mod launch_parameters;
mod calc;
mod rho;

fn main() {
	println!("Welcome to FlareFlo's missile-range calculator!");
	println!("Please note that this tool is very WIP");
	println!("enable debug mode? y/n");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	let debug = match line.trim() {
		"y" => {true}
		_ => {false}
	};

	let mut launch_parameters = LaunchParameter::new_from_parameters(false, 350.0, 1000.0, 350.0, 0);

	let missiles = Missile::new_from_generated(Some("./all.json"), None);
	loop {
		run_calc(&missiles, launch_parameters.clone(), debug);
	}
}

#[allow(clippy::ptr_arg)]
fn run_calc(missiles: &Vec<Missile>, launch_parameters: LaunchParameter, debug: bool) {
	println!("Enter which missile to test (all lowercase) or leave empty to configure launch selection");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	let mut new_parameters = launch_parameters.clone();

	if line.trim().len() == 0 {
		input_launch_parameters(&mut new_parameters);
	} else {
		if Missile::select_by_name(missiles, line.trim()).is_none() {
			println!("Cannot find missile");
		}else {
			let missile = Missile::select_by_name(missiles, line.trim()).unwrap();

			generate(&missile, &launch_parameters, 0.1, debug);
		}
	}
}

fn input_launch_parameters(launch_parameters: &mut LaunchParameter) {
	println!("Enter start/carrier aircraft velocity in m/s (mach 1 = 343m/s)");
	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.start_velocity = line.trim().parse().unwrap();

	println!("Enter distance to target aircraft in m (0 if no target required)");
	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.distance_to_target = line.trim().parse().unwrap();

	println!("Enter target velocity in m/s (mach 1 = 343m/s)");
	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.target_speed = line.trim().parse().unwrap();

	println!("Enter altitude to simulate in in m");
	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.altitude = line.trim().parse().unwrap();
}