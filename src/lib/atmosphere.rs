use std::cmp::{Ordering};
use simple_si_units::base::Distance;

/// Implemented according to https://github.com/GaijinEntertainment/DagorEngine/blob/main/prog/gameLibs/gamePhys/props/atmosphere.cpp


// Public API with proper name
pub fn altitude_to_rho(altitude: Distance<f64>) -> f64 {
	Atmosphere::new(760.0, 18.0).density(altitude.to_meters())
}

#[cfg(test)]
mod tests {
	use simple_si_units::base::Distance;
	use super::*;

	#[test]
	fn rho() {
		println!("{}", altitude_to_rho(Distance::from_km(10.0)));
	}

	#[test]
	fn test_parts() {
		[0,1,2,3,5,8,10,12,15,20].into_iter()
			.map(|e|(e, altitude_to_rho(Distance::from_km(e as _))))
			.for_each(|(a, e)|eprintln!("alt:{a}m {e:.3}"))
	}
}

// Re-implementation below

const DENSITY: [f64; 5] = [1., -9.59387e-05, 3.53118e-09, -5.83556e-14, 2.28719e-19];
const PRESSURE: [f64; 5] = [1., -0.000118441, 5.6763e-09, -1.3738e-13, 1.60373e-18];
const TEMPARATURE: [f64; 5] = [1., -2.27712e-05, 2.18069e-10, -5.71104e-14, 3.97306e-18];

fn compute_polynomial(i: [f64; 5], v: f64) -> f64 {
	(((i[4] * v + i[3]) * v + i[2]) * v + i[1]) * v + i[0]
}

const STD_P0: f64 =  101300.; // Standard pressure at sea level, Pa
const STD_T0: f64 =  288.16;  // Standard temperature at sea level, K
const STD_RO0: f64 =  1.225;  // Standard density [kg/m3] t: f64 = 15`C, p: f64 = 760 mm/1013 gPa


// Uncommented globals are statics used as temporary storage, refer to the Atmosphere struct instead
const G: f64 =  9.81;       // Earth gravity
// const P0 : f64 =  101300.;   // Pressure at sea level, Pa
// const T0 : f64 =  288.16;    // Temperature at sea level, K
// const RO0: f64 =  1.225;    // Density [kg/m3] t: f64 = 15`C, p: f64 = 760 mm/1013 gPa
const MU0: f64 =  1.825e-6; // Viscosity [Pa*sec]
const H_MAX: f64 =  18300.0; // Maximal altitude
const WATER_DENSITY: f64 =  1000.0;


pub struct Atmosphere {
	pressure: f64,
	temperature: f64,
}

impl Atmosphere {
	/// Pressure in mm of Mercury column
	/// Temperature in degree C
	pub fn new(mut pressure: f64, mut temperature: f64) -> Self {
		pressure *= 101300. / 760.; // [Pa] , [N/m2];l
		temperature += 273.16;       // scale of Kelvin

		Self {
			pressure,
			temperature,
		}
	}

	/// Density [kg/m3] t: f64 = 15`C, p: f64 = 760 mm/1013 gPa
	fn calc_density(&self) -> f64 {
		1.225 * (self.pressure / 101300.) * (288.16 / self.temperature)
	}

	/// Get Pressure( H [meters] ) , [ Pa ]
	pub fn pressure(&self, h: f64) -> f64 {
		self.pressure * compute_polynomial(PRESSURE, min(h as _, H_MAX)) * (H_MAX / max(H_MAX, h as _))
	}

	/// Get Temperature( H [meters] ) , [ K ]
	pub fn temperature(&self, h: f64) -> f64 {
		self.temperature * compute_polynomial(TEMPARATURE, min(h as _, H_MAX))
	}

	/// Get Sonic Speed( H [meters] ) , [ m/s ]
	pub fn sonic_speed(&self, h: f64) -> f64 {
		20.1 * self.temperature(h).sqrt()
	}

	/// Get Density( H [meters] ) [kg*sec²/m⁴]
	pub fn density(&self, h: f64) -> f64 {
		self.calc_density() * compute_polynomial(DENSITY, min(h as _, H_MAX)) * (H_MAX / max(H_MAX, h as _))
	}

	/// Get {Dynamic Turbulent} viscosity( H [meters] ) , [ Pa*sec ]
	pub fn viscosity(&self, h: f64) -> f64 {
		MU0 * (self.temperature(h) / self.temperature).powf(0.76)
	}

	/// Get Kinetic(kinematic turbulent) Viscosity( T [K] ) , [ m2/sec ]
	pub fn kinetic_viscosity(&self, h: f64) -> f64 {
		self.viscosity(h) / self.density(h)
	}
}

fn min(l: f64, r: f64) -> f64 {
	if l.total_cmp(&r) == Ordering::Less {
		l
	} else {
		r
	}
}

fn max(l: f64, r: f64) -> f64 {
	if l.total_cmp(&r) == Ordering::Greater {
		l
	} else {
		r
	}
}