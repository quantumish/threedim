
pub mod engine;
pub use crate::engine::*;

use legion::*;

use kiss3d::light::Light;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector3, Point2, Point3, Translation3};
use kiss3d::text::Font;
use kiss3d::scene::SceneNode;
use std::path::Path;

use kiss3d::conrod;
use kiss3d::conrod::color::Color;
use kiss3d::conrod::position::Positionable;
use kiss3d::conrod::widget_ids;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const TIME_STEP: f64 = 0.001;

pub struct DemoApp {
    sz: usize,
    text_box: String,
    text_edit: String,
}


widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // The title and introduction widgets.
        title,
        introduction,
        // Shapes.
        shapes_canvas,
        rounded_rectangle,
        shapes_left_col,
        shapes_right_col,
        shapes_title,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,
        // Image.
        image_title,
        cat,
        // Button, XyPad, Toggle.
        button_title,
        button,
        xy_pad,
        toggle,
        ball,
        // NumberDialer, PlotPath
        dialer_title,
        number_dialer,
        plot_path,
        // TextBox and TextEdit
        text_box,
        text_edit,
        // Scrollbar
        canvas_scrollbar,
    }
}

struct AppState {
    w: World,
    ids: Ids,
    a: DemoApp,
}

impl DemoApp {
    /// Simple constructor for the `DemoApp`.
    pub fn new() -> Self {
        DemoApp {
            sz: 0,
            text_box: "Hello".to_string(),
            text_edit: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\nUt enim ad minim veniam...".to_string(),
        }
    }
}

pub fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut DemoApp) {
    use conrod::{widget, Colorable, Labelable, Sizeable, Widget};
    use std::iter::once;

    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 32;

    // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
    // By default, its size is the size of the window. We'll use this as a background for the
    // following widgets, as well as a scrollable container for the children widgets.
    const TITLE: &'static str = "\ncube.";
    widget::Canvas::new()
        .align_right()
        .w(300.0)
	.h(400.0)
        .set(ids.canvas, ui);

    ////////////////
    ///// TEXT /////
    ////////////////

    // We'll demonstrate the `Text` primitive widget by using it to draw a title and an
    // introduction to the example.
    widget::Text::new(TITLE)
        .font_size(TITLE_SIZE)
        .mid_top_of(ids.canvas)
        .set(ids.title, ui);
    const INTRODUCTION: &'static str =
        "A bunch of cubes.";
    widget::Text::new(INTRODUCTION)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.introduction, ui);

    for _press in widget::Button::new()
        .label("PRESS ME")
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.button_title, 60.0)
	.h(100.0)
        .set(ids.button, ui)
    {        
        app.sz += 1;
    }
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
	    let sz = shape.dx + self.a.sz as f32;
            let mut node = w.add_cube(sz, sz, sz);
            node.set_local_translation(Translation3::new(position.x, position.y, position.z));
        }

	let mut ui = w.conrod_ui_mut().set_widgets();
        gui(&mut ui, &self.ids, &mut self.a);	
    }
}

pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

#[wasm_bindgen(start)]
pub fn main() { 
    let mut window = Window::new("threedim");
    
    window.set_light(Light::StickToCamera);    
    window.set_background_color(17.0/255.0, 19.0/255.0, 19.0/255.0);
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());
    window.conrod_ui_mut().theme = theme();

    
    let mut world = World::default();
    world.extend(vec![
        (Shape { dx: 1.0 }, Position { x: 0.0, y: 0.0, z: 0.0 }, Velocity { x: 0.06, y: 0.0, z: 0.0 }),
        (Shape { dx: 1.0 }, Position { x: 1.0, y: 2.0, z: 0.0 }, Velocity { x: 0.0, y: 0.05, z: 0.0 }),
    ]);
    world.push((Shape { dx: 100.0 }, Position { x: 0.0, y: -100.0, z: 0.0 }));

    let state = AppState { w: world, ids,  a: DemoApp::new() };    
    window.render_loop(state)   
}
