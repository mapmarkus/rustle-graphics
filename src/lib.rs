use std::f64::consts::PI;

use wasm_bindgen::prelude::*;
use web_sys::Path2d;

mod canvas;
mod draw;
pub mod turtle;
mod units;

use canvas::{draw_turtle_head, draw_turtle_trails, get_context};
use draw::Draw;
use turtle::*;
use units::Angle;

// WASM STUFF

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let context = get_context()?;

    let mut turtle = Turtle {
        head: Angle::new(0.0),
        position: (250.0, 150.0),
    };

    let red = Style {
        color: "red".to_string(),
        width: 2.0,
    };
    let blue = Style {
        color: "blue".to_string(),
        width: 2.0,
    };

    draw_turtle_head(&turtle, &context);

    let _steps1 = vec![
        Step::PenDown(red.clone()),
        // Step::Move(10.0),
        Step::Pivot {
            distance: 50.0,
            arc: Angle::new(PI / 3.0),
        },
        Step::PenDown(blue.clone()),
        // Step::Move(10.0),
        Step::Pivot {
            distance: 50.0,
            arc: Angle::new(PI / 3.0),
        },
        Step::Turn(Angle::quarter_turn()),
        Step::Go(50.0),
        Step::PenUp,
        Step::Go(-50.0),
        Step::Turn(Angle::quarter_turn().negate()),
    ];

    let _steps2 = vec![
        Step::PenDown(Style {
            color: "red".to_string(),
            width: 1.0,
        }),
        Step::Turn(Angle::new(2.0 * PI / 3.0)),
        Step::Go(100.0),
        Step::PenDown(Style {
            color: "blue".to_string(),
            width: 1.0,
        }),
        Step::Turn(Angle::new(2.0 * PI / 3.0)),
        Step::Go(100.0),
        Step::Pivot {
            distance: 50.0,
            arc: Angle::quarter_turn(),
        },
        Step::PenUp,
        Step::Go(100.0),
        Step::PenDown(Style {
            color: "green".to_string(),
            width: 1.0,
        }),
        Step::Repeat {
            count: 5,
            steps: [Step::Turn(Angle::new(PI / 6.0)), Step::Go(100.0)].to_vec(),
        },
        Step::PenUp,
        Step::PenDown(Style {
            color: "yellow".to_string(),
            width: 1.0,
        }),
        Step::Turn(Angle::new(2.0 * PI / 3.0)),
        Step::Go(100.0),
    ];

    let _steps3 = vec![
        Step::PenDown(Style {
            color: "red".to_string(),
            width: 1.0,
        }),
        Step::Repeat {
            count: 4,
            steps: vec![
                Step::Pivot {
                    distance: 20.0,
                    arc: Angle::new(PI / 3.0),
                },
                Step::Pivot {
                    distance: 20.0,
                    arc: Angle::new(-PI / 3.0),
                },
                Step::Pivot {
                    distance: 20.0,
                    arc: Angle::new(PI / 2.0),
                },
                Step::Pivot {
                    distance: 20.0,
                    arc: Angle::new(-PI),
                },
            ],
        },
    ];

    let _flower = vec![
        Step::PenDown(red.clone()),
        Step::Repeat {
            count: 12,
            steps: vec![
                Step::Go(20.0),
                Step::Turn(Angle::new(PI / 3.0)),
                Step::Repeat {
                    count: 9,
                    steps: vec![Step::Go(20.0), Step::Turn(Angle::new(PI / 6.0))],
                },
            ],
        },
    ];

    let trails: Vec<(Style, Draw<Path2d>)> = draw_turtle(&mut turtle, &_steps3);

    draw_turtle_trails(&trails, &context);
    draw_turtle_head(&turtle, &context);

    Ok(())
}
