use std::io;

use plotters::prelude::*;
use wt_datamine_extractor_lib::missile::missile::Missile;

use wt_ballistics_calc_lib::launch_parameters::LaunchParameter;
use wt_ballistics_calc_lib::runner::{generate, LaunchResults};

fn main() {
	let target = 27;
	let missiles: Vec<Missile> = serde_json::from_str(&std::fs::read_to_string("../wt_datamine_extractor/missile_index/all.json").unwrap()).unwrap();
	let results = generate(&missiles[target], &LaunchParameter::new_from_default_hor(), 0.1, true);
	println!("{}", missiles[target].name);

	let mut v_profile: Vec<(f32, f64)> = Vec::new();
	for i in results.profile.v.clone().iter().enumerate() {
		v_profile.push((i.0 as f32, *i.1 as f64));
	}

	let mut a_profile: Vec<(f32, f64)> = Vec::new();
	for i in results.profile.a.clone().iter().enumerate() {
		a_profile.push((i.0 as f32, *i.1 as f64));
	}

	let mut d_profile: Vec<(f32, f64)> = Vec::new();
	for i in results.profile.d.clone().iter().enumerate() {
		d_profile.push((i.0 as f32, *i.1 / 20 as f64));
	}

	let root = BitMapBackend::new("5.png", (640, 480)).into_drawing_area();
	root.fill(&WHITE);
	let root = root.margin(10, 10, 10, 10);
	// After this point, we should be able to draw construct a chart context
	let mut chart = ChartBuilder::on(&root)
		// Set the size of the label region
		.x_label_area_size(20)
		.y_label_area_size(40)
		// Finally attach a coordinate on the drawing area and make a chart context
		.build_cartesian_2d(0f32..results.profile.sim_len as f32 * 1.1, -(results.min_a + 200.0)..(results.max_v + 50.0)).unwrap();

	// Then we can draw a mesh
	chart
		.configure_mesh()
		// We can customize the maximum number of labels allowed for each axis
		.x_labels(10)
		.y_labels(10)
		// We can also change the format of the label text
		.y_label_formatter(&|x| format!("{:.3}", x))
		.draw().unwrap();


	// And we can draw something in the drawing area
	chart.draw_series(LineSeries::new(
		v_profile,
		&RED,
	)).unwrap();

	chart.draw_series(LineSeries::new(
		a_profile,
		&BLUE,
	)).unwrap();

	chart.draw_series(LineSeries::new(
		d_profile,
		&GREEN,
	)).unwrap();
}