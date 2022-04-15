pub mod engine;
pub use crate::engine::*;

use legion::*;

use kiss3d::light::Light;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector3, Point2, Point3, Translation3};
use kiss3d::text::Font;
use kiss3d::scene::SceneNode;
use std::path::Path;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const TIME_STEP: f64 = 0.001;

struct AppState {
    w: World,   
}

impl State for AppState {
    fn step(&mut self, w: &mut Window) {
        *w.scene_mut() = SceneNode::new_empty();

        let mut query = <(&mut Velocity, &mut Position)>::query();
        for (velocity, position) in query.iter_mut(&mut self.w) {
            velocity.y -= 0.001;
            position.x += velocity.x;
            position.y += velocity.y;
        }

	unsafe {
	    let mut query = <(Entity, &mut Velocity, &mut Position, &Shape)>::query();
	    for (e, velocity, position, shape) in query.iter_unchecked(&self.w) {
		for (e2, position2, shape2) in <(Entity, &Position, &Shape)>::query().iter_unchecked(&self.w) {	    
		    if e == e2 { continue; }
		    if (position.y-position2.y).abs() < (shape2.dx/2.0)+(shape.dx/2.0) {
			position.y = position2.y + shape2.dx/2.0;
			*velocity = Velocity { x: 0.0, y: 0.0, z: 0.0 };
		    }
		}
	    }
	}

        let mut query = <(&Position, &Shape)>::query();
        for (position, shape) in query.iter(&self.w) {
            let mut node = w.add_cube(shape.dx, shape.dx, shape.dx);
            node.set_local_translation(Translation3::new(position.x, position.y, position.z));
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() { 
    let mut window = Window::new("threedim");
    
    window.set_light(Light::StickToCamera);    
    window.set_background_color(17.0/255.0, 19.0/255.0, 19.0/255.0);
    
    let mut world = World::default();
    world.extend(vec![
        (Shape { dx: 1.0 }, Position { x: 0.0, y: 0.0, z: 0.0 }, Velocity { x: 0.06, y: 0.0, z: 0.0 }),
        (Shape { dx: 1.0 }, Position { x: 1.0, y: 2.0, z: 0.0 }, Velocity { x: 0.0, y: 0.05, z: 0.0 }),
    ]);
    world.push((Shape { dx: 100.0 }, Position { x: 0.0, y: -100.0, z: 0.0 }));

    // let mut query = <&Position>::query();
    // for position in query.iter(&self.w) {
    //  let mut node = w.add_cube(1.0,1.0,1.0);
    // }
    
    let state = AppState { w: world };    
    window.render_loop(state)   
}
