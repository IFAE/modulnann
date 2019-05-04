extern crate nannou;
extern crate modulator;
extern crate rand;

use nannou::prelude::*;

#[allow(unused_imports)]
use modulator::sources::{
    Newtonian, ScalarGoalFollower, ScalarSpring, ShiftRegister, ShiftRegisterInterp, Wave,
};

#[allow(unused_imports)]
use modulator::{Modulator, ModulatorEnv};

use std::{f32, time::Instant};
use rand::prelude::*;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
	m1: ModulatorEnv<f32>,

	earlier: Instant,
}

impl Model {
	fn new() -> Self {
		Model{
			m1: ModulatorEnv::new(),
			earlier: Instant::now()
		}
	}
}

fn setup_modulators(st: &mut Model){

	// Create a sine wave modulator, initial amplitude of 1 and frequency of 0.5Hz
    let wave = Wave::new(1.0, 0.5).wave(Box::new(|w, t| {
            (t * w.frequency * f32::consts::PI * 2.0).sin() * w.amplitude
        }));
    st.m1.take("wave_sin", Box::new(wave));


    // Make a random wave modulator
    let wave = Wave::new(2.0, 0.1).wave(Box::new(|w, _| {
	    let n = w.value + thread_rng().gen_range(-w.frequency, w.frequency);
        f32::min(f32::max(n, -w.amplitude), w.amplitude)
    }));

    st.m1.take("rnd_wave", Box::new(wave));

    
}

fn time_delta(earlier: &mut Instant) -> u64 {
    let now = Instant::now();
    let dt = ModulatorEnv::<f32>::duration_to_micros(now.duration_since(*earlier));
    *earlier = now;

    dt
}

fn model(app: &App) -> Model {
    let _window = app.new_window().with_dimensions(720, 720).build().unwrap();
    let mut model = Model::new();
    setup_modulators(&mut model);
    model
}

fn event(_app: &App, mut model: Model, event: Event) -> Model {
	//let mut testing = 1u32;

	//let mut earlier = Instant::now();

    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            Moved(_pos) => {}

            KeyPressed(_key) => {}

            KeyReleased(_key) => {}

            MouseMoved(_pos) => {}

            MouseDragged(_pos, _button) => {}

            MousePressed(_button) => {}

            MouseReleased(_button) => {}

            MouseEntered => {}

            MouseExited => {}

            Resized(_size) => {}

            _other => (),
        },

        Event::Update(_dt) => {
        	let dt = time_delta(&mut model.earlier);
        	model.m1.advance(dt);
        }

        _ => (),
    }
    model
}

fn view(app: &App, _model: &Model, frame: Frame) -> Frame {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to pink.
    draw.background().color(LIGHT_PURPLE);

    // Draw a red ellipse with default size and position.
    draw.ellipse()
    	.radius(20.0 + 100.0*_model.m1.value("rnd_wave"))
    	.color(DARK_BLUE);

   //

    draw.rect()
        .color(RED)
        .x_y(30.0,30.0)
        .w(40.0 + 100.0*_model.m1.value("wave_sin"))
        .h(80.0);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}