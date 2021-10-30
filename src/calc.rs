use std::f64::consts::PI;
use wt_missile_calc_lib::missiles::Missile;


use crate::launch_parameters::LaunchParameters;

const GRAVITY: f64 = 9.81;
const RHO: [f64; 1] = [1.22];

pub fn generate(missile: Missile, launch_parameters: LaunchParameters, timestep: f64, debug: bool) {
	// State parameters
	let mut drag_force: f64 = 0.0;
	let mut a: f64 = 0.0;
	let mut velocity: f64 = launch_parameters.start_velocity;
	let mut distance: f64 = 0.0;
	let mut launch_distance = 0.0;

	// Constants for calculations
	// javascript moment
	let gravity = GRAVITY * launch_parameters.use_gravity as u64 as f64;
	let area = PI * (missile.caliber / 2.0).powi(2);
	let launch_velocity = velocity;

	// Target parameters
	let target_velocity = launch_parameters.target_speed;
	let mut target_distance = launch_parameters.distance_to_target;

	// Statistical values
	let mut closest = f64::MAX;
	let mut max_v = 0.0;

	for i in 0..((missile.timelife / timestep).round() as u32) {
		drag_force = 0.5 * RHO[0] * velocity.powi(2) * missile.cxk * area;
		if (i as f64 * timestep) < missile.timefire0 {
			a = ((missile.force0 - drag_force) / missile.mass) - gravity;
		} else {
			a = (-drag_force / missile.mass_end) - gravity;
		}

		target_distance += target_velocity * timestep;
		launch_distance += launch_velocity * timestep;


		velocity += a * timestep;
		distance += velocity * timestep;

		// if target_distance < distance {
		// 	println!("Splash at {}m! The target is {}m from the launch aircraft", target_distance, target_distance - launch_distance);
		// 	break;
		// }

		if velocity > max_v {
			max_v = velocity;
		}

		if target_distance - distance < closest {
			closest = target_distance - distance;
		}

		if debug {
			println!("m: {} t: {} a: {} v: {} d: {}", distance, target_distance, a, velocity, drag_force);
		}
	}
	println!("max velocity: {}", max_v);
	println!("max distance reached: {}", distance);

	if launch_parameters.target_speed != 0.0 {
		println!("min missile - target: {}m", closest);
	}
}