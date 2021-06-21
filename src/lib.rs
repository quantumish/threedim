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
    cube: SceneNode,
    state: LinearState,
	t: f64,	
}

impl State for AppState {
    fn step(&mut self, w: &mut Window) {
		let vec = integrate(&mut self.state, self.t, dt);
        self.cube.prepend_to_local_translation(&Translation3::new(vec.x as f32, vec.y as f32, vec.z as f32));
    }
}

#[wasm_bindgen(start)]
pub fn main() {	
    let mut window = Window::new("Kiss3d: wasm example");
    let mut c = window.add_cube(0.1, 0.1, 0.1);
 
    c.set_color(97.0/255.0, 97.0/255.0, 97.0/255.0);

    window.set_light(Light::StickToCamera);
	window.set_background_color(17.0/255.0, 19.0/255.0, 19.0/255.0);

    let v: Translation3<f32> = Translation3::new(0.0, -0.001, 0.0);
	let s = LinearState {
		position: Vector3::new(0.0, 0.0, 0.0),
		velocity: Vector3::new(0.0, 0.0, 0.0),
		momentum: Vector3::new(0.0, -0.01, 0.0),
		mass: 1.0,
		inv_mass: 1.0,
	};
    let state = AppState { cube: c, state: s, t: 0.0 };

    window.render_loop(state)
}
