use kiss3d::scene::SceneNode;
pub use kiss3d::nalgebra::Vector3;

use legion::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {
    pub dx: f32,
}
    

#[system(for_each)]
fn update_positions(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x;
    pos.y += vel.y;
    pos.z += vel.z;
}

