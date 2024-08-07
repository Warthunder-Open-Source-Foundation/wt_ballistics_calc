use std::f64::consts::PI;
use std::fmt::{Display, Formatter};

use pad::PadStr;
use wt_datamine_extractor_lib::missile::missile::Missile;

use crate::launch_parameters::LaunchParameter;
use crate::atmosphere::altitude_to_rho;
use crate::runner::EngineStage::{BurntOut, Running};

const GRAVITY: f64 = 9.81;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct LaunchResults {
	pub distance_flown: f64,
	pub distance_to_missile: f64,
	pub splash: Splash,
	pub max_v: f64,
	pub max_a: f64,
	pub min_a: f64,
	pub timestep: f64,
	#[serde(skip_serializing)]
	pub profile: Profile,
	pub parameters: LaunchParameter,
}

pub enum EngineStage {
	Running {
		level: u8,
	},
	BurntOut,
}

impl Display for EngineStage {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}",
			   match self {
				   Running { level } => {
					   level.to_string()
				   }
				   BurntOut => {
					   "-".to_string()
				   }
			   })
	}
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

pub fn generate(missile: &Missile, launch_parameters: LaunchParameter, timestep: f64, debug: bool) -> LaunchResults {
	let sim_len = (missile.timelife / timestep).round().abs() as u32;

	let mut results: LaunchResults = LaunchResults {
		distance_flown: 0.0,
		distance_to_missile: 0.0,
		splash: Splash { splash: false, at: 0.0 },
		max_v: 0.0,
		max_a: 0.0,
		min_a: 0.0,
		timestep,
		profile: Profile { sim_len, a: Vec::with_capacity(sim_len as usize), v: Vec::with_capacity(sim_len as usize), d: Vec::with_capacity(sim_len as usize) },
		parameters: launch_parameters,
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

	let gravity = GRAVITY * if launch_parameters.use_gravity { 1.0 } else { 0.0 };
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

	for i in 0..sim_len {
		drag_force = 0.5 * rho * velocity.powi(2) * missile.cxk * area;


		// Current engine stage
		let engine_stage: EngineStage;

		// Current mass of missile
		let mass;

		// Sum of forces applied to missile
		let force;

		let burn_0 = 0.0..missile.timefire0;
		let burn_1 = burn_0.end..burn_0.end + missile.timefire1;

		let flight_time = f64::from(i) * timestep;
		let compute_delta_mass = |mass, mass_end, timefire, relative_time| {
			(mass - mass_end) * ((flight_time - relative_time) / timefire)
		};

		match () {
			// Booster stage
			_ if burn_0.contains(&flight_time) => {
				mass = missile.mass - compute_delta_mass(missile.mass, missile.mass_end, missile.timefire0, 0.0);
				force = missile.force0;
				engine_stage = Running { level: 0 };
			}
			// Sustainer stage
			_ if burn_1.contains(&flight_time) => {
				mass = missile.mass_end - compute_delta_mass(missile.mass_end, missile.mass_end1, missile.timefire1, missile.timefire0);
				force = missile.force1;
				engine_stage = Running { level: 1 };
			}
			// Coasting
			_ => {
				if missile.mass_end1 != 0.0 {
					mass = missile.mass_end1;
				} else {
					mass = missile.mass_end;
				}
				force = 0.0;
				engine_stage = BurntOut;
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

		if debug {
			println!("ts(s): {} D(m): {} Dt(m): {} a(m/s²): {} v(m/s): {} d(N): {} rho: {} m(kg): {} s: {}",
					 format!("{:.1}", (i as f64 * timestep)).to_string().pad_to_width(4),
					 distance.round().to_string().pad_to_width(5),
					 target_distance.round().to_string().pad_to_width(4),
					 a.round().to_string().pad_to_width(3),
					 velocity.round().to_string().pad_to_width(4),
					 drag_force.round().to_string().pad_to_width(5),
					 rho.to_string()[..6].pad_to_width(4),
					 mass.to_string()[..5].pad_to_width(5),
					 engine_stage.to_string().pad_to_width(1),
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

	if debug {
		// println!("Simulation took: {:?}", start.elapsed());
		println!("max velocity: {}m/s", max_v.round());
		println!("max distance reached: {}m", distance.round());
	}

	if launch_parameters.target_speed != 0.0 {
		println!("min missile - target: {}m", closest.round());
	}

	results.distance_flown = distance;
	results.distance_to_missile = launch_plane_distance;
	results.splash = splash;
	results.max_a = max_a;
	results.max_v = max_v;
	results.min_a = min_a;


	results

	// LaunchResults {
	// 	distance_flown: distance,
	// 	distance_to_missile: launch_plane_distance,
	// 	splash: splash,
	// 	max_v,
	// 	max_a,
	// 	t_to_target: TimeToTarget {
	// 		t_to_1km: 0.0,
	// 		t_to_2km: 0.0,
	// 		t_to_mach2: 0.0,
	// 		t_to_mach3: 0.0
	// 	}
	// }
}