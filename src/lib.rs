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


struct AppState {
    cube: SceneNode,
    vel: Translation3<f32>,	
}

impl State for AppState {
    fn step(&mut self, w: &mut Window) {
        self.cube.prepend_to_local_translation(&self.vel);
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
    let state = AppState { cube: c, vel: v };

    window.render_loop(state)
}
