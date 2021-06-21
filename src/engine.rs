pub use kiss3d::nalgebra::Vector3;

pub struct LinearState {
	pub position: Vector3<f64>,
	pub momentum: Vector3<f64>,
	pub velocity: Vector3<f64>,
	pub mass: f32,
	pub inv_mass: f32,
}

impl LinearState {
	fn recalculate(&mut self) {
		self.velocity = self.momentum * self.inv_mass;
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

pub fn acceleration(state: &LinearState, t: f64) -> f64 {
	return -9.8;
}

pub fn eval(initial: &LinearState, t: f64, dt: f64, d: &LinearDerivative) -> LinearDerivative {
	let state: LinearState;
	state.position = initial.position + d.velocity * dt;
	state.velocity = initial.momentum / initial.mass;
	state.momentum = initial.momentum + d.force * dt;

	let out: LinearDerivative;
	out.velocity = state.velocity;
	out.force = acceleration(state, t+dt) * state.mass;
	return out;
}

pub fn integrate(state: &LinearState, t: f64, dt: f64) -> Vector3<f64> {
	let (a, b, c, d): (LinearDerivative, LinearDerivative, LinearDerivative, LinearDerivative);
	a = eval(state, t, 0.0, LinearDerivative {});
	b = eval(state, t, dt*0.5, a);
	c = eval(state, t, dt*0.5, b);
	d = eval(state, t, dt, c);

	let dxdt = 1.0 / 6.0* (a.velocity + 2.0 * (b.velocity + c.velocity) + d.velocity);
	let dvdt = 1.0 / 6.0 *
		(a.momentum/state.mass + 2.0 * (b.momentum/state.mass + c.momentum/state.mass) + d.momentum/state.mass);
	let dmdt = 1.0 / 6.0* (a.force + 2.0 * (b.force + c.force) + d.force);

	state.position += dxdt * dt;
	state.velocity += dvdt * dt;
	state.momentum += dmdt * dt;
	return dxdt * dt;
}

