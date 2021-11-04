use std::collections::{BTreeMap};

pub fn altitude_to_rho(altitude: u32) -> f64 {
	static CONSTANTS: [(u32, f64); 21] = [
		(0, 1.209),
		(500, 1.139),
		(1000, 1.072),
		(1500, 1.0083),
		(2000, 0.9478),
		(2500, 0.8903),
		(3000, 0.8356),
		(3500, 0.7837),
		(4000, 0.7344),
		(4500, 0.6877),
		(5000, 0.6434),
		(5500, 0.6014),
		(6000, 0.5617),
		(6500, 0.5241),
		(7000, 0.4886),
		(7500, 0.4551),
		(8000, 0.42345),
		(8500, 0.3936),
		(9000, 0.3655),
		(9500, 0.339),
		(10000, 0.3142),
	];
	#[allow(clippy::cast_possible_truncation)]
	let map = IntoIterator::into_iter(CONSTANTS).map(|t| (t.0, t.1)).collect::<BTreeMap<u32, f64>>();

	for key in map.keys() {
		if altitude <= *key {
			println!("Matched key at: {}", key);
		return *map.get(key).unwrap();
		}
	}
	panic!("Cant find air density for altitude")
}