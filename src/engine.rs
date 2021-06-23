use kiss3d::scene::SceneNode;
pub use kiss3d::nalgebra::Vector3;

#[derive(Clone)]
pub struct LinearState {
	pub position: Vector3<f64>,
	pub momentum: Vector3<f64>,
	pub velocity: Vector3<f64>,
	pub mass: f64,
	pub inv_mass: f64
}

impl LinearState {
	fn recalculate(&mut self) {
		self.velocity = self.momentum * self.inv_mass;
	}
	
	fn force(&self, t: f64) -> Vector3<f64> {
		return Vector3::new(0.0, -9.8, 0.0);
	}

}

pub struct LinearDerivative {
	pub velocity: Vector3<f64>,
	pub force: Vector3<f64>,
}

struct RotState {
	
}

struct RotDerivative {
	
}

pub fn eval(initial: &LinearState, t: f64, dt: f64, d: &LinearDerivative) -> LinearDerivative {
	let mut state: LinearState = initial.clone();
	state.position = initial.position + d.velocity * dt;
	state.velocity = initial.momentum / initial.mass;
	state.momentum = initial.momentum + d.force * dt;

	let out: LinearDerivative = LinearDerivative {
		velocity: state.velocity,
		force: initial.force(t+dt),
	};
	return out;
}

pub fn integrate(state: &mut LinearState, t: f64, dt: f64) -> Vector3<f64> {
	let (a, b, c, d): (LinearDerivative, LinearDerivative, LinearDerivative, LinearDerivative);
	a = eval(state, t, 0.0, &LinearDerivative {velocity: Vector3::new(0.0,0.0,0.0),
											   force: Vector3::new(0.0,0.0,0.0)});
	b = eval(state, t, dt*0.5, &a);
	c = eval(state, t, dt*0.5, &b);
	d = eval(state, t, dt, &c);

	let dxdt = 1.0 / 6.0 * (a.velocity + 2.0 * (b.velocity + c.velocity) + d.velocity);
	let dvdt = 1.0 / 6.0 * (state.momentum/state.mass + 2.0 * (state.momentum/state.mass + state.momentum/state.mass) + state.momentum/state.mass);
	let dmdt = 1.0 / 6.0* (a.force + 2.0 * (b.force + c.force) + d.force);

	state.position += dxdt * dt;
	state.velocity += dvdt * dt;
	state.momentum += dmdt * dt;
	return dxdt * dt;
}

#[derive(Clone, Copy)]
pub enum Shape {
	Cube(Vector3<f64>),
	// Sphere(f32),
}

#[derive(Clone)]
pub struct Object {
	pub node: SceneNode,
	pub lstate: LinearState,
	pub shape: Shape,
	pub immovable: bool,
}
