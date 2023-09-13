use std::io::{Cursor, Write};
use zip::CompressionMethod;
use zip::write::FileOptions;
use crate::runner::LaunchResults;



impl LaunchResults {

	// Returns zipped files containing results
	pub fn as_csv(&self, plot: Option<&[u8]>) -> Option<Vec<u8>> {
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

		let mut csv = format!("acceleration;velocity;distance_traveled\n");

		let profile = &self.profile;
		let a = &profile.a;
		let v = &profile.d;
		let d = &profile.d;
		for (a, v, d) in a.iter().zip(v.iter()).zip(d.iter()).map(|e|(e.0.0, e.0.1, e.1)) {
			csv.push_str(&format!("{a};{v};{d}\n"));
		}
		zip.write_all(csv.as_bytes()).ok()?;


		Some(zip.finish().ok()?.into_inner())
	}
}