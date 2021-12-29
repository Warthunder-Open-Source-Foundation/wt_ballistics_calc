
use std::time::Instant;

use plotters::prelude::*;

use wt_datamine_extractor_lib::missile::missile::Missile;

use wt_ballistics_calc_lib::launch_parameters::LaunchParameter;
use wt_ballistics_calc_lib::runner::{generate};

const WIDTH: u32 = 3840;
const HEIGHT: u32 = 2160;

const TIMESTEP: f64 = 0.001;

fn main() {
	let start = Instant::now();

	let target = 18;
	let missiles: Vec<Missile> = serde_json::from_str(&std::fs::read_to_string("../wt_datamine_extractor/missile_index/all.json").unwrap()).unwrap();
	let results = generate(&missiles[target], &LaunchParameter::new_from_default_hor(), TIMESTEP, false);
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
		d_profile.push((i.0 as f32, *i.1 / 10 as f64));
	}


	let root = BitMapBackend::new("5.png", (WIDTH, HEIGHT)).into_drawing_area();
	root.fill(&WHITE).unwrap();
	let root = root.margin(10, 10, 10, 10);
	// After this point, we should be able to draw construct a chart context
	let mut chart = ChartBuilder::on(&root)
		// Set the size of the label region
		.x_label_area_size(20)
		.y_label_area_size(40)
		.set_label_area_size(LabelAreaPosition::Bottom, 40)
		.caption(&format!("{}", &missiles[target].localized), ("sans-serif", 50))
		// Finally attach a coordinate on the drawing area and make a chart context
		.build_cartesian_2d(0f32..results.profile.sim_len as f32 * 1.1, -(results.min_a.abs() + 50.0)..(results.max_v + 50.0)).unwrap();

	// Then we can draw a mesh
	chart
		.configure_mesh()
		// We can customize the maximum number of labels allowed for each axis
		.x_labels(50)
		.y_labels(50)
		.x_desc("time in s")
		// We can also change the format of the label text
		.y_label_formatter(&|x| format!("{:.0}", x))
		.x_label_formatter(&|x| format!("{}", x /  TIMESTEP.powi(-1) as f32))
		.draw().unwrap();


	// And we can draw something in the drawing area
	chart.draw_series(LineSeries::new(
		v_profile,
		&RED,
	)).unwrap()
		.label("Velocity m/s")
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + (WIDTH / 50) as i32, y)], &RED));

	chart.draw_series(LineSeries::new(
		a_profile,
		&BLUE,
	)).unwrap()
		.label("Acceleration m/s²")
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + (WIDTH / 50) as i32, y)], &BLUE));

	chart.draw_series(LineSeries::new(
		d_profile,
		&GREEN,
	)).unwrap()
		.label("Distance m / 10")
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + (WIDTH / 50) as i32, y)], &GREEN));

	chart.draw_series(LineSeries::new(
		vec![(0.0, 0.0), (WIDTH as f32 * TIMESTEP.powi(-1) as f32, 0.0)],
		&BLACK,
	)).unwrap();

	// chart.draw_series(
	// 	vec![(3.1_f32, 4.1)].iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
	// ).unwrap();

	chart.configure_series_labels()
		.border_style(&BLACK)
		.background_style(&WHITE.mix(0.8))
		.legend_area_size(WIDTH / 40)
		.label_font(("sans-serif", 20))
		.draw().unwrap();

	println!("{:?}", start.elapsed());
}