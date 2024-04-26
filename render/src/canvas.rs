use wasm_bindgen::prelude::{JsCast, JsValue};
use web_sys::{console, CanvasRenderingContext2d, Path2d};

use turtle::turtle::{PathIns, Trail, TurtleState};
use turtle::units::Angle;

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

pub fn draw_turtle_trails(turtle: &TurtleState, context: &CanvasRenderingContext2d) {
    for Trail { style, path } in turtle.iter() {
        context.set_stroke_style(&style.color.clone().into());
        context.set_line_width(style.width);
        let path2d = Path2d::new().expect("Path");
        for ins in path.iter() {
            console::log_1(&format!("{:?}", ins).into());
            match ins {
                PathIns::MoveTo { x, y } => {
                    path2d.move_to(*x, *y);
                }
                PathIns::LineTo { x, y } => {
                    path2d.line_to(*x, *y);
                }
                PathIns::Arc {
                    center_x,
                    center_y,
                    radius,
                    start_angle,
                    end_angle,
                } => {
                    path2d
                        .arc(*center_x, *center_y, *radius, *start_angle, *end_angle)
                        .unwrap();
                }
            }
        }
        context.stroke_with_path(&path2d);
    }
}

pub fn draw_turtle_head(turtle: &TurtleState, context: &CanvasRenderingContext2d) {
    let angle = turtle.heading();
    let pos = turtle.position();
    context.begin_path();
    context
        .ellipse(pos.0, pos.1, 3.0, 3.0, 0.0, 0.0, Angle::turn().value())
        .unwrap();
    context.set_fill_style(&"black".into());
    context.fill();
    context.begin_path();
    context.set_stroke_style(&"black".into());
    context.move_to(pos.0, pos.1);
    context.line_to(pos.0 + angle.cos_r(6.0), pos.1 + angle.sin_r(6.0));
    context.stroke();
}
