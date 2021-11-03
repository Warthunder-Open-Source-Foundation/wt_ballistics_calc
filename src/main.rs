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
	println!("enable debug mode? y/n");

	let mut debug = false;
	let mut a: f64;
	let area = PI * (DIAM / 2.0).powi(2);

	let mut drag_force: f64;
	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	match line.trim() {
		"y" => {debug = true}
		_ => {}
	}
	let mut velocity: f64 = 367.0;

	loop {
		let missiles = Missile::new_from_generated(Some("./all.json"), None);
		run_calc(missiles, debug)
	}
}

fn run_calc(missiles: Vec<Missile>, debug: bool) {
	println!("{}", "Enter which which missile to test (all lowercase)");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	let target_velocity = 343.0;
	let mut target_distance = 2000.0;

	if let None = Missile::select_by_name(&missiles, line.trim()) {
		println!("{}", "Cannot find missile");
	}else {
		let missile = Missile::select_by_name(&missiles, line.trim()).unwrap();
	let mut max_v = 0.0;
	let mut closest = f64::MAX;

	for i in 0..((SIMTIME / TIMESTEP).round().abs() as u64) {
		drag_force = 0.5 * RHO * velocity.powi(2) * CXK * area;
		if (i as f64 * TIMESTEP) < TIMEFIRE0 {
			a = ((THRUST0 - drag_force) / MASS) - gravity;
		} else {
			a = (-drag_force / MASS_END) - gravity;
		}

		target_distance += target_velocity * TIMESTEP;
		launch_distance += launch_velocity * TIMESTEP;

		velocity += a * TIMESTEP;
		if velocity > ENDSPEED {
			velocity = ENDSPEED;
		}

		distance += velocity * TIMESTEP;

		if target_distance < distance {
			println!("Splash! The target is {}m from the launch aircraft", (target_distance - launch_distance).round());
			break;
		}

		if velocity > max_v {
			max_v = velocity;
		}

		if target_distance - distance < closest {
			closest = target_distance - distance;
		}

		println!("i: {}  ld(m): {}  td(m): {}  dd(m): {}  am(ms2): {}  mv(m/s): {}  md(N): {}",
				 i.to_string().pad_to_width(3),
				 distance.round().to_string().pad_to_width(4),
				 target_distance.round().to_string().pad_to_width(4),
				 (target_distance - distance).round().to_string().pad_to_width(4),
				 a.round().to_string().pad_to_width(4),
				 velocity.round().to_string().pad_to_width(3),
				 drag_force.round().to_string().pad_to_width(4)
		);
		generate(missile, LaunchParameters::new_from_default_hor(), 0.1, debug);
	}
	println!("max v: {}", max_v.round());
	println!("delta target-missile: {}m", closest.round());
}