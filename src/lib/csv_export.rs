use std::io::{Cursor, Write};
use wt_datamine_extractor_lib::missile::missile::Missile;
use zip::{CompressionMethod, ZipWriter};
use zip::write::FileOptions;
use crate::runner::LaunchResults;



impl LaunchResults {
	// Returns zipped files containing results
	pub fn as_csv(&self, plot: Option<&[u8]>, missile: Missile) -> Option<Vec<u8>> {
		let zip = self.start_csv(plot, missile);
		Self::finish_zip(zip)
	}

	pub fn start_csv(&self,  plot: Option<&[u8]>, missile: Missile) -> MissileZipWriter {
		let out= Cursor::new(vec![]);
		let mut zip = ZipWriter::new(out);
		self.write_into_zip(plot, missile, &mut zip);
		zip
	}

	pub fn finish_zip( zip: MissileZipWriter) -> Option<Vec<u8>> {
		Some(zip.finish().ok()?.into_inner())
	}

	pub fn write_into_zip(&self, plot: Option<&[u8]>, missile: Missile, zip: &mut MissileZipWriter) -> Option<()> {
		let options: FileOptions<()> = FileOptions::default().compression_method(CompressionMethod::Stored);
		let fname = |f| format!("{}/{f}", missile.name);
		zip.add_directory(fname(""), options).ok()?;

		let params_toml = toml::to_string(&self).ok()?;
		zip.start_file(fname("launch_parameters.toml"), options).ok()?;
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
			zip.start_file(fname("data_plot.png"), options).ok()?;
			zip.write_all(plot).ok()?;
		}

		zip.start_file(fname("plot.csv"), options).ok()?;

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
		Some(())
	}
}


pub type MissileZipWriter = ZipWriter<Cursor<Vec<u8>>>;