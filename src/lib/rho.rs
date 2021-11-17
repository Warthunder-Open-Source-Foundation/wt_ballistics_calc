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

	return match altitude {
		// Values below 0 remain base altitude as they are too unpredictable
		x if x <= 0 => {
			CONSTANTS[0].1
		}
		// Values above 10km are interpolated and will use a rough algorithm
		x if x > 10000 => {
			altitude as f64 / 10000.0 * 0.3142
		}
		_ => {
			let mut closest = u32::MAX;
			let mut best = -0.0;
			for constant in CONSTANTS {
				if altitude < closest {
					closest = altitude;
					best = constant.1;
				}
			}
			best
		}
	};
}