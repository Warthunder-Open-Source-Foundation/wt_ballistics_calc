pub fn altitude_to_rho(altitude: u32) -> f64 {
	return 3.0099_f64 - (0.868106 * ((0.0014659 * altitude as f64) + 7.88959).ln());

	// 3.0099 - 0.868106 ln (0.0014659x + 7.88959)
	// static CONSTANTS: [(u32, f64); 21] = [
	// 	(0, 1.209),
	// 	(500, 1.139),
	// 	(1000, 1.072),
	// 	(1500, 1.0083),
	// 	(2000, 0.9478),
	// 	(2500, 0.8903),
	// 	(3000, 0.8356),
	// 	(3500, 0.7837),
	// 	(4000, 0.7344),
	// 	(4500, 0.6877),
	// 	(5000, 0.6434),
	// 	(5500, 0.6014),
	// 	(6000, 0.5617),
	// 	(6500, 0.5241),
	// 	(7000, 0.4886),
	// 	(7500, 0.4551),
	// 	(8000, 0.42345),
	// 	(8500, 0.3936),
	// 	(9000, 0.3655),
	// 	(9500, 0.339),
	// 	(10000, 0.3142),
	// ];
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rho() {
		println!("{}", altitude_to_rho(10000));
	}
}