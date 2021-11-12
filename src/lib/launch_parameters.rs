#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct LaunchParameter {
	pub use_gravity: bool,
	pub start_velocity: f64,
	pub distance_to_target: f64,
	pub target_speed: f64,
	pub altitude: u32,
}

impl LaunchParameter {
	pub fn new_from_parameters(use_gravity: bool, start_velocity: f64, distance_to_target: f64, target_speed: f64, altitude: u32) -> Self {
		Self {
			use_gravity,
			start_velocity,
			distance_to_target,
			target_speed,
			altitude,
		}
	}
	pub fn new_from_default_hor() -> Self {
		Self {
			use_gravity: false,
			start_velocity: 343.0,
			distance_to_target: 0.0,
			target_speed: 0.0,
			altitude: 0
		}
	}
}