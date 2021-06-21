pub mod engine;
pub use crate::engine::*;
extern crate kiss3d;
extern crate alloc;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector3, Point2, Point3, Translation3};
use kiss3d::text::Font;
use std::path::Path;

use wasm_bindgen::prelude::*;

const dt: f64 = 0.001;

struct AppState {
    objs: Vec<Object>,
	t: f64,	
}

impl State for AppState {
    fn step(&mut self, w: &mut Window) {
		for obj in self.objs.iter_mut() {
			if obj.immovable {continue;}
			integrate(&mut obj.lstate, self.t, dt);
			obj.node.set_local_translation(Translation3::new(obj.lstate.position.x as f32, obj.lstate.position.y as f32, obj.lstate.position.z as f32));
		}
    }
}

#[wasm_bindgen(start)]
pub fn main() {	
    let mut window = Window::new("Kiss3d: wasm example");
	let mut objs: Vec<Object> = Vec::new();
	
    objs.push(Object {
		node: window.add_cube(0.1, 0.1, 0.1),
		lstate: LinearState {
			position: Vector3::new(0.0, 0.0, 0.0),
			velocity: Vector3::new(0.0, 0.0, 0.0),
			momentum: Vector3::new(100.0, 0.0, 400.0),
			mass: 100.0,
			inv_mass: 1.0, 
		},
		shape: Shape::Cube(0.1, 0.1, 0.1),
		immovable: false,
	});
	let tmp = objs.len();
    objs[tmp-1].node.set_color(97.0/255.0, 97.0/255.0, 97.0/255.0);

	objs.push(Object {
		node: window.add_cube(0.5, 0.02, 0.5),
		lstate: LinearState {
			position: Vector3::new(0.0, 0.0, 0.0),
			velocity: Vector3::new(0.0, 0.0, 0.0),
			momentum: Vector3::new(0.0, 0.0, 0.0),
			mass: 0.0,
			inv_mass: 0.0, 
		},
		shape: Shape::Cube(0.5, 0.02, 0.5),
		immovable: true,
	});
	let tmp = objs.len();
	objs[tmp-1].node.set_local_translation(Translation3::new(0.0, -0.5, 0.0));

	
    window.set_light(Light::StickToCamera);
	window.set_background_color(17.0/255.0, 19.0/255.0, 19.0/255.0);

    let v: Translation3<f32> = Translation3::new(0.0, -0.001, 0.0);
    let state = AppState { objs: objs, t: 0.0 };

    window.render_loop(state)
}
