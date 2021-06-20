extern crate kiss3d;
extern crate alloc;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector3, Point2, Point3};
use kiss3d::text::Font;
use std::path::Path;

use wasm_bindgen::prelude::*;


struct AppState {
    c: SceneNode,
    rot: UnitQuaternion<f32>,
	font: alloc::rc::Rc<Font>
}

impl State for AppState {
    fn step(&mut self, w: &mut Window) {
        self.c.prepend_to_local_rotation(&self.rot);
		w.draw_text(
			"Hi there! I'm David, and here's a cube.",
			&Point2::new(100.0, 80.0),
			60.0,
			&self.font, 
			&Point3::new(229.0/255.0, 229.0/255.0, 229.0/255.0),
		);

    }
}

#[wasm_bindgen(start)]
pub fn main() {	
    let mut window = Window::new("Kiss3d: wasm example");
    let mut c = window.add_cube(0.1, 0.1, 0.1);
 
    c.set_color(97.0/255.0, 97.0/255.0, 97.0/255.0);

    window.set_light(Light::StickToCamera);
	window.set_background_color(17.0/255.0, 19.0/255.0, 19.0/255.0);
	let font = Font::new(&Path::new("./LiberationMono-Regular.ttf")).unwrap_or(Font::default());

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let state = AppState { c, rot, font};

    window.render_loop(state)
}
