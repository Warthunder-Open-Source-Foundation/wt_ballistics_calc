use std::f64::consts::PI;
use std::fmt::{Display, Formatter};

use pad::PadStr;
use simple_si_units::base::{Distance, Mass};
use simple_si_units::mechanical::{Acceleration, Force, Velocity};
use wt_datamine_extractor_lib::missile::missile::Missile;

use crate::launch_parameters::LaunchParameter;
use crate::atmosphere::{altitude_to_rho, ias_to_tas, Atmosphere};
use crate::runner::EngineStage::{BurntOut, Running};

const GRAVITY: f64 = 9.81;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct LaunchResults {
	pub distance_flown: Distance<f64>,
	pub distance_to_missile: Distance<f64>,
	pub splash: Splash,
	pub max_v: Velocity<f64>,
	pub max_a: Acceleration<f64>,
	pub min_a: Acceleration<f64>,
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
	pub a: Vec<Acceleration<f64>>,
	pub v: Vec<Velocity<f64>>,
	pub d: Vec<Distance<f64>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Splash {
	pub splash: bool,
	pub at: Distance<f64>,
}

pub fn generate(missile: &Missile, launch_parameters: LaunchParameter, timestep: f64, debug: bool) -> LaunchResults {
	let sim_len = (missile.timelife / timestep).round().abs() as u32;

	let mut results: LaunchResults = LaunchResults {
		distance_flown: Distance::from_m(0.0),
		distance_to_missile: Distance::from_m(0.0),
		splash: Splash { splash: false, at: Distance::from_m(0.0) },
		max_v: Velocity::from_mps(0.0),
		max_a: Acceleration::from_mps2(0.0),
		min_a: Acceleration::from_mps2(0.0),
		timestep,
		profile: Profile { sim_len, a: Vec::with_capacity(sim_len as usize), v: Vec::with_capacity(sim_len as usize), d: Vec::with_capacity(sim_len as usize) },
		parameters: launch_parameters,
	};

	// State parameters
	let mut drag_force: Force<f64>;
	let mut a: Acceleration<f64>;
	//Equiv airspeed
	let mut ias = launch_parameters.start_velocity;
	let mut distance = Distance::from_m(0.0);
	let altitude = launch_parameters.altitude;
	let mut launch_plane_distance = Distance::from_m(0.0);
	let atmosphere = Atmosphere::default();

	// IMPORTANT when changing altitude anywhere move this function too
	let rho = altitude_to_rho(altitude);

	let mut launch_distance = Distance::from_m(0.0);

	let gravity = Acceleration::from_mps2(GRAVITY * if launch_parameters.use_gravity { 1.0 } else { 0.0 });
	let area = PI * (missile.caliber / 2.0).powi(2);
	let launch_velocity = ias;

	// Target parameters
	let target_velocity = launch_parameters.target_speed;
	let mut target_distance = launch_parameters.distance_to_target;

	// Statistical values
	let mut closest = Distance::from_m(f64::MAX);
	let mut max_v = Velocity::from_mps(0.0);
	let mut max_a = Acceleration::from_mps2(0.0);
	let mut min_a = Acceleration::from_mps2(0.0);
	let mut splash: Splash = Splash { splash: false, at: Distance::from_m(0.0)};

	for i in 0..sim_len {
		drag_force = Force::from_N(0.5 * rho * ias.to_mps().powi(2) * missile.cxk * area);

		// Current engine stage
		let engine_stage: EngineStage;

		// Current mass of missile
		let mass: Mass<f64>;

		// Sum of forces applied to missile
		let force: Force<f64>;

		let burn_0 = 0.0..missile.timefire0;
		let burn_1 = burn_0.end..burn_0.end + missile.timefire1;

		let flight_time = f64::from(i) * timestep;
		let compute_delta_mass = |mass, mass_end, timefire, relative_time| {
			Mass::from_kg((mass - mass_end) * ((flight_time - relative_time) / timefire))
		};

		match () {
			// Booster stage
			_ if burn_0.contains(&flight_time) => {
				mass = Mass::from_kg(missile.mass) - compute_delta_mass(missile.mass, missile.mass_end, missile.timefire0, 0.0);
				force = Force::from_N(missile.force0);
				engine_stage = Running { level: 0 };
			}
			// Sustainer stage
			_ if burn_1.contains(&flight_time) => {
				mass = Mass::from_kg(missile.mass_end) - compute_delta_mass(missile.mass_end, missile.mass_end1, missile.timefire1, missile.timefire0);
				force = Force::from_N(missile.force1);
				engine_stage = Running { level: 1 };
			}
			// Coasting
			_ => {
				if missile.mass_end1 != 0.0 {
					mass = Mass::from_kg(missile.mass_end1);
				} else {
					mass = Mass::from_kg(missile.mass_end);
				}
				force = Force::from_N(0.0);
				engine_stage = BurntOut;
			}
		}

		a = ((force - drag_force) / mass) - gravity;

		target_distance += Distance::from_m(target_velocity.to_mps()) * timestep;
		launch_distance += Distance::from_m(launch_velocity.to_mps()) * timestep;
		launch_plane_distance += Distance::from_m(launch_parameters.start_velocity.to_mps()) * timestep;


		ias += Velocity::from_mps(a.to_mps2()) * timestep;
		let tas = ias_to_tas(ias, &atmosphere, altitude);
		distance += Distance::from_m(tas.to_mps()) * timestep;


		if ias > max_v {
			max_v = ias;
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
		results.profile.v.push(ias);
		results.profile.d.push(distance);

		if debug {
			println!("ts(s): {} D(m): {} Dt(m): {} a(m/s²): {} v(m/s): {} d(N): {} rho: {} m(kg): {} s: {}",
					 format!("{:.1}", (i as f64 * timestep)).to_string().pad_to_width(4),
					 distance.to_meters().round().to_string().pad_to_width(5),
					 target_distance.to_meters().round().to_string().pad_to_width(4),
					 a.to_mps2().round().to_string().pad_to_width(3),
					 ias.to_mps().round().to_string().pad_to_width(4),
					 drag_force.to_N().round().to_string().pad_to_width(5),
					 rho.to_string()[..6].pad_to_width(4),
					 mass.to_string()[..5].pad_to_width(5),
					 engine_stage.to_string().pad_to_width(1),
			);
		}

		if target_distance != Distance::from_m(0.0) && target_distance < distance {
			splash.splash = true;
			splash.at = target_distance - launch_distance;
		}
	}

	if debug {
		println!("max velocity: {}m/s", max_v.to_mps().round());
		println!("max distance reached: {}m", distance.to_meters().round());
		if launch_parameters.target_speed != Velocity::from_mps(0.0) {
			println!("min missile - target: {}m", closest.to_meters().round());
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