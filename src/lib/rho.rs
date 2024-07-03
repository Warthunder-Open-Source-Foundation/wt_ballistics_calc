
// True to real life, but inaccurate for the game
mod realistic {
	use std::ops::Mul;

	/// Implements `https://www.grc.nasa.gov/www/k-12/airplane/atmosmet.html`
	pub fn altitude_to_rho(altitude: u32) -> f64 {
		alt_to_pressure(altitude) / (0.2869 * (18.0 + 273.1))
	}

	pub fn alt_to_pressure(altitude: u32) -> f64 {
		let t = 15.04 - 0.00649.mul(altitude as f64);
		let fraction = (t + 273.1) / 288.08;
		let frac_exp = fraction.powf(5.256);

		let pressure = 101.29 * frac_exp;
		pressure
	}
}

pub fn altitude_to_rho(altitude: u32) -> f64 {
	let alt = altitude as f64 / 1000.0;
	// y=−0.000036x³+0.002048x²−0.047574x+0.567771
	-0.000036 * alt.powi(3) + 0.002048 * alt.powi(2) - 0.047574 * alt + 0.567771
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rho() {
		println!("{}", altitude_to_rho(10000));
	}

	#[test]
	fn test_parts() {
		(0..=10)
			.map(|e|e * 1000)
			.map(|e|(e, altitude_to_rho(e)))
			.for_each(|(a, e)|eprintln!("alt:{a}m {e:.3}"))
	}
}