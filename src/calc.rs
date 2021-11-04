use std::f64::consts::PI;
use std::time::Instant;

use pad::PadStr;
use wt_missile_calc_lib::missiles::Missile;

use crate::launch_parameters::LaunchParameter;
use crate::rho::altitude_to_rho;

const GRAVITY: f64 = 9.81;

pub fn generate(missile: &Missile, launch_parameters: &LaunchParameter, timestep: f64, debug: bool) {
	let start = Instant::now();

	// State parameters
	let mut drag_force: f64;
	let mut a: f64;
	let mut velocity: f64 = launch_parameters.start_velocity;
	let mut distance: f64 = 0.0;
	let mut altitude: f64 = 4650.0;

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

	// Save allow thanks to abs() and never overflowing value thanks to division beforehand
	#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
	for i in 0..((missile.timelife / timestep).round().abs() as u32) {
		let rho = altitude_to_rho(altitude.round() as u32);
		drag_force = 0.5 * rho * velocity.powi(2) * missile.cxk * area;


		let mut engine_stage = "0";

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


		velocity += a * timestep;
		distance += velocity * timestep;


		if velocity > max_v {
			max_v = velocity;
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
			println!("Splash at {}m! The target is {}m from the launch aircraft after {}s of flight time", target_distance, target_distance - launch_distance, i as f64 * timestep);
			break;
		}
	}
	println!("max velocity: {}m/s", max_v.round());
	println!("max distance reached: {}m", distance.round());

	if debug {
		println!("Simulation took: {:?}", start.elapsed());
	}

	if launch_parameters.target_speed != 0.0 {
		println!("min missile - target: {}m", closest.round());
	}
}