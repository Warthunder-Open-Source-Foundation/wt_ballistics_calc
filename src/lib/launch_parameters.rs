use simple_si_units::base::Distance;
use simple_si_units::mechanical::Velocity;

#[derive(Copy, serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct LaunchParameter {
	pub use_gravity: bool,
	pub start_velocity: Velocity<f64>,
	pub distance_to_target: Distance<f64>,
	pub target_speed: Velocity<f64>,
	pub altitude: Distance<f64>,
}

impl LaunchParameter {
	pub fn new_from_parameters(use_gravity: bool, start_velocity: f64, distance_to_target: f64, target_speed: f64, altitude: f64) -> Self {
		Self {
			use_gravity,
			start_velocity: Velocity::from_mps(start_velocity),
			distance_to_target: Distance::from_m(distance_to_target),
			target_speed: Velocity::from_mps(target_speed),
			altitude: Distance::from_m(altitude),
		}
	}
	pub fn new_from_default_hor() -> Self {
		Self {
			use_gravity: false,
			start_velocity: Velocity::from_mps(343.0),
			distance_to_target: Distance::from_m(0.0),
			target_speed: Velocity::from_mps(0.0),
			altitude: Distance::from_m(1000.0),
		}
	}
}