use std::io;
use wt_missile_calc_lib::missiles::Missile;
use wt_ballistics_calc_lib::launch_parameters::LaunchParameter;
use wt_ballistics_calc_lib::runner::run_calc;

fn main() {
	println!("Welcome to FlareFlo's missile-range calculator!");
	println!("Please note that this tool is very WIP");
	println!("enable debug mode? y/n");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	let debug = match line.trim() {
		"y" => { true }
		_ => { false }
	};

	let launch_parameters = LaunchParameter::new_from_parameters(false, 343.0, 0.0, 0.0, 0);

	let missiles: Vec<Missile>;

	#[cfg(not(feature = "inline"))]
	if let Some(generated) = Missile::new_from_generated(Some("./all.json"), None) {
		missiles = generated;
		if debug {
			println!("Using json-index");
		}
	} else {
		panic!("Cannot find inline json-index")
	}

	#[cfg(feature = "inline")]
	if let Some(fallback) = serde_json::from_str(include_str!("../../../wt_missile_calc/index/all.json")).unwrap() {
		missiles = fallback;
		if debug {
			println!("Using inline-index");
		}
	} else {
		panic!("Cannot find inline index")
	}

	loop {
		run_calc(&missiles, launch_parameters.clone(), debug);
	}
}