use std::f64::consts::PI;
use std::io::stdin;
use pad::PadStr;
use wt_datamine_extractor_lib::missile::missile::Missile;
use crate::launch_parameters::LaunchParameter;
use crate::rho::altitude_to_rho;

#[allow(clippy::ptr_arg)]
pub fn run_calc(missiles: &Vec<Missile>, launch_parameters: LaunchParameter, debug: bool) {
	println!("Enter which missile to test (all lowercase) or leave empty to configure launch selection");

	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	let mut new_parameters = launch_parameters.clone();

	if line.trim().len() == 0 {
		input_launch_parameters(&mut new_parameters);
	} else {
		if Missile::select_by_name(missiles, line.trim()).is_none() {
			println!("Cannot find missile");
		} else {
			let missile = Missile::select_by_name(missiles, line.trim()).unwrap();

			generate(&missile, &launch_parameters, 0.1, debug);
		}
	}
}

pub fn input_launch_parameters(launch_parameters: &mut LaunchParameter) {
	println!("Enter start/carrier aircraft velocity in m/s (mach 1 = 343m/s)");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.start_velocity = line.trim().parse().unwrap();

	println!("Enter distance to target aircraft in m (0 if no target required)");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.distance_to_target = line.trim().parse().unwrap();

	println!("Enter target velocity in m/s (mach 1 = 343m/s)");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.target_speed = line.trim().parse().unwrap();

	println!("Enter altitude to simulate in in m");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.altitude = line.trim().parse().unwrap();
}

const GRAVITY: f64 = 9.81;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct LaunchResults {
	pub distance_flown: f64,
	pub distance_to_missile: f64,
	pub splash: Splash,
	pub max_v: f64,
	pub max_a: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Splash {
	pub splash: bool,
	pub at: f64,
}

pub fn generate(missile: &Missile, launch_parameters: &LaunchParameter, timestep: f64, debug: bool) -> LaunchResults {
	// let start = Instant::now();

	// State parameters
	let mut drag_force: f64;
	let mut a: f64;
	let mut velocity: f64 = launch_parameters.start_velocity;
	let mut distance: f64 = 0.0;
	let altitude: f64 = launch_parameters.altitude as f64;
	let mut launch_plane_distance: f64 = 0.0;

	// IMPORTANT when changing altitude anywhere move this function too
	let rho = altitude_to_rho(altitude.round() as u32);

	#[allow(unused_variables)] // Clippy being retarded again
		let mut launch_distance: f64 = 0.0;

	// Constants for calculations
	// javascript moment
	#[allow(clippy::cast_precision_loss)] // save cast thanks to gravity being normal
		let gravity = GRAVITY * launch_parameters.use_gravity as u64 as f64;
	let area = PI * (missile.caliber / 2.0).powi(2);
	let launch_velocity = velocity;

	// Target parameters
	let target_velocity = launch_parameters.target_speed;
	let mut target_distance = launch_parameters.distance_to_target;

	// Statistical values
	let mut closest = f64::MAX;
	let mut max_v = 0.0;
	let mut max_a = 0.0;
	let mut splash: Splash = Splash { splash: false, at: 0.0 };

	// Save allow thanks to abs() and never overflowing value thanks to division beforehand
	#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
	for i in 0..((missile.timelife / timestep).round().abs() as u32) {
		drag_force = 0.5 * rho * velocity.powi(2) * missile.cxk * area;


		let engine_stage;

		if (f64::from(i) * timestep) < missile.timefire0 {
			a = ((missile.force0 - drag_force) / missile.mass) - gravity;
			engine_stage = "0";
		} else if missile.timefire1 != 0.0 && (f64::from(i) * timestep) < missile.timefire1 {
			a = ((missile.force1 - drag_force) / missile.mass) - gravity;
			engine_stage = "1";
		} else {
			a = (-drag_force / missile.mass_end) - gravity;
			engine_stage = "-";
		}

		target_distance += target_velocity * timestep;
		launch_distance += launch_velocity * timestep;
		launch_plane_distance += launch_parameters.start_velocity * timestep;


		velocity += a * timestep;
		distance += velocity * timestep;


		if velocity > max_v {
			max_v = velocity;
		}

		if a > max_a {
			max_a = a;
		}

		if target_distance - distance < closest {
			closest = target_distance - distance;
		}

		if debug {
			println!("ts(s): {} D(m): {} Dt(m): {} a(m/s²): {} v(m/s): {} d(N): {} rho: {} s: {}",
					 format!("{:.1}", (i as f64 * timestep)).to_string().pad_to_width(4),
					 distance.round().to_string().pad_to_width(4),
					 target_distance.round().to_string().pad_to_width(4),
					 a.round().to_string().pad_to_width(3),
					 velocity.round().to_string().pad_to_width(4),
					 drag_force.round().to_string().pad_to_width(4),
					 rho.to_string().pad_to_width(6),
					 engine_stage.pad_to_width(1),
			);
		}

		if target_distance != 0.0 && target_distance < distance {
			splash.splash = true;
			splash.at = target_distance - launch_distance;
		}

		// if target_distance != 0.0 && target_distance < distance {
		// 	println!("Splash at {}m! The target is {}m from the launch aircraft after {}s of flight time", target_distance, target_distance - launch_distance, i as f64 * timestep);
		// 	break;
		// }
	}
	println!("max velocity: {}m/s", max_v.round());
	println!("max distance reached: {}m", distance.round());

	if debug {
		// println!("Simulation took: {:?}", start.elapsed());
	}

	if launch_parameters.target_speed != 0.0 {
		println!("min missile - target: {}m", closest.round());
	}

	LaunchResults {
		distance_flown: distance,
		distance_to_missile: launch_plane_distance,
		splash: splash,
		max_v,
		max_a,
	}
}