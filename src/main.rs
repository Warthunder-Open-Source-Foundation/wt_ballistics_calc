use std::collections::HashMap;
use std::f64::consts::{E, PI};

use pad::PadStr;

const MASS: f64 = 44.0;

const MASS_END: f64 = 34.0;

const GRAVITY: f64 = 9.81;

const DIAM: f64 = 0.21;

const CXK: f64 = 3.1;

const RHO_0: f64 = 1.225;
const RHO_500: f64 = 1.167;
const RHO_1000: f64 = 1.11;
const RHO_2000: f64 = 1.0;
const RHO_4000: f64 = 0.819;
const RHO_6000: f64 = 0.66;
const RHO_8000: f64 = 0.525;
const RHO_10000: f64 = 0.413;

const TIMEFIRE0: f64 = 3.0;

const THRUST0: f64 = 95000.0;

const ENDSPEED: f64 = 1000.0;

const TIMESTEP: f64 = 0.1;

const SIMTIME: f64 = 21.0;

fn main() {
	let gravity = GRAVITY * 0.0;

	let mut a: f64;
	let area = PI * (DIAM / 2.0).powi(2);

	let mut drag_force: f64;

	let mut velocity: f64 = 367.0;

	let mut distance: f64 = 0.0;

	let launch_velocity = velocity;
	let mut launch_distance = 0.0;

	let target_velocity = 343.0;
	let mut target_distance = 2000.0;

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
	}
	println!("max v: {}", max_v.round());
	println!("delta target-missile: {}m", closest.round());
}


fn old_formula() {
	let k = 0.5 * RHO_0 * CXK * PI * (DIAM / 2.0).powi(2);

	let mg = MASS * GRAVITY;

	let q = ((THRUST0 - mg) / k).sqrt();

	let x = (2.0 * k * q) / MASS;

	let v = q * ((1.0 - E.powf(-x - TIMEFIRE0)) / (1.0 + E.powf(-x * TIMEFIRE0)));

	let tmg = THRUST0 - mg;

	let kv2 = k * v.powi(2);

	let y1 = -MASS / 2.0 * k * ((tmg - kv2) / tmg).ln();

	let yc = (MASS / 2.0 * k) * ((mg + kv2) / mg).ln();

	let yd = y1 + yc;
	eprintln!("yd = {:?}", yd);
}