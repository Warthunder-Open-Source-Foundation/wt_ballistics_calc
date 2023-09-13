use std::io::stdin;
use std::ops::Range;
use std::time::Instant;

use plotters::prelude::*;
use wt_datamine_extractor_lib::missile::missile::Missile;

use wt_ballistics_calc_lib::launch_parameters::LaunchParameter;
use wt_ballistics_calc_lib::runner::generate;

const WIDTH: u32 = 3840;
const HEIGHT: u32 = 2160;

const TIMESTEP: f64 = 0.1;

// Scaling settings
const FONT_AXIS: u32 = ((WIDTH + HEIGHT) / 2) as u32;
const DIST_C: u32 = 100;
const Y_RANGE: Range<f64> = -50.0..1040.0;

fn main() {
	let start = Instant::now();

	let missiles: Vec<Missile> = serde_json::from_str(&std::fs::read_to_string("../wt_datamine_extractor/missile_index/all.json").unwrap()).unwrap();

	let missile = Missile::select_by_name(&missiles, "us_aim7f_sparrow").unwrap();

	let params = LaunchParameter {
		use_gravity: false,
		start_velocity: 343.0,
		distance_to_target: 0.0,
		target_speed: 0.0,
		altitude: 7000,
	};
	let results = generate(&missile, params, TIMESTEP, false);
	println!("{}", "Finished simulation");
	dbg!(results.max_v);

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
		d_profile.push((i.0 as f32, *i.1 / DIST_C as f64));
	}
	println!("{}", "Calculated profiles");


	let root = BitMapBackend::new("5.png", (WIDTH, HEIGHT)).into_drawing_area();
	root.fill(&WHITE).unwrap();
	let root = root.margin(10, 10, 10, 10);

	println!("{}", "Created chart");

	let x_dim = 0f32..results.profile.sim_len as f32 * 1.1;
	let y_dim = -(results.min_a.abs() + 50.0).round()..(results.max_v + 50.0).round();
	println!("{:?} {:?}", x_dim, y_dim);

	// After this point, we should be able to draw construct a chart context
	let mut chart = ChartBuilder::on(&root)
		// Set the size of the label region
		.x_label_area_size(20)
		.y_label_area_size(40)
		.set_label_area_size(LabelAreaPosition::Bottom, FONT_AXIS / 50)
		.set_label_area_size(LabelAreaPosition::Left, FONT_AXIS / 50)
		.caption(&format!("{}", &missile.localized), ("sans-serif", FONT_AXIS / 20))
		// Finally attach a coordinate on the drawing area and make a chart context
		.build_cartesian_2d(x_dim, Y_RANGE).unwrap(); // Any y range >= 1000 breaks the tool

	// Then we can draw a mesh
	chart
		.configure_mesh()
		// We can customize the maximum number of labels allowed for each axis
		.x_labels(50)
		.y_labels(50)
		.x_desc("time in s")
		.x_label_style(("sans-serif", FONT_AXIS / 100))
		.y_label_style(("sans-serif", FONT_AXIS / 100))
		// We can also change the format of the label text
		.y_label_formatter(&|x| format!("{:.0}", x))
		.x_label_formatter(&|x| format!("{}", x / TIMESTEP.powi(-1) as f32))
		.draw().unwrap();

	println!("{}", "Configured meshes");


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
		.label(format!("Distance m / {DIST_C}"))
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + (WIDTH / 50) as i32, y)], &GREEN));

	chart.draw_series(LineSeries::new(
		vec![(0.0, 0.0), (WIDTH as f32 * TIMESTEP.powi(-1) as f32, 0.0)],
		&BLACK,
	)).unwrap();

	// let mach_lines = for i in 1..(results.max_v / 343.0).ceil() as u32 {
	// 	let target: f64 = (i * 343) as f64;
	// 	chart.draw_series(LineSeries::new(
	// 		vec![(0.0 as f32, target), (120000.0 as f32, target)],
	// 		&RED,
	// 	)).unwrap();
	// };

	// chart.draw_series(
	// 	vec![(3.1_f32, 4.1)].iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
	// ).unwrap();

	chart.configure_series_labels()
		.border_style(&BLACK)
		.background_style(&WHITE.mix(0.8))
		.legend_area_size(WIDTH / 40)
		.label_font(("sans-serif", FONT_AXIS / 50))
		.draw().unwrap();

	println!("{:?}", start.elapsed());
	let _zip = results.as_csv(None).unwrap();
}

pub fn input_launch_parameters(launch_parameters: &mut LaunchParameter) {
	println!("Enter start/carrier aircraft velocity in m/s (mach 1 = 343m/s)");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.start_velocity = line.trim().parse().unwrap();

	println!("Enter distance to target aircraft in m (0 if no target required)");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.distance_to_target = line.trim().parse().unwrap();

	println!("Enter target velocity in m/s (mach 1 = 343m/s)");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.target_speed = line.trim().parse().unwrap();

	println!("Enter altitude to simulate in in m");
	let mut line = "".to_owned();
	stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");
	launch_parameters.altitude = line.trim().parse().unwrap();
}