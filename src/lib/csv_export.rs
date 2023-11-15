use std::io::{Cursor, Write};
use wt_datamine_extractor_lib::missile::missile::Missile;
use zip::CompressionMethod;
use zip::write::FileOptions;
use crate::runner::LaunchResults;



impl LaunchResults {
	// Returns zipped files containing results
	pub fn as_csv(&self, plot: Option<&[u8]>, missile: Missile) -> Option<Vec<u8>> {
		let out = Cursor::new(vec![]);
		let mut zip = zip::ZipWriter::new(out);

		let params_toml = toml::to_string(&self).ok()?;
		zip.start_file("launch_parameters.toml", FileOptions::default().compression_method(CompressionMethod::Stored)).ok()?;
		zip.write_all(
			format!(
				"# wt_ballistics_cal commit-hash: {} \n# Repository: {}\n",
				env!("GIT_HASH"),
				"https://github.com/Warthunder-Open-Source-Foundation/wt_ballistics_calc",
			)
				.as_bytes()
		).ok()?;
		zip.write_all(params_toml.as_bytes()).ok()?;

		if let Some(plot) = plot {
			zip.start_file("data_plot.png", FileOptions::default().compression_method(CompressionMethod::Stored)).ok()?;
			zip.write_all(plot).ok()?;
		}

		zip.start_file("plot.csv", FileOptions::default().compression_method(CompressionMethod::Stored)).ok()?;

		let mut csv = format!("acceleration;velocity;distance_traveled;turn_radius\n");

		let turning_radius = |velocity: f64| {
			velocity.powi(2) / missile.reqaccelmax
		};
		let profile = &self.profile;
		let a = &profile.a;
		let v = &profile.v;
		let d = &profile.d;
		for (a, v, d) in a.iter().zip(v.iter()).zip(d.iter()).map(|e|(e.0.0, e.0.1, e.1)) {
			csv.push_str(&format!("{a};{v};{d};{}\n", turning_radius(*v)));
		}
		zip.write_all(csv.as_bytes()).ok()?;


		Some(zip.finish().ok()?.into_inner())
	}
}