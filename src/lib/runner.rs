use std::f64::consts::PI;
use std::io::stdin;

use pad::PadStr;
use wt_datamine_extractor_lib::missile::missile::Missile;

use crate::launch_parameters::LaunchParameter;
use crate::rho::altitude_to_rho;

pub const GRAVITY_TO_ACCEL: f64 = 9.81;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct LaunchResults {
	pub distance_flown: f64,
	pub distance_to_missile: f64,
	pub splash: Splash,
	pub max_v: f64,
	pub max_a: f64,
	pub min_a: f64,
	pub timestep: f64,
	pub profile: Profile,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Profile {
	pub sim_len: u32,
	pub a: Vec<f64>,
	pub v: Vec<f64>,
	pub d: Vec<f64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Splash {
	pub splash: bool,
	pub at: f64,
}

pub fn generate(missile: &Missile, launch_parameters: &LaunchParameter, timestep: f64) -> LaunchResults {
	let sim_len = (missile.timelife / timestep).round().abs() as u32;

	let mut results: LaunchResults = LaunchResults {
		distance_flown: 0.0,
		distance_to_missile: 0.0,
		splash: Splash { splash: false, at: 0.0 },
		max_v: 0.0,
		max_a: 0.0,
		min_a: 0.0,
		timestep,
		profile: Profile { sim_len, a: vec![], v: vec![], d: vec![] },
	};


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

	let mut launch_distance: f64 = 0.0;

	// Constants for calculations
	// javascript moment
	#[allow(clippy::cast_precision_loss)] // save cast thanks to gravity being normal
		let gravity = GRAVITY_TO_ACCEL * launch_parameters.use_gravity as u64 as f64;
	let area = PI * (missile.caliber / 2.0).powi(2);
	let launch_velocity = velocity;

	// Target parameters
	let target_velocity = launch_parameters.target_speed;
	let mut target_distance = launch_parameters.distance_to_target;

	// Statistical values
	let mut closest = f64::MAX;
	let mut max_v = 0.0;
	let mut max_a = 0.0;
	let mut min_a = 0.0;
	let mut splash: Splash = Splash { splash: false, at: 0.0 };

	// Save allow thanks to abs() and never overflowing value thanks to division beforehand
	#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
	for i in 0..sim_len {
		drag_force = 0.5 * rho * velocity.powi(2) * missile.cxk * area;


		let engine_stage;
		let mass;
		let force;

		let burn_0 = 0.0..missile.timefire0;
		let burn_1 = burn_0.end..burn_0.end + missile.timefire1;

		let flight_time = f64::from(i) * timestep;
		let compute_delta_mass = |mass, mass_end, timefire, relative_time| {
			((mass - mass_end) * ((flight_time - relative_time) / timefire))
		};

		match () {
			_ if burn_0.contains(&flight_time) => {
				mass = missile.mass - compute_delta_mass(missile.mass, missile.mass_end, missile.timefire0, 0.0);
				force = missile.force0;
				engine_stage = "0";
			}
			_ if burn_1.contains(&flight_time) => {
				mass = missile.mass_end - compute_delta_mass(missile.mass_end, missile.mass_end1, missile.timefire1, missile.timefire0);
				force = missile.force1;
				engine_stage = "1";
			}
			_ => {
				if missile.mass_end1 != 0.0 {
					mass = missile.mass_end1;
				} else {
					mass = missile.mass_end;
				}
				force = 0.0;
				engine_stage = "-";
			}
		}

		a = ((force - drag_force) / mass) - gravity;

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

		if a < min_a {
			min_a = a;
		}

		if target_distance - distance < closest {
			closest = target_distance - distance;
		}

		results.profile.a.push(a);
		results.profile.v.push(velocity);
		results.profile.d.push(distance);

		if target_distance != 0.0 && target_distance < distance {
			splash.splash = true;
			splash.at = target_distance - launch_distance;
		}
	}

	results.distance_flown = distance;
	results.distance_to_missile = launch_plane_distance;
	results.splash = splash;
	results.max_a = max_a;
	results.max_v = max_v;
	results.min_a = min_a;


	results
}