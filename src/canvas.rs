use wasm_bindgen::prelude::{JsCast, JsValue};
use web_sys::{console, CanvasRenderingContext2d, Path2d};

use crate::turtle::Turtle;
use crate::units::Angle;
use crate::{draw::*, Style};

impl Drawable for Draw<Path2d> {
    fn move_to(&mut self, x: f64, y: f64) {
        self.path.move_to(x, y);
    }

    fn line_to(&mut self, x: f64, y: f64) {
        self.path.line_to(x, y);
    }

    fn arc(&mut self, center_x: f64, center_y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        self.path
            .arc(center_x, center_y, radius, start_angle, end_angle)
            .unwrap();
    }
}

impl Default for Draw<Path2d> {
    fn default() -> Self {
        let path = Path2d::new().unwrap();
        Draw { path }
    }
}

pub fn get_context() -> Result<CanvasRenderingContext2d, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document.get_element_by_id("canvas").unwrap_or_else(|| {
        console::log_1(&"Canvas element not found".into());
        panic!("Canvas element not found");
    });
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .expect("Rendering context not found")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok(context)
}

pub fn _test(context: &CanvasRenderingContext2d) {
    let path = Path2d::new().expect("Path");
    path.move_to(20.0, 20.0);
    path.line_to(20.0, 30.0);
    path.rect(10.0, 10.0, 100.0, 50.0);

    context.fill_with_path_2d(&path);
}

pub fn draw_turtle_trails(trails: &Vec<(Style, Draw<Path2d>)>, context: &CanvasRenderingContext2d) {
    for (style, draw) in trails {
        context.set_stroke_style(&style.color.clone().into());
        context.set_line_width(style.width);
        // context.rect(10.0, 10.0, 100.0, 100.0);
        // context.stroke();
        context.stroke_with_path(&draw.path);
    }
    // console::log_1(&format!("Trails {}", trails.len()).into());
}

pub fn draw_turtle_head(turtle: &Turtle, context: &CanvasRenderingContext2d) {
    context.begin_path();
    context
        .ellipse(
            turtle.position.0,
            turtle.position.1,
            3.0,
            3.0,
            0.0,
            0.0,
            Angle::turn().value(),
        )
        .unwrap();
    context.set_fill_style(&"black".into());
    context.fill();
    context.begin_path();
    context.set_stroke_style(&"black".into());
    context.move_to(turtle.position.0, turtle.position.1);
    context.line_to(
        turtle.position.0 + turtle.head.cos_r(6.0),
        turtle.position.1 + turtle.head.sin_r(6.0),
    );
    context.stroke();
}
